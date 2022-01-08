use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
