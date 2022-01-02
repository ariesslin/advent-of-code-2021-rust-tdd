use std::{
    error::Error,
    path::Path,
};
use common::lines_from_file;

const WINDOW_SIZE: usize = 3;

fn read_nums_from_file_by_line(filename: impl AsRef<Path>) -> Result<Vec<i64>, Box<dyn Error>> {
    let lines_from_file = lines_from_file(filename)?;
    let mut nums = vec![];
    for line in lines_from_file {
        //println!("{:?}", line);
        let num = line.trim().parse::<i64>()?;
        nums.push(num);
    }
    Ok(nums)
}

fn get_sonar_measurement_increase_count(nums: Vec<i64>) -> usize {
    let mut count = 0;
    let mut pre_num = nums[0];
    for line in &nums[1..] {
        let cur_num = *line;
        if cur_num > pre_num {
            count += 1;
        }
        pre_num = cur_num;
    }
    count
}

fn get_sonar_measurement_increase_count_by_sliding_window(nums: Vec<i64>) -> usize {
    if nums.len() <= WINDOW_SIZE {
        return 0;
    }

    let mut count = 0;
    let mut pre_num: i64 = nums[0..WINDOW_SIZE].iter().sum();

    for (index, _line) in nums[WINDOW_SIZE..].iter().enumerate() {
        let cur_num = nums[index + 1..index + WINDOW_SIZE + 1].iter().sum();
        if cur_num > pre_num {
            count += 1;
        }
        pre_num = cur_num;
    }
    count
}

pub fn main() {
    let nums = read_nums_from_file_by_line("day1_input.txt").expect("Could not load lines");
    println!("total {} lines", nums.len());

    let count = get_sonar_measurement_increase_count(nums.clone());
    println!("the final count is {}", count);

    let count = get_sonar_measurement_increase_count_by_sliding_window(nums);
    println!("the final count by sliding window is {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // TODO: is this an integration test?
    fn should_get_all_nums_from_file_by_line_given_the_filename() {
        let filename = "day1_test.txt";
        let nums = read_nums_from_file_by_line(filename).unwrap();

        assert_eq!(nums.len(), 10);
    }

    #[test]
    fn should_get_right_increase_count_given_the_sonar_measurement_numbers() {
        let nums = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let count = get_sonar_measurement_increase_count(nums);
        assert_eq!(count, 7);
    }

    #[test]
    fn should_get_right_increase_count_by_sliding_window_given_the_sonar_measurement_numbers() {
        let nums = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let count = get_sonar_measurement_increase_count_by_sliding_window(nums);
        assert_eq!(count, 5);
    }
}
