use common::parse_strings_without_split_sign_from_lines_in_file;
use std::{error::Error, path::Path};

enum LineStatus {
    Corrupted { illeagal_character: String },
    InCompleted { to_complete_stack: Stack<String> },
    Completed,
}

struct CorruptedLine {
    illeagal_character: String,
}

struct IncompleteLine {
    completion_string: Vec<String>,
}

fn read_sign_chunks_from_file(
    filename: impl AsRef<Path>,
) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    parse_strings_without_split_sign_from_lines_in_file(filename)
}

fn get_line_states(sign_chunks: Vec<Vec<String>>) -> Vec<LineStatus> {
    let mut line_states = vec![];
    for sign_line in sign_chunks {
        let mut stack: Stack<String> = Stack::new();
        let mut is_corrupted = false;
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
                    line_states.push(LineStatus::Corrupted { illeagal_character });
                    is_corrupted = true;
                    break;
                }
                _ => panic!("not recognized sign found {}", sign),
            }
            //println!("the stack is {:?}", stack);
        }
        if !is_corrupted {
            if stack.is_empty() {
                line_states.push(LineStatus::Completed);
            } else {
                line_states.push(LineStatus::InCompleted {
                    to_complete_stack: stack,
                });
            }
        }
    }
    line_states
}

fn get_corrupted_lines(sign_chunks: Vec<Vec<String>>) -> Vec<CorruptedLine> {
    let line_states = get_line_states(sign_chunks);

    let mut corrupted_lines = vec![];
    for line_state in line_states {
        if let LineStatus::Corrupted { illeagal_character } = line_state {
            corrupted_lines.push(CorruptedLine { illeagal_character });
        }
    }

    corrupted_lines
}

fn get_incompleted_lines(sign_chunks: Vec<Vec<String>>) -> Vec<IncompleteLine> {
    let line_states = get_line_states(sign_chunks);

    let mut incompleted_lines = vec![];
    for line_state in line_states {
        if let LineStatus::InCompleted {
            mut to_complete_stack,
        } = line_state
        {
            let mut v = vec![];
            while !to_complete_stack.is_empty() {
                let to_complete_str = to_complete_stack.pop().unwrap();
                match to_complete_str.as_str() {
                    "(" => v.push(")".to_string()),
                    "[" => v.push("]".to_string()),
                    "{" => v.push("}".to_string()),
                    "<" => v.push(">".to_string()),
                    _ => panic!("not recognized sign found {}", to_complete_str),
                }
            }
            //println!("the completion strings are {:?}", v);
            incompleted_lines.push(IncompleteLine {
                completion_string: v,
            });
        }
    }

    incompleted_lines
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

fn get_completion_string_scores(incompleted_lines: Vec<IncompleteLine>) -> Vec<i64> {
    let mut scores = vec![];

    for line in incompleted_lines {
        let mut count = 0;
        for str in line.completion_string {
            count *= 5;
            count += match str.as_ref() {
                ")" => 1,
                "]" => 2,
                "}" => 3,
                ">" => 4,
                _ => 0,
            }
        }

        scores.push(count);
    }

    scores
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

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    // fn peek(&self) -> Option<&T> {
    //     self.stack.last()
    // }
}

fn main() {
    let filename = "day10_input.txt";
    let sign_chunks = read_sign_chunks_from_file(filename).unwrap();
    let corrupted_lines = get_corrupted_lines(sign_chunks.clone());

    let total_points = get_total_points_of_illegal_characters(corrupted_lines);
    println!("total_points is {}", total_points);

    let incompleted_lines = get_incompleted_lines(sign_chunks);
    let mut completion_string_scores = get_completion_string_scores(incompleted_lines);
    completion_string_scores.sort_unstable();
    println!(
        "final score is {}",
        completion_string_scores[completion_string_scores.len() / 2]
    );
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

    #[test]
    fn should_get_all_incompleted_lines_given_sign_chunks() {
        let filename = "day10_test.txt";
        let sign_chunks = read_sign_chunks_from_file(filename).unwrap();

        let incompleted_lines = get_incompleted_lines(sign_chunks);

        assert_eq!(incompleted_lines.len(), 5);
        assert_eq!(
            incompleted_lines[0].completion_string.join(""),
            "}}]])})]".to_string()
        );
        assert_eq!(
            incompleted_lines[1].completion_string.join(""),
            ")}>]})".to_string()
        );
        assert_eq!(
            incompleted_lines[2].completion_string.join(""),
            "}}>}>))))".to_string()
        );
        assert_eq!(
            incompleted_lines[3].completion_string.join(""),
            "]]}}]}]}>".to_string()
        );
        assert_eq!(
            incompleted_lines[4].completion_string.join(""),
            "])}>".to_string()
        );

        let mut completion_string_scores = get_completion_string_scores(incompleted_lines);

        assert_eq!(completion_string_scores[0], 288957);
        assert_eq!(completion_string_scores[1], 5566);
        assert_eq!(completion_string_scores[2], 1480781);
        assert_eq!(completion_string_scores[3], 995444);
        assert_eq!(completion_string_scores[4], 294);

        completion_string_scores.sort_unstable();
        assert_eq!(
            completion_string_scores[completion_string_scores.len() / 2],
            288957
        );
    }
}
