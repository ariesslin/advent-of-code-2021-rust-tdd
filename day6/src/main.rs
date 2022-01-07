use common::numbers_from_first_line_in_file;
use std::{error::Error, path::Path};

fn main() {
    let filename = "day6_input.txt";
    let fishes = read_fishes_from_file_by_line(filename).unwrap();

    let fish_count = get_fish_count_after_spawning(fishes.clone(), 80);
    println!("the fishes number is {:?}", fish_count);

    let fish_count = get_fish_count_after_spawning(fishes, 256);
    println!("the fishes number is {:?}", fish_count);
}

fn read_fishes_from_file_by_line(filename: impl AsRef<Path>) -> Result<Vec<i64>, Box<dyn Error>> {
    numbers_from_first_line_in_file(filename)
}

fn get_fish_count_after_spawning(fishes: Vec<i64>, spawn_days: i64) -> i64 {
    let mut fish_day_count = vec![0; 9];
    for fish in fishes.iter() {
        fish_day_count[*fish as usize] += 1;
    }

    for _ in 0..spawn_days {
        let mut tmp_pre = fish_day_count[8];
        let mut tmp_now = 0;
        for c in (0..8).rev() {
            tmp_now = fish_day_count[c];
            fish_day_count[c] = tmp_pre;
            tmp_pre = tmp_now;
        }
        fish_day_count[8] = tmp_now;
        fish_day_count[6] += tmp_now;

        //println!("fish day count vector is {:?}", fish_day_count);
    }

    fish_day_count.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_all_initial_fishes_from_file_by_first_line_given_the_filename() {
        let filename = "day6_test.txt";
        let fishes = read_fishes_from_file_by_line(filename).unwrap();

        assert_eq!(fishes.len(), 5);
    }

    #[test]
    fn should_get_right_number_of_fishes_from_given_days_of_spawning() {
        let filename = "day6_test.txt";
        let fishes = read_fishes_from_file_by_line(filename).unwrap();

        let fish_count = get_fish_count_after_spawning(fishes.clone(), 18);

        assert_eq!(fish_count, 26);

        let fish_count = get_fish_count_after_spawning(fishes, 80);

        assert_eq!(fish_count, 5934);
    }
}
