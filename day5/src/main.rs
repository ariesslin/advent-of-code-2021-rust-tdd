use common::lines_from_file;
use std::{error::Error, path::Path};

//TIP: 经验表明用tuple表示x，y不直观，还是要多用一个struct，把它们表示为x，y
#[derive(Clone, Debug)]
struct VentReading {
    start: (i64, i64),
    end: (i64, i64),
}

#[derive(Clone, Debug)]
struct VentMap {
    val_mapping: Vec<Vec<i64>>,
    x_max: i64,
    y_max: i64,
}

fn read_vents_from_file_by_line(
    filename: impl AsRef<Path>,
) -> Result<(Vec<VentReading>, VentMap), Box<dyn Error>> {
    let lines_from_file = lines_from_file(filename)?;
    let mut readings = vec![];
    let mut x_max = 0;
    let mut y_max = 0;
    for line in lines_from_file {
        // println!("{:?}", line);
        let mut split = line.trim().split("->");
        let mut split_start = split.next().unwrap().trim().split(',');
        let x_start = split_start.next().unwrap().parse::<i64>()?;
        let y_start = split_start.next().unwrap().parse::<i64>()?;
        if x_start > x_max {
            x_max = x_start;
        }
        if y_start > y_max {
            y_max = y_start;
        }
        let mut split_end = split.next().unwrap().trim().split(',');
        let x_end = split_end.next().unwrap().parse::<i64>()?;
        let y_end = split_end.next().unwrap().parse::<i64>()?;
        if x_end > x_max {
            x_max = x_end;
        }
        if y_end > y_max {
            y_max = y_end;
        }

        let reading = VentReading {
            start: (x_start, y_start),
            end: (x_end, y_end),
        };
        readings.push(reading);
    }
    let vent_map = VentMap {
        val_mapping: vec![vec![0; (x_max + 1) as usize]; (y_max + 1) as usize],
        x_max: x_max + 1,
        y_max: y_max + 1,
    };
    Ok((readings, vent_map))
}

fn get_number_of_overlapping_points_from_vents(
    vent_readings: Vec<VentReading>,
    mut vent_map: VentMap,
    check_diagonal: bool,
) -> i64 {
    for vent_reading in vent_readings {
        if vent_reading.start.0 == vent_reading.end.0 {
            let min = if vent_reading.start.1 <= vent_reading.end.1 {
                vent_reading.start.1
            } else {
                vent_reading.end.1
            } as usize;
            let max = if vent_reading.start.1 > vent_reading.end.1 {
                vent_reading.start.1
            } else {
                vent_reading.end.1
            } as usize;
            /*println!(
                "the x is {}, y min is {}, y max is {}",
                vent_reading.start.0, min, max
            );*/
            for y in min..max + 1 {
                vent_map.val_mapping[vent_reading.start.0 as usize][y] += 1;
            }
        } else if vent_reading.start.1 == vent_reading.end.1 {
            let min = if vent_reading.start.0 <= vent_reading.end.0 {
                vent_reading.start.0
            } else {
                vent_reading.end.0
            } as usize;
            let max = if vent_reading.start.0 > vent_reading.end.0 {
                vent_reading.start.0
            } else {
                vent_reading.end.0
            } as usize;
            /*println!(
                "the x min is {}, x max is {}, y is {}",
                min, max, vent_reading.start.1
            );*/
            for x in min..max + 1 {
                vent_map.val_mapping[x][vent_reading.start.1 as usize] += 1;
            }
        } else if check_diagonal
            && (vent_reading.start.0 - vent_reading.end.0).abs()
                == (vent_reading.start.1 - vent_reading.end.1).abs()
        {
            /*println!(
                "diagonal start x is {}, y is {}, end x is {}, y is {}",
                vent_reading.start.0, vent_reading.start.1, vent_reading.end.0, vent_reading.end.1
            );*/
            let steps = (vent_reading.start.0 - vent_reading.end.0).abs();
            //println!("lengh is {}", steps);
            let mut x = vent_reading.start.0;
            let mut y = vent_reading.start.1;
            for _ in 0..steps + 1 {
                //println!("diagonal x is {}, y is {}", x, y);
                vent_map.val_mapping[x as usize][y as usize] += 1;
                x += (vent_reading.end.0 - vent_reading.start.0) / steps;
                y += (vent_reading.end.1 - vent_reading.start.1) / steps;
            }
        }
    }

    let mut count = 0;
    for x in 0..vent_map.x_max {
        for y in 0..vent_map.y_max {
            if vent_map.val_mapping[x as usize][y as usize] >= 2 {
                count += 1;
            }
        }
    }
    //println!("the vent map is {:?}", vent_map.val_mapping);

    count
}

fn main() {
    let filename = "day5_input.txt";
    let (readings, vent_map) = read_vents_from_file_by_line(filename).unwrap();

    let overlapping_points =
        get_number_of_overlapping_points_from_vents(readings.clone(), vent_map.clone(), false);

    println!("the overlapping points have {}", overlapping_points);

    let overlapping_points = get_number_of_overlapping_points_from_vents(readings, vent_map, true);

    println!(
        "the overlapping points with diagonal cases have {}",
        overlapping_points
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_all_actions_from_file_by_line_given_the_filename() {
        let filename = "day5_test.txt";
        let (readings, vent_map) = read_vents_from_file_by_line(filename).unwrap();

        assert_eq!(readings.len(), 10);
        assert_eq!(vent_map.x_max, 10);
        assert_eq!(vent_map.y_max, 10);
    }

    #[test]
    fn should_get_right_number_of_overlapping_points_without_diagonal_cases_given_the_vents_input_and_vent_map(
    ) {
        let filename = "day5_test.txt";
        let (readings, vent_map) = read_vents_from_file_by_line(filename).unwrap();

        let overlapping_points =
            get_number_of_overlapping_points_from_vents(readings, vent_map, false);

        assert_eq!(overlapping_points, 5);
    }

    #[test]
    fn should_get_right_number_of_overlapping_points_with_diagonal_cases_given_the_vents_input_and_vent_map(
    ) {
        let filename = "day5_test.txt";
        let (readings, vent_map) = read_vents_from_file_by_line(filename).unwrap();

        let overlapping_points =
            get_number_of_overlapping_points_from_vents(readings, vent_map, true);

        assert_eq!(overlapping_points, 12);
    }
}
