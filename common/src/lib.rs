use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

#[derive(Debug, Clone)]
pub struct Stack<T> {
    pub stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    // fn length(&self) -> usize {
    //     self.stack.len()
    // }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

pub fn numbers_from_first_line_in_file(
    filename: impl AsRef<Path>,
) -> Result<Vec<i64>, Box<dyn Error>> {
    let lines_from_file = lines_from_file(filename)?;
    let mut numbers = vec![];
    for number in lines_from_file[0].trim().split(',').collect::<Vec<&str>>() {
        //println!("{:?}", line);
        let number = number.trim().parse::<i64>()?;
        numbers.push(number);
    }
    Ok(numbers)
}

pub fn parse_numbers_without_split_sign_from_lines_in_file(
    filename: impl AsRef<Path>,
) -> Result<Vec<Vec<i64>>, Box<dyn Error>> {
    let lines_from_file = lines_from_file(filename)?;
    let mut numbers = vec![];
    for line in lines_from_file {
        //println!("{:?}", line);
        let chars = line.trim().chars();
        let mut number = vec![];
        for c in chars {
            let bit: i64 = c.to_digit(10).unwrap() as i64;
            number.push(bit);
        }
        numbers.push(number);
    }
    Ok(numbers)
}

pub fn parse_strings_without_split_sign_from_lines_in_file(
    filename: impl AsRef<Path>,
) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let lines_from_file = lines_from_file(filename)?;
    let mut strs = vec![];
    for line in lines_from_file {
        //println!("{:?}", line);
        let chars = line.trim().chars();
        let mut str = vec![];
        for c in chars {
            let bit: String = c.to_string();
            str.push(bit);
        }
        strs.push(str);
    }
    Ok(strs)
}

pub fn get_extended_metrix(metrix: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let height = metrix.len() + 2;
    let width = metrix[0].len() + 2;
    let mut extended_metrix = vec![vec![10; width]; height];
    for (index, extended_row) in extended_metrix[1..height - 1].iter_mut().enumerate() {
        extended_row.splice(1..width - 1, metrix[index].iter().cloned());
    }
    extended_metrix
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
