use common::parse_strings_without_split_sign_from_lines_in_file;
use std::{error::Error, path::Path};

struct CorruptedLine {
    illeagal_character: String,
}

fn read_sign_chunks_from_file(
    filename: impl AsRef<Path>,
) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    parse_strings_without_split_sign_from_lines_in_file(filename)
}

fn get_corrupted_lines(sign_chunks: Vec<Vec<String>>) -> Vec<CorruptedLine> {
    let mut corrupted_lines = vec![];
    for sign_line in sign_chunks {
        let mut stack: Stack<String> = Stack::new();
        for sign in sign_line {
            match sign.as_ref() {
                "(" | "[" | "{" | "<" => stack.push(sign),
                ")" | "]" | "}" | ">" => {
                    let expected_sign = match sign.as_ref() {
                        ")" => "(".to_string(),
                        "]" => "[".to_string(),
                        "}" => "{".to_string(),
                        ">" => "<".to_string(),
                        _ => String::new(),
                    };

                    let illeagal_character = match stack.pop() {
                        Some(e) => {
                            if e == expected_sign {
                                continue;
                            } else {
                                sign
                            }
                        }
                        None => sign,
                    };
                    corrupted_lines.push(CorruptedLine { illeagal_character });
                    break;
                }
                _ => panic!("not recognized sign found {}", sign),
            }
            //println!("the stack is {:?}", stack);
        }
    }
    corrupted_lines
}

fn get_total_points_of_illegal_characters(corrupted_lines: Vec<CorruptedLine>) -> i64 {
    let mut total = 0;
    for line in corrupted_lines {
        total += match line.illeagal_character.as_str() {
            ")" => 3,
            "]" => 57,
            "}" => 1197,
            ">" => 25137,
            _ => 0,
        }
    }

    total
}

#[derive(Debug)]
struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    // fn length(&self) -> usize {
    //     self.stack.len()
    // }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    // fn is_empty(&self) -> bool {
    //     self.stack.is_empty()
    // }

    // fn peek(&self) -> Option<&T> {
    //     self.stack.last()
    // }
}

fn main() {
    let filename = "day10_input.txt";
    let sign_chunks = read_sign_chunks_from_file(filename).unwrap();
    let corrupted_lines = get_corrupted_lines(sign_chunks);

    let total_points = get_total_points_of_illegal_characters(corrupted_lines);
    println!("total_points is {}", total_points);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_all_sign_chunks_from_file_by_line_given_the_filename() {
        let filename = "day10_test.txt";
        let sign_chunks = read_sign_chunks_from_file(filename).unwrap();

        assert_eq!(sign_chunks.len(), 10);
    }

    #[test]
    fn should_get_all_corrupted_lines_given_sign_chunks() {
        let filename = "day10_test.txt";
        let sign_chunks = read_sign_chunks_from_file(filename).unwrap();

        let corrupted_lines = get_corrupted_lines(sign_chunks);

        assert_eq!(corrupted_lines.len(), 5);
        assert_eq!(corrupted_lines[0].illeagal_character, "}".to_string());
        assert_eq!(corrupted_lines[1].illeagal_character, ")".to_string());
        assert_eq!(corrupted_lines[2].illeagal_character, "]".to_string());
        assert_eq!(corrupted_lines[3].illeagal_character, ")".to_string());
        assert_eq!(corrupted_lines[4].illeagal_character, ">".to_string());

        let total_points = get_total_points_of_illegal_characters(corrupted_lines);
        assert_eq!(total_points, 26397);
    }
}
