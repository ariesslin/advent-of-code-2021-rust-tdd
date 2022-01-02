use std::{error::Error, path::Path};
use common::lines_from_file;

#[derive(Clone)]
struct DiveAction {
    direction: DiveDirection,
    length: i64,
}

impl DiveAction {
    pub fn get_action_direction(&self) -> DiveDirection {
        self.direction.clone()
    }

    pub fn get_action_length(&self) -> i64 {
        self.length.clone()
    }
}

#[derive(Clone)]
enum DiveDirection {
    Forward,
    Up,
    Down,
}

fn read_actions_from_file_by_line(
    filename: impl AsRef<Path>,
) -> Result<Vec<DiveAction>, Box<dyn Error>> {
    let lines_from_file = lines_from_file(filename)?;
    let mut actions = vec![];
    for line in lines_from_file {
        //println!("{:?}", line);
        let mut split = line.trim().split_whitespace();
        let action = DiveAction {
            direction: match split.next().unwrap() {
                "forward" => DiveDirection::Forward,
                "up" => DiveDirection::Up,
                "down" => DiveDirection::Down,
                _ => return Err("Unknown direction".into()),
            },
            length: split.next().unwrap().parse::<i64>()?,
        };
        actions.push(action);
    }
    Ok(actions)
}

fn get_final_diving_position(
    (mut horizontal_pos, mut depth_pos): (i64, i64),
    actions: Vec<DiveAction>,
) -> (i64, i64) {
    for action in actions {
        match action.get_action_direction() {
            DiveDirection::Forward => horizontal_pos += action.get_action_length(),
            DiveDirection::Up => depth_pos -= action.get_action_length(),
            DiveDirection::Down => depth_pos += action.get_action_length(),
        }
    }
    (horizontal_pos, depth_pos)
}

fn get_final_diving_position_adjusted_by_aim(
    mut aim: i64,
    (mut horizontal_pos, mut depth_pos): (i64, i64),
    actions: Vec<DiveAction>,
) -> (i64, i64) {
    for action in actions {
        match action.get_action_direction() {
            DiveDirection::Forward => {
                horizontal_pos += action.get_action_length();
                depth_pos += aim * action.get_action_length();
            }
            DiveDirection::Up => aim -= action.get_action_length(),
            DiveDirection::Down => aim += action.get_action_length(),
        }
    }
    (horizontal_pos, depth_pos)
}

fn main() {
    let actions = read_actions_from_file_by_line("day2_input.txt").expect("Could not load lines");
    println!("total {} lines", actions.len());

    let (horizontal_pos, depth_pos) = get_final_diving_position((0, 0), actions.clone());
    println!(
        "the multiply of final positions is {}",
        horizontal_pos * depth_pos
    );

    let (horizontal_pos, depth_pos) = get_final_diving_position_adjusted_by_aim(0, (0, 0), actions);
    println!(
        "the multiply of final positions with aim is {}",
        horizontal_pos * depth_pos
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // TODO: is this an integration test?
    fn should_get_all_actions_from_file_by_line_given_the_filename() {
        let filename = "day2_test.txt";
        let nums = read_actions_from_file_by_line(filename).unwrap();

        assert_eq!(nums.len(), 6);
    }

    #[test]
    fn should_get_right_final_position_given_the_diving_action_consequences() {
        let (horizontal_pos, depth_pos) = (0, 0);

        let actions = vec![
            DiveAction {
                direction: DiveDirection::Forward,
                length: 5,
            },
            DiveAction {
                direction: DiveDirection::Down,
                length: 5,
            },
            DiveAction {
                direction: DiveDirection::Forward,
                length: 8,
            },
            DiveAction {
                direction: DiveDirection::Up,
                length: 3,
            },
            DiveAction {
                direction: DiveDirection::Down,
                length: 8,
            },
            DiveAction {
                direction: DiveDirection::Forward,
                length: 2,
            },
        ];
        let (horizontal_pos, depth_pos) =
            get_final_diving_position((horizontal_pos, depth_pos), actions);
        assert_eq!(horizontal_pos * depth_pos, 150);
    }

    #[test]
    fn should_get_right_final_position_given_the_diving_action_consequences_and_aim() {
        let (horizontal_pos, depth_pos) = (0, 0);
        let aim = 0;

        let actions = vec![
            DiveAction {
                direction: DiveDirection::Forward,
                length: 5,
            },
            DiveAction {
                direction: DiveDirection::Down,
                length: 5,
            },
            DiveAction {
                direction: DiveDirection::Forward,
                length: 8,
            },
            DiveAction {
                direction: DiveDirection::Up,
                length: 3,
            },
            DiveAction {
                direction: DiveDirection::Down,
                length: 8,
            },
            DiveAction {
                direction: DiveDirection::Forward,
                length: 2,
            },
        ];
        let (horizontal_pos, depth_pos) =
            get_final_diving_position_adjusted_by_aim(aim, (horizontal_pos, depth_pos), actions);
        assert_eq!(horizontal_pos * depth_pos, 900);
    }
}
