use common::lines_from_file;
use std::{error::Error, path::Path};

#[derive(Debug, Clone)]
struct ThermalManual {
    transparent_paper: Vec<Vec<i64>>,
    fold_instructions: Vec<(String, i64)>,
}

fn get_transparent_paper_from_file(
    filename: impl AsRef<Path>,
) -> Result<ThermalManual, Box<dyn Error>> {
    let lines_from_file = lines_from_file(filename)?;
    let mut x_max = 0;
    let mut y_max = 0;
    let mut init_list = vec![];
    let mut index_fold = 0;
    for (index, line) in lines_from_file.iter().enumerate() {
        if line.is_empty() {
            index_fold = index;
            break;
        }
        let mut splits = line.trim().split(',');
        let x = splits.next().unwrap().parse::<i64>()?;
        let y = splits.next().unwrap().parse::<i64>()?;
        if x > x_max {
            x_max = x;
        }
        if y > y_max {
            y_max = y;
        }
        init_list.push((x, y));
    }

    let mut paper = vec![vec![0; x_max as usize + 1]; y_max as usize + 1];
    for item in init_list {
        paper[item.1 as usize][item.0 as usize] = 1;
    }
    //println!("the init transparent paper is {:?}", paper);

    let mut instructions = vec![];
    for line in &lines_from_file[index_fold + 1..] {
        let splits = line.trim().split_whitespace();
        let mut fold_split = splits.last().unwrap().trim().split('=');
        let direction = fold_split.next().unwrap().to_string();
        let pos = fold_split.next().unwrap().parse::<i64>()?;
        instructions.push((direction, pos));
    }

    Ok(ThermalManual {
        transparent_paper: paper,
        fold_instructions: instructions,
    })
}

fn get_folded_paper_and_remaining_instructions(manual: ThermalManual) -> ThermalManual {
    let height = manual.transparent_paper.len();
    let width = manual.transparent_paper[0].len();
    let mut transparent_paper;

    let (direction, pos) = manual.fold_instructions[0].clone();
    match direction.as_str() {
        "x" => {
            let x_left_width = pos;
            let x_right_width = width as i64 - 1 - pos;
            let new_width = if x_left_width >= x_right_width {
                x_left_width
            } else {
                x_right_width
            };
            transparent_paper = vec![vec![0; new_width as usize]; height];

            for (row, item) in transparent_paper.iter_mut().enumerate().take(height) {
                *item = manual.transparent_paper[row][0..pos as usize].to_vec();
            }

            for (row, item) in transparent_paper.iter_mut().enumerate().take(height) {
                for column in pos as usize + 1..width {
                    item[width - 1 - column] |= manual.transparent_paper[row][column];
                }
            }
        }
        _ => {
            let y_up_height = pos;
            let y_down_height = height as i64 - 1 - pos;
            let new_height = if y_up_height >= y_down_height {
                y_up_height
            } else {
                y_down_height
            };
            transparent_paper = vec![vec![0; width]; new_height as usize];

            for row in 0..pos {
                transparent_paper[(new_height - pos + row) as usize] =
                    manual.transparent_paper[row as usize].clone();
            }

            for row in pos + 1..height as i64 {
                transparent_paper[height - 1 - row as usize] = or_operation(
                    &transparent_paper[height - 1 - row as usize],
                    &manual.transparent_paper[row as usize],
                );
            }
        }
    }

    let fold_instructions = manual.fold_instructions[1..].to_vec();

    ThermalManual {
        transparent_paper,
        fold_instructions,
    }
}

fn or_operation(a: &[i64], b: &[i64]) -> Vec<i64> {
    a.iter().zip(b).map(|(a, b)| a | b).collect()
}

fn get_visible_dots_from_paper(paper: Vec<Vec<i64>>) -> i64 {
    let mut dots = 0;
    for row in paper {
        for item in row {
            if item > 0 {
                dots += 1;
            }
        }
    }
    dots
}

fn main() {
    let filename = "day13_input.txt";
    let manual = get_transparent_paper_from_file(filename).unwrap();

    let new_manual = get_folded_paper_and_remaining_instructions(manual);

    println!(
        "the dots in new transparent paper are {:?}",
        get_visible_dots_from_paper(new_manual.transparent_paper)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_trasparent_paper_from_file_given_the_filename() {
        let filename = "day13_test.txt";
        let manual = get_transparent_paper_from_file(filename).unwrap();

        assert_eq!(manual.transparent_paper.len(), 15);
        assert_eq!(manual.transparent_paper[0].len(), 11);

        assert_eq!(manual.fold_instructions.len(), 2);
    }

    #[test]
    fn should_get_new_folded_paper_given_the_initial_paper_and_fold_instructions() {
        let filename = "day13_test.txt";
        let manual = get_transparent_paper_from_file(filename).unwrap();

        let new_manual = get_folded_paper_and_remaining_instructions(manual);

        // println!(
        //     "the new transparent paper is {:?}",
        //     new_manual.transparent_paper
        // );

        assert_eq!(new_manual.transparent_paper.len(), 7);
        assert_eq!(new_manual.transparent_paper[0].len(), 11);

        assert_eq!(new_manual.fold_instructions.len(), 1);

        assert_eq!(
            get_visible_dots_from_paper(new_manual.clone().transparent_paper),
            17
        );

        let new_manual = get_folded_paper_and_remaining_instructions(new_manual);

        // println!(
        //     "the new transparent paper is {:?}",
        //     new_manual.transparent_paper
        // );

        assert_eq!(new_manual.transparent_paper.len(), 7);
        assert_eq!(new_manual.transparent_paper[0].len(), 5);

        assert_eq!(new_manual.fold_instructions.len(), 0);

        assert_eq!(
            get_visible_dots_from_paper(new_manual.clone().transparent_paper),
            16
        );
    }
}
