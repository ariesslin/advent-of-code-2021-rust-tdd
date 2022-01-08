use common::lines_from_file;
use std::{collections::HashSet, error::Error, path::Path};

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

fn read_input_values_from_file_by_line(
    filename: impl AsRef<Path>,
) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let lines_from_file = lines_from_file(filename)?;
    let mut input_values = vec![];

    for line in lines_from_file {
        //println!("{:?}", line);
        let mut split = line.trim().split('|');
        let output_value = split
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        input_values.push(output_value);
    }

    Ok(input_values)
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

fn generate_decode_input_set(input_line_values: Vec<String>) -> Vec<HashSet<String>> {
    let mut decode_set = vec![HashSet::new(); 10];
    let mut input_set = vec![];
    for input_value in input_line_values {
        let set: HashSet<String> = input_value
            .chars()
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        input_set.push(set);
    }

    for (_, input_value) in input_set.iter().enumerate() {
        match input_value.len() {
            2 => decode_set[1] = input_value.clone(),
            3 => decode_set[7] = input_value.clone(),
            4 => decode_set[4] = input_value.clone(),
            7 => decode_set[8] = input_value.clone(),
            _ => continue,
        }
    }

    for (index, input_value) in input_set.iter().enumerate() {
        if input_value.len() == 5 && input_value.is_superset(&decode_set[7]) {
            decode_set[3] = input_value.clone();
            input_set.remove(index);
            break;
        }
    }

    for (index, input_value) in input_set.iter().enumerate() {
        if input_value.len() == 6 && input_value.is_superset(&decode_set[3]) {
            decode_set[9] = input_value.clone();
            input_set.remove(index);
            break;
        }
    }

    for (_, input_value) in input_set.iter().enumerate() {
        if input_value.len() == 6 {
            if input_value.is_superset(&decode_set[7]) {
                decode_set[0] = input_value.clone();
            } else {
                decode_set[6] = input_value.clone();
            }
        }

        if input_value.len() == 5 {
            if input_value.is_subset(&decode_set[9]) {
                decode_set[5] = input_value.clone();
            } else {
                decode_set[2] = input_value.clone();
            }
        }
    }

    decode_set
}

fn get_decode_numbers(input_values: Vec<Vec<String>>, output_values: Vec<Vec<String>>) -> Vec<i64> {
    let mut decode_numbers = vec![];
    for (index, output_line_values) in output_values.iter().enumerate() {
        let mut output_predecode = vec![];
        for output_value in output_line_values {
            let set: HashSet<String> = output_value
                .chars()
                .into_iter()
                .map(|x| x.to_string())
                .collect();
            output_predecode.push(set);
        }

        //println!("current output set is {:?}", output_predecode);

        let decode_input_set = generate_decode_input_set(input_values[index].clone());

        //println!("current decode input set is {:?}", decode_input_set);

        let mut n = 1;
        let mut decode_output_number = 0;
        for output_number in output_predecode.iter().rev() {
            //println!("\ncurrent output number is {:?}", output_number);
            for (number, decode_input_number) in decode_input_set.iter().enumerate() {
                if output_number == decode_input_number {
                    //println!("matched input number is {:?}\n", number);
                    decode_output_number += number as i64 * n;
                    n *= 10;
                }
            }
        }
        decode_numbers.push(decode_output_number);
    }
    decode_numbers
}

fn main() {
    let filename = "day8_input.txt";
    let output_values = read_output_values_from_file_by_line(filename).unwrap();

    let unique_number_count = get_unique_number_count(output_values.clone());
    println!("the unique number count is {}", unique_number_count);

    let input_values = read_input_values_from_file_by_line(filename).unwrap();
    let decode_numbers = get_decode_numbers(input_values, output_values);

    println!(
        "the sum of output values is {}",
        decode_numbers.iter().sum::<i64>()
    );
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
    fn should_get_all_input_values_from_file_by_line_given_the_filename() {
        let filename = "day8_test.txt";
        let input_values = read_input_values_from_file_by_line(filename).unwrap();

        assert_eq!(input_values.len(), 10);

        for input_value in input_values {
            assert_eq!(input_value.len(), 10);
        }
    }

    #[test]
    fn should_get_unique_number_count_given_output_values() {
        let filename = "day8_test.txt";
        let output_values = read_output_values_from_file_by_line(filename).unwrap();

        let unique_number_count = get_unique_number_count(output_values);

        assert_eq!(unique_number_count, 26);
    }

    #[test]
    fn should_decode_right_numbers_given_input_values_and_output_values() {
        let filename = "day8_test.txt";
        let input_values = read_input_values_from_file_by_line(filename).unwrap();
        let output_values = read_output_values_from_file_by_line(filename).unwrap();

        let decode_numbers = get_decode_numbers(input_values, output_values);

        assert_eq!(decode_numbers[0], 8394);
        assert_eq!(decode_numbers[1], 9781);
        assert_eq!(decode_numbers[2], 1197);
        assert_eq!(decode_numbers[3], 9361);
        assert_eq!(decode_numbers[4], 4873);
        assert_eq!(decode_numbers[5], 8418);
        assert_eq!(decode_numbers[6], 4548);
        assert_eq!(decode_numbers[7], 1625);
        assert_eq!(decode_numbers[8], 8717);
        assert_eq!(decode_numbers[9], 4315);

        assert_eq!(decode_numbers.iter().sum::<i64>(), 61229);
    }
}
