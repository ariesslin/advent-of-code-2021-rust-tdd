use common::parse_numbers_without_split_sign_from_lines_in_file;
use std::{collections::HashSet, error::Error, path::Path};

fn read_octopuses_metrix_from_file(
    filename: impl AsRef<Path>,
) -> Result<Vec<Vec<i64>>, Box<dyn Error>> {
    parse_numbers_without_split_sign_from_lines_in_file(filename)
}

fn is_valid(metrix: &[Vec<i64>], x: i64, y: i64, flashed: &HashSet<(i64, i64)>) -> bool {
    x >= 0
        && y >= 0
        && x < metrix.len() as i64
        && y < metrix.len() as i64
        && !flashed.contains(&(x, y))
}

fn added_one_to_metrix(mut octopuses: Vec<Vec<i64>>) -> (Vec<Vec<i64>>, i64) {
    let rows = octopuses.len();
    let columns = octopuses[0].len();
    let mut flashed = HashSet::new();
    for row in octopuses.iter_mut().take(rows) {
        for octopus in row.iter_mut().take(columns) {
            *octopus += 1;
        }
    }

    for x in 0..rows {
        for y in 0..columns {
            if octopuses[x][y] > 9 {
                //println!("x is {}, y is {}", x, y);
                let mut check_list = vec![];
                check_list.push((x, y));
                while !check_list.is_empty() {
                    let x = check_list[0].0;
                    let y = check_list[0].1;
                    if octopuses[x][y] > 9 {
                        octopuses[x][y] = 0;
                        flashed.insert((x as i64, y as i64));

                        if is_valid(&octopuses, x as i64 - 1, y as i64, &flashed) {
                            check_list.push((x - 1, y));
                            octopuses[x - 1][y] += 1;
                        }
                        if is_valid(&octopuses, x as i64 + 1, y as i64, &flashed) {
                            check_list.push((x + 1, y));
                            octopuses[x + 1][y] += 1;
                        }
                        if is_valid(&octopuses, x as i64, y as i64 - 1, &flashed) {
                            check_list.push((x, y - 1));
                            octopuses[x][y - 1] += 1;
                        }
                        if is_valid(&octopuses, x as i64, y as i64 + 1, &flashed) {
                            check_list.push((x, y + 1));
                            octopuses[x][y + 1] += 1;
                        }
                        if is_valid(&octopuses, x as i64 - 1, y as i64 - 1, &flashed) {
                            check_list.push((x - 1, y - 1));
                            octopuses[x - 1][y - 1] += 1;
                        }
                        if is_valid(&octopuses, x as i64 - 1, y as i64 + 1, &flashed) {
                            check_list.push((x - 1, y + 1));
                            octopuses[x - 1][y + 1] += 1;
                        }
                        if is_valid(&octopuses, x as i64 + 1, y as i64 - 1, &flashed) {
                            check_list.push((x + 1, y - 1));
                            octopuses[x + 1][y - 1] += 1;
                        }
                        if is_valid(&octopuses, x as i64 + 1, y as i64 + 1, &flashed) {
                            check_list.push((x + 1, y + 1));
                            octopuses[x + 1][y + 1] += 1;
                        }
                    }
                    check_list.drain(0..1);
                }
            }
        }
    }
    //println!("the current octopuses is {:?}\n", octopuses);
    (octopuses, flashed.len() as i64)
}

fn get_octopuses_snapshot(
    initial_octopuses: Vec<Vec<i64>>,
    step: i64,
) -> (Vec<Vec<i64>>, Vec<i64>) {
    let mut flashed_list = vec![];
    let mut octopuses = initial_octopuses;
    for _ in 0..step {
        let (o, flashed) = added_one_to_metrix(octopuses);
        octopuses = o;
        flashed_list.push(flashed);
    }
    (octopuses, flashed_list)
}

fn main() {
    let filename = "day11_input.txt";
    let octopuses = read_octopuses_metrix_from_file(filename).unwrap();

    println!(
        "total flashes after 100 steps are {}",
        get_octopuses_snapshot(octopuses, 100).1.iter().sum::<i64>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_octopuses_metrix_from_file_given_the_filename() {
        let filename = "day11_test.txt";
        let octopuses = read_octopuses_metrix_from_file(filename).unwrap();

        assert_eq!(octopuses.len(), 10);
        assert_eq!(octopuses[0].len(), 10);
    }

    #[test]
    fn should_get_snapshot_of_octopuses_metrix_given_the_initial_metrix_and_step_number() {
        let filename = "day11_test.txt";
        let octopuses = read_octopuses_metrix_from_file(filename).unwrap();

        let snapshot5 = vec![
            vec![4, 4, 8, 4, 1, 4, 4, 0, 0, 0],
            vec![2, 0, 4, 4, 1, 4, 4, 0, 0, 0],
            vec![2, 2, 5, 3, 3, 3, 3, 4, 9, 3],
            vec![1, 1, 5, 2, 3, 3, 3, 2, 7, 4],
            vec![1, 1, 8, 7, 3, 0, 3, 2, 8, 5],
            vec![1, 1, 6, 4, 6, 3, 3, 2, 3, 3],
            vec![1, 1, 5, 3, 4, 7, 2, 2, 3, 1],
            vec![6, 6, 4, 3, 3, 5, 2, 2, 3, 3],
            vec![2, 6, 4, 3, 3, 5, 8, 3, 2, 2],
            vec![2, 2, 4, 3, 3, 4, 1, 3, 2, 2],
        ];
        assert_eq!(get_octopuses_snapshot(octopuses.clone(), 5).0, snapshot5);

        assert_eq!(
            get_octopuses_snapshot(octopuses.clone(), 10)
                .1
                .iter()
                .sum::<i64>(),
            204
        );

        assert_eq!(
            get_octopuses_snapshot(octopuses, 100).1.iter().sum::<i64>(),
            1656
        );
    }
}
