use common::parse_numbers_without_split_sign_from_lines_in_file;
use std::{error::Error, path::Path};

#[derive(Default, Clone, Debug)]
struct DiagnosticReportBinary {
    bits: Vec<i64>,
}

fn read_power_consumption_binary_from_file_by_line(
    filename: impl AsRef<Path>,
) -> Result<Vec<DiagnosticReportBinary>, Box<dyn Error>> {
    let numbers = parse_numbers_without_split_sign_from_lines_in_file(filename)?;
    let mut readings = vec![];
    for number in numbers {
        let reading = DiagnosticReportBinary { bits: number };
        readings.push(reading);
    }
    Ok(readings)
}

fn get_power_consumption(readings: Vec<DiagnosticReportBinary>) -> (u32, u32) {
    let bit_length = readings[0].bits.len();
    let mut bit_sums = vec![0; bit_length];
    let major_threshold = readings.len() as i64;

    for reading in readings {
        for (index, bit) in reading.bits.iter().enumerate() {
            bit_sums[index] += *bit;
        }
    }

    let mut final_bit_sums = vec![];
    for bit_sum in bit_sums.iter().rev() {
        let bit_sum = if *bit_sum * 2 > major_threshold { 1 } else { 0 };
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

fn get_life_support_rating(readings: Vec<DiagnosticReportBinary>) -> (i64, i64) {
    let oxygen_generator_rating = get_final_reading(readings.clone(), true);
    let co2_scrubber_rating = get_final_reading(readings, false);

    let oxygen_generator_rating = calculate_decimal(oxygen_generator_rating);
    let co2_scrubber_rating = calculate_decimal(co2_scrubber_rating);

    (oxygen_generator_rating, co2_scrubber_rating)
}

fn calculate_decimal(binary: DiagnosticReportBinary) -> i64 {
    let mut i = 1;
    let mut decimal = 0;
    for bit_sum in binary.bits.iter().rev() {
        decimal += bit_sum * i;
        i *= 2;
    }
    //println!("decimal is {}", decimal);
    decimal
}

fn get_final_reading(
    mut readings: Vec<DiagnosticReportBinary>,
    is_major: bool,
) -> DiagnosticReportBinary {
    let bit_length = readings[0].bits.len();
    for index in 0..bit_length {
        if readings.len() <= 1 {
            break;
        }

        let mut bit_sum = 0;
        let major_threshold = readings.len() as i64;

        for reading in &readings {
            bit_sum += reading.bits[index];
        }

        let bit_sum = match is_major {
            true => {
                if bit_sum * 2 >= major_threshold {
                    1
                } else {
                    0
                }
            }
            false => {
                if bit_sum * 2 >= major_threshold {
                    0
                } else {
                    1
                }
            }
        };

        readings = readings
            .clone()
            .into_iter()
            .filter(|x| x.bits[index] == bit_sum)
            .collect::<Vec<DiagnosticReportBinary>>();
    }
    //println!("The final reading is {:?}", readings);
    readings[0].clone()
}

fn main() {
    let readings = read_power_consumption_binary_from_file_by_line("day3_input.txt")
        .expect("Could not load lines");
    println!("total {} lines", readings.len());

    let (gamma_rate, epsilon_rate) = get_power_consumption(readings.clone());
    println!("the power consumption is {}", gamma_rate * epsilon_rate);

    let (oxygen_generator_rating, co2_scrubber_rating) = get_life_support_rating(readings);
    println!(
        "the power consumption is {}",
        oxygen_generator_rating * co2_scrubber_rating
    );
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

    #[test]
    fn should_get_right_life_support_rating_given_the_diagnostic_report_in_binary() {
        let filename = "day3_test.txt";
        let readings = read_power_consumption_binary_from_file_by_line(filename).unwrap();

        let (oxygen_generator_rating, co2_scrubber_rating) = get_life_support_rating(readings);
        assert_eq!(oxygen_generator_rating * co2_scrubber_rating, 230);
    }
}
