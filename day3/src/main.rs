use common::lines_from_file;
use std::{error::Error, path::Path};

#[derive(Default)]
struct DiagnosticReportBinary {
    bits: Vec<u32>,
}

fn read_power_consumption_binary_from_file_by_line(
    filename: impl AsRef<Path>,
) -> Result<Vec<DiagnosticReportBinary>, Box<dyn Error>> {
    let lines_from_file = lines_from_file(filename)?;
    let mut readings = vec![];
    for line in lines_from_file {
        //println!("{:?}", line);
        let chars = line.trim().chars();
        let mut bits = vec![];
        for c in chars {
            let bit = c.to_digit(10).unwrap();
            bits.push(bit);
        }
        let reading = DiagnosticReportBinary { bits };
        readings.push(reading);
    }
    Ok(readings)
}

fn get_power_consumption(readings: Vec<DiagnosticReportBinary>) -> (u32, u32) {
    let length = readings[0].bits.len();
    let mut bit_sums = vec![0; length];
    let major_threshold = (readings.len() / 2) as u32;

    for reading in readings {
        for (index, bit) in reading.bits.iter().enumerate() {
            bit_sums[index] += *bit;
        }
    }

    let mut final_bit_sums = vec![];
    for bit_sum in bit_sums.iter().rev() {
        let bit_sum = if *bit_sum > major_threshold { 1 } else { 0 };
        final_bit_sums.push(bit_sum);
    }

    let mut i = 1;
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for bit_sum in final_bit_sums {
        gamma_rate += bit_sum * i;
        epsilon_rate += (1 - bit_sum) * i;
        i *= 2;
    }

    (gamma_rate, epsilon_rate)
}

fn main() {
    let readings = read_power_consumption_binary_from_file_by_line("day3_input.txt")
        .expect("Could not load lines");
    println!("total {} lines", readings.len());

    let (gamma_rate, epsilon_rate) = get_power_consumption(readings);
    println!("the power consumption is {}", gamma_rate * epsilon_rate);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // TODO: is this an integration test?
    fn should_get_all_actions_from_file_by_line_given_the_filename() {
        let filename = "day3_test.txt";
        let readings = read_power_consumption_binary_from_file_by_line(filename).unwrap();

        assert_eq!(readings.len(), 12);
    }

    #[test]
    fn should_get_right_power_consumption_given_the_diagnostic_report_in_binary() {
        let filename = "day3_test.txt";
        let readings = read_power_consumption_binary_from_file_by_line(filename).unwrap();

        let (gamma_rate, epsilon_rate) = get_power_consumption(readings);
        assert_eq!(gamma_rate * epsilon_rate, 198);
    }
}
