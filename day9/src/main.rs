use common::parse_numbers_without_split_sign_from_lines_in_file;
use std::{collections::HashSet, error::Error, path::Path};

struct LowPoint {
    x: i64,
    y: i64,
    value: i64,
}

struct ExtendedHeightMap {
    height: i64,
    width: i64,
    heightmap: Vec<Vec<i64>>,
}

fn read_heightmap_from_file(filename: impl AsRef<Path>) -> Result<Vec<Vec<i64>>, Box<dyn Error>> {
    parse_numbers_without_split_sign_from_lines_in_file(filename)
}

fn get_extended_heightmap(heightmap: &[Vec<i64>]) -> ExtendedHeightMap {
    let height = heightmap.len() + 2;
    let width = heightmap[0].len() + 2;

    let mut extended_heightmaps = vec![vec![10; width]; height];
    for (index, extended_row) in extended_heightmaps[1..height - 1].iter_mut().enumerate() {
        extended_row.splice(1..width - 1, heightmap[index].iter().cloned());
    }

    ExtendedHeightMap {
        height: height as i64,
        width: width as i64,
        heightmap: extended_heightmaps,
    }
}

fn get_low_points_from_heightmap(heightmap: Vec<Vec<i64>>) -> Vec<LowPoint> {
    let eh = get_extended_heightmap(&heightmap);
    let extended_heightmap = eh.heightmap;

    //println!("the extended heightmaps is {:#?}", extended_heightmap);

    let mut low_points = vec![];
    for x in 1..(eh.height as usize - 1) {
        for y in 1..(eh.width as usize - 1) {
            if extended_heightmap[x][y] < extended_heightmap[x][y - 1]
                && extended_heightmap[x][y] < extended_heightmap[x][y + 1]
                && extended_heightmap[x][y] < extended_heightmap[x - 1][y]
                && extended_heightmap[x][y] < extended_heightmap[x + 1][y]
            {
                let low_point = LowPoint {
                    x: x as i64,
                    y: y as i64,
                    value: extended_heightmap[x][y],
                };
                low_points.push(low_point);
            }
        }
    }

    low_points
}

fn is_valid(extended_heightmap: &[Vec<i64>], x: i64, y: i64, basin: &HashSet<(i64, i64)>) -> bool {
    extended_heightmap[x as usize][y as usize] < 9 && !basin.contains(&(x, y))
}

fn get_basin_sizes_from_heightmap(heightmap: Vec<Vec<i64>>) -> Vec<i64> {
    let eh = get_extended_heightmap(&heightmap);
    let extended_heightmap = eh.heightmap;

    let low_points = get_low_points_from_heightmap(heightmap);
    let mut basin_sizes = vec![];

    for low_point in low_points {
        let mut check_list = vec![(low_point.x, low_point.y)];
        let mut basin = HashSet::new();
        basin.insert((low_point.x, low_point.y));
        while !check_list.is_empty() {
            let x = check_list[0].0;
            let y = check_list[0].1;
            if is_valid(&extended_heightmap, x - 1, y, &basin) {
                check_list.push((x - 1, y));
                basin.insert((x - 1, y));
            }
            if is_valid(&extended_heightmap, x + 1, y, &basin) {
                check_list.push((x + 1, y));
                basin.insert((x + 1, y));
            }
            if is_valid(&extended_heightmap, x, y - 1, &basin) {
                check_list.push((x, y - 1));
                basin.insert((x, y - 1));
            }
            if is_valid(&extended_heightmap, x, y + 1, &basin) {
                check_list.push((x, y + 1));
                basin.insert((x, y + 1));
            }
            //println!("check_list is {:?}", check_list);
            check_list.drain(0..1);
        }
        let basin_size = basin.len() as i64;
        //println!("basin size is {:?}", basin_size);
        basin_sizes.push(basin_size);
    }

    basin_sizes
}

fn main() {
    let filename = "day9_input.txt";
    let heightmap = read_heightmap_from_file(filename).unwrap();

    let low_points = get_low_points_from_heightmap(heightmap.clone());
    let sum_risk_level = low_points
        .iter()
        .map(|lower_point| lower_point.value + 1)
        .sum::<i64>();
    println!("sum_risk_level is {}", sum_risk_level);

    let mut basins = get_basin_sizes_from_heightmap(heightmap);
    basins.sort_unstable();
    basins.reverse();

    println!(
        "multiple of largest basin sizes is {}",
        basins[0] * basins[1] * basins[2]
    );
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

        let sum_risk_level = low_points
            .iter()
            .map(|lower_point| lower_point.value + 1)
            .sum::<i64>();
        assert_eq!(sum_risk_level, 15);
    }

    #[test]
    fn should_get_right_multiply_of_3_largest_basin_sizes_given_heightmap() {
        let filename = "day9_test.txt";
        let heightmap = read_heightmap_from_file(filename).unwrap();

        let mut basins = get_basin_sizes_from_heightmap(heightmap);
        assert_eq!(basins.len(), 4);

        basins.sort();
        basins.reverse();
        //println!("sorted basin sizes are {:?}", basins);
        let large_basins = &basins[0..3];
        assert_eq!(large_basins[0], 14);
        assert_eq!(large_basins[1], 9);
        assert_eq!(large_basins[2], 9);

        assert_eq!(large_basins[0] * large_basins[1] * large_basins[2], 1134);
    }
}
