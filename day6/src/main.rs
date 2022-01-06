use common::lines_from_file;
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
    let lines_from_file = lines_from_file(filename)?;
    let mut fishes = vec![];
    for fish in lines_from_file[0].trim().split(',').collect::<Vec<&str>>() {
        //println!("{:?}", line);
        let fish = fish.trim().parse::<i64>()?;
        fishes.push(fish);
    }
    Ok(fishes)
}

fn get_fish_count_after_spawning(init_fishes: Vec<i64>, spawn_days: i64) -> i64 {
    let mut fishes = init_fishes;
    for _ in 0..spawn_days {
        let mut new_producing_count = 0;
        for fish in fishes.iter_mut() {
            if *fish == 0 {
                *fish = 6;
                new_producing_count += 1;
            } else {
                *fish -= 1;
            }
        }
        let mut new_fishes = vec![8; new_producing_count];
        fishes.append(&mut new_fishes);
        //println!("the fishes are {:?}", fishes);
    }
    fishes.len() as i64
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
