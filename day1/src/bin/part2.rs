use std::collections::HashMap;
use std::fs;
use std::num::ParseIntError;
use itertools::Itertools;

fn main() {
    let file = fs::read_to_string("./src/bin/input2.txt");
    let string = file.unwrap();
    let result = process(string.as_str());
    print!("{}", result.unwrap())
}


fn process(input: &str) -> Result<usize, ParseIntError> {
    let numbers = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]);

    let output = input.lines().map(|(line)| {
        // index in the string, index of numbers hashmap
        let mut map: HashMap<usize, usize> = HashMap::new();
        for number in &numbers {
            let indices: Vec<_> = line.match_indices(number.0).collect();

            for match_results in indices {
                // filter out indices which have value
                let index_in_string = match_results.0;
                let matching = match_results.1;

                if index_in_string >= 0 {
                    let value = numbers.get(matching).unwrap();
                    println!("inserting value {} for index {}", value.to_string(), index_in_string);
                    map.insert(index_in_string, *value);
                }
            }
        }
        // order by index
        let mut sorted_iter = map.keys().sorted();
        // get first
        let first = map.get(sorted_iter.next().unwrap()).unwrap();
        println!("{}", first.to_string());
        // get last (which will be same as first if only one entry)

        let last = match sorted_iter.last() {
            None => { Option::from(first)}
            Some(value) => {map.get(value)}
        }.unwrap();
        println!("{}", last.to_string());
        // concat string with numbers
        let output = format!("{first}{last}").parse::<usize>().unwrap();
        println!("Final result for line: {}", output.to_string());
        println!("New line");

        return output;
    })
        .sum::<usize>();
    return Ok(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(281, process("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen").unwrap());
    }
}