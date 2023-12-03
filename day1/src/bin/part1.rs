use std::fmt::{Error, format};
use std::fs;
use std::num::ParseIntError;

fn main() {
    let file = fs::read_to_string("input1.txt");
    let string = file.unwrap();
    let result = process(string.as_str());
    print!("{}", result.unwrap())
}

fn process(input: &str) -> Result<u32, ParseIntError> {
    let output = input.lines().map(|line| {
        let mut numbers_iterator = line.chars().filter_map(|char| {
            char.to_digit(10)
        });

        let first = numbers_iterator
            .next()
            .expect("Should be a valid number");

        return match numbers_iterator.last() {
            Some(num) => format!("{first}{num}"),
            None => { format!("{first}{first}") }
        }
            .parse::<u32>()
            .expect("Should be a valid number");
    })
        .sum::<u32>();
    return Ok(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(142, process("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet").unwrap());
    }
}