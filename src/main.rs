use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

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

fn main() {
    let nums = read_nums_from_file_by_line("input.txt").expect("Could not load lines");
    println!("total {} lines", nums.len());

    let count = get_sonar_measurement_increase_count(nums);
    println!("the final count is {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // TODO: is this an integration test?
    fn should_get_all_nums_from_file_by_line_given_the_filename() {
        let filename = "test.txt";
        let nums = read_nums_from_file_by_line(filename).unwrap();

        assert_eq!(nums.len(), 10);
    }

    #[test]
    fn should_get_right_increase_count_given_the_sonar_measurement_numbers() {
        let nums = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let count = get_sonar_measurement_increase_count(nums);
        assert_eq!(count, 7);
    }
}
