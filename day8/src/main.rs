use common::lines_from_file;
use std::{error::Error, path::Path};

fn read_output_values_from_file_by_line(
    filename: impl AsRef<Path>,
) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let lines_from_file = lines_from_file(filename)?;
    let mut output_values = vec![];

    for line in lines_from_file {
        //println!("{:?}", line);
        let mut split = line.trim().split('|');
        let _ = split.next();
        let output_value = split
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        output_values.push(output_value);
    }

    Ok(output_values)
}

fn get_unique_number_count(output_values: Vec<Vec<String>>) -> i64 {
    let mut count = 0;
    for output_value in output_values {
        count += output_value
            .into_iter()
            .filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
            .count();
    }
    count as i64
}

fn main() {
    let filename = "day8_input.txt";
    let output_values = read_output_values_from_file_by_line(filename).unwrap();

    let unique_number_count = get_unique_number_count(output_values);
    println!("the unique number count is {}", unique_number_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_all_output_values_from_file_by_line_given_the_filename() {
        let filename = "day8_test.txt";
        let output_values = read_output_values_from_file_by_line(filename).unwrap();

        assert_eq!(output_values.len(), 10);

        for output_value in output_values {
            assert_eq!(output_value.len(), 4);
        }
    }

    #[test]
    fn should_get_unique_number_count_given_output_values() {
        let filename = "day8_test.txt";
        let output_values = read_output_values_from_file_by_line(filename).unwrap();

        let unique_number_count = get_unique_number_count(output_values);

        assert_eq!(unique_number_count, 26);
    }
}
