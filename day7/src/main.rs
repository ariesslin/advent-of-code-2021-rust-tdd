use common::numbers_from_first_line_in_file;
use std::{error::Error, path::Path};

fn main() {
    let filename = "day7_input.txt";
    let positions = read_crab_horizontal_positions_from_file_by_line(filename).unwrap();

    let least_fuel_cost = get_least_fuel_cost(positions.clone(), false);
    println!("the least fuel cost is {:?}", least_fuel_cost);

    let least_fuel_cost = get_least_fuel_cost(positions, true);
    println!("the least fuel cost is {:?}", least_fuel_cost);
}

fn read_crab_horizontal_positions_from_file_by_line(
    filename: impl AsRef<Path>,
) -> Result<Vec<i64>, Box<dyn Error>> {
    numbers_from_first_line_in_file(filename)
}

fn fuel_cost_by_target_position(horizontal_positions: Vec<i64>, target_position: i64) -> i64 {
    horizontal_positions
        .iter()
        .map(|p| (p - target_position).abs())
        .sum()
}

fn expensive_fuel_cost_by_target_position(
    horizontal_positions: Vec<i64>,
    target_position: i64,
) -> i64 {
    horizontal_positions
        .iter()
        .map(|p| (1..(p - target_position).abs() + 1).sum::<i64>())
        .sum()
}

fn get_least_fuel_cost(horizontal_positions: Vec<i64>, expensive: bool) -> i64 {
    let length = horizontal_positions.len();
    let mut fuel_costs = vec![0; length];
    for (p, item) in fuel_costs.iter_mut().enumerate().take(length) {
        *item = match expensive {
            true => expensive_fuel_cost_by_target_position(horizontal_positions.clone(), p as i64),
            false => fuel_cost_by_target_position(horizontal_positions.clone(), p as i64),
        };
    }

    *(fuel_costs.iter().min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_all_crab_horizontal_positions_from_file_by_first_line_given_the_filename() {
        let filename = "day7_test.txt";
        let positions = read_crab_horizontal_positions_from_file_by_line(filename).unwrap();

        assert_eq!(positions.len(), 10);
    }

    #[test]
    fn should_get_least_fuel_cost_given_all_crab_horizontal_positions() {
        let filename = "day7_test.txt";
        let positions = read_crab_horizontal_positions_from_file_by_line(filename).unwrap();

        let fule_cost = fuel_cost_by_target_position(positions.clone(), 2);
        assert_eq!(fule_cost, 37);

        let fule_cost = fuel_cost_by_target_position(positions.clone(), 1);
        assert_eq!(fule_cost, 41);

        let fule_cost = fuel_cost_by_target_position(positions.clone(), 3);
        assert_eq!(fule_cost, 39);

        let fule_cost = fuel_cost_by_target_position(positions.clone(), 10);
        assert_eq!(fule_cost, 71);

        let least_fuel_cost = get_least_fuel_cost(positions, false);
        assert_eq!(least_fuel_cost, 37);
    }

    #[test]
    fn should_get_least_fuel_cost_given_all_crab_horizontal_positions_in_expensive_fuel_consuming_mode(
    ) {
        let filename = "day7_test.txt";
        let positions = read_crab_horizontal_positions_from_file_by_line(filename).unwrap();

        let fule_cost = expensive_fuel_cost_by_target_position(positions.clone(), 2);
        assert_eq!(fule_cost, 206);

        let fule_cost = expensive_fuel_cost_by_target_position(positions.clone(), 5);
        assert_eq!(fule_cost, 168);

        let least_fuel_cost = get_least_fuel_cost(positions, true);
        assert_eq!(least_fuel_cost, 168);
    }
}
