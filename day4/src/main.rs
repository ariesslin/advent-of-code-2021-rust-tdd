use common::lines_from_file;
use std::{collections::HashMap, error::Error, path::Path};

#[derive(Clone, Debug)]
struct BingoBoard {
    pos_mapping: HashMap<i64, (usize, usize)>,
    val_mapping: Vec<Vec<i64>>,
    row_sum: Vec<i64>,
    column_sum: Vec<i64>,
}

fn read_bingo_input_from_file(filename: impl AsRef<Path>) -> Result<Vec<i64>, Box<dyn Error>> {
    let lines_from_file = lines_from_file(filename)?;
    let mut nums = vec![];
    let line = lines_from_file[0].clone();
    let splits = line.trim().split(',');
    for split in splits {
        let num = split.parse::<i64>()?;
        nums.push(num);
    }
    //println!("{:?}", nums);
    Ok(nums)
}

fn get_bingo_boards_from_file(
    filename: impl AsRef<Path>,
) -> Result<Vec<BingoBoard>, Box<dyn Error>> {
    let lines_from_file = lines_from_file(filename)?;
    let board_nums = lines_from_file.len() / 6;
    let mut bingo_boards = vec![];

    for board_num in 0..board_nums {
        let mut row_index = 0;
        let mut pos_mapping = HashMap::new();
        for line in &lines_from_file[2 + board_num * 6..7 + board_num * 6] {
            //println!("{:?}", line);
            let nums = line.trim().split_whitespace().collect::<Vec<&str>>();
            if nums.len() == 5 {
                for (column_index, num) in nums.iter().enumerate() {
                    pos_mapping.insert(num.parse::<i64>().unwrap(), (row_index, column_index));
                }
                row_index += 1;
            } else {
                panic!("Error pasing the board");
            }
        }
        let bingo_board = BingoBoard {
            pos_mapping,
            val_mapping: vec![vec![0; 5]; 5],
            row_sum: vec![0; 5],
            column_sum: vec![0; 5],
        };
        bingo_boards.push(bingo_board);
    }
    //println!("bingo boards are {:?}", bingo_boards);
    Ok(bingo_boards)
}

fn get_winner_bingo_board(
    bingo_inputs: Vec<i64>,
    mut bingo_boards: Vec<BingoBoard>,
    get_last: bool,
) -> (Option<usize>, i64, Option<BingoBoard>) {
    let mut count = 0;
    let bingo_boards_num = bingo_boards.len();
    let mut board_win_record = vec![0; bingo_boards_num];

    for bingo_input in bingo_inputs {
        for (board_number, bingo_board) in bingo_boards.iter_mut().enumerate() {
            if board_win_record[board_number] == 1 {
                continue;
            }
            //println!("bingo board is {:?}", bingo_board);
            let (x, y) = match bingo_board.pos_mapping.get(&bingo_input) {
                Some((x, y)) => (x, y),
                None => continue,
            };
            bingo_board.val_mapping[*x][*y] = 1;
            bingo_board.row_sum[*x] += 1;
            bingo_board.column_sum[*y] += 1;
            for i in bingo_board.row_sum.clone() {
                if i == 5 {
                    board_win_record[board_number] = 1;
                }
            }
            for j in bingo_board.column_sum.clone() {
                if j == 5 {
                    board_win_record[board_number] = 1;
                }
            }
            if board_win_record[board_number] == 1 {
                count += 1;
                if get_last {
                    if count == bingo_boards_num {
                        return (Some(board_number), bingo_input, Some(bingo_board.clone()));
                    }
                } else {
                    return (Some(board_number), bingo_input, Some(bingo_board.clone()));
                }
            }
        }
    }

    (None, 0, None)
}

fn get_sum_of_unmarked_number_in_board(bingo_board: BingoBoard) -> i64 {
    let mut sum = 0;
    for (num, (x, y)) in bingo_board.pos_mapping {
        if bingo_board.val_mapping[x][y] == 0 {
            sum += num;
        }
    }
    sum
}

fn main() {
    let filename = "day4_input.txt";
    let bingo_inputs = read_bingo_input_from_file(filename).unwrap();
    let bingo_boards = get_bingo_boards_from_file(filename).unwrap();
    let (_winner, bingo_input, bingo_board) =
        get_winner_bingo_board(bingo_inputs.clone(), bingo_boards.clone(), false);

    let sum = get_sum_of_unmarked_number_in_board(bingo_board.unwrap());

    println!("The first round final result is {}", bingo_input * sum);

    let (_winner, bingo_input, bingo_board) =
        get_winner_bingo_board(bingo_inputs, bingo_boards, true);

    let sum = get_sum_of_unmarked_number_in_board(bingo_board.unwrap());

    println!("The second round final result is {}", bingo_input * sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_all_bingo_inputs_in_the_first_line_from_file_given_the_filename() {
        let filename = "day4_test.txt";
        let bingo_inputs = read_bingo_input_from_file(filename).unwrap();

        assert_eq!(bingo_inputs.len(), 27);
    }

    #[test]
    fn should_get_all_bingo_boards_from_file_given_the_filename() {
        let filename = "day4_test.txt";
        let boards = get_bingo_boards_from_file(filename).unwrap();

        assert_eq!(boards.len(), 3);
        assert_eq!(boards[0].pos_mapping.len(), 25);
    }

    #[test]
    fn should_get_winner_given_the_bingo_inputs_and_bingo_boards() {
        let filename = "day4_test.txt";
        let bingo_inputs = read_bingo_input_from_file(filename).unwrap();
        let bingo_boards = get_bingo_boards_from_file(filename).unwrap();
        let (winner, bingo_input, bingo_board) =
            get_winner_bingo_board(bingo_inputs, bingo_boards, false);

        let sum = get_sum_of_unmarked_number_in_board(bingo_board.unwrap());

        assert_eq!(winner, Some(2));
        assert_eq!(bingo_input, 24);
        assert_eq!(sum, 188);
    }

    #[test]
    fn should_get_last_winner_given_the_bingo_inputs_and_bingo_boards() {
        let filename = "day4_test.txt";
        let bingo_inputs = read_bingo_input_from_file(filename).unwrap();
        let bingo_boards = get_bingo_boards_from_file(filename).unwrap();
        let (winner, bingo_input, bingo_board) =
            get_winner_bingo_board(bingo_inputs, bingo_boards, true);

        let sum = get_sum_of_unmarked_number_in_board(bingo_board.unwrap());

        assert_eq!(winner, Some(1));
        assert_eq!(bingo_input, 13);
        assert_eq!(sum, 148);
    }
}
