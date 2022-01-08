use common::parse_numbers_without_split_sign_from_lines_in_file;
use std::{error::Error, path::Path};

fn read_heightmap_from_file(filename: impl AsRef<Path>) -> Result<Vec<Vec<i64>>, Box<dyn Error>> {
    parse_numbers_without_split_sign_from_lines_in_file(filename)
}

fn get_low_points_from_heightmap(heightmaps: Vec<Vec<i64>>) -> Vec<i64> {
    let height = heightmaps.len() + 2;
    let width = heightmaps[0].len() + 2;

    let mut extended_heightmaps = vec![vec![10; width]; height];
    for (index, extended_row) in extended_heightmaps[1..height - 1].iter_mut().enumerate() {
        extended_row.splice(1..width - 1, heightmaps[index].iter().cloned());
    }

    //println!("the extended heightmaps is {:#?}", extended_heightmaps);

    let mut low_points = vec![];
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if extended_heightmaps[y][x] < extended_heightmaps[y - 1][x]
                && extended_heightmaps[y][x] < extended_heightmaps[y + 1][x]
                && extended_heightmaps[y][x] < extended_heightmaps[y][x - 1]
                && extended_heightmaps[y][x] < extended_heightmaps[y][x + 1]
            {
                low_points.push(extended_heightmaps[y][x]);
            }
        }
    }

    low_points
}

fn main() {
    let filename = "day9_input.txt";
    let heightmap = read_heightmap_from_file(filename).unwrap();

    let low_points = get_low_points_from_heightmap(heightmap);
    let sum_risk_level = low_points.iter().map(|x| x + 1).sum::<i64>();
    println!("sum_risk_level is {}", sum_risk_level);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_heightmap_from_file_given_the_filename() {
        let filename = "day9_test.txt";
        let heightmap = read_heightmap_from_file(filename).unwrap();

        assert_eq!(heightmap.len(), 5);
        assert_eq!(heightmap[0].len(), 10);
    }

    #[test]
    fn should_get_sum_of_risk_level_of_low_points_given_heightmap() {
        let filename = "day9_test.txt";
        let heightmap = read_heightmap_from_file(filename).unwrap();

        let low_points = get_low_points_from_heightmap(heightmap);
        assert_eq!(low_points.len(), 4);

        let sum_risk_level = low_points.iter().map(|x| x + 1).sum::<i64>();
        assert_eq!(sum_risk_level, 15);
    }
}
