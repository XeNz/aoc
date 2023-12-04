use std::fs;
use std::num::ParseIntError;
use itertools::Itertools;


fn main() {
    let file = fs::read_to_string("./src/bin/input2.txt");
    let string = file.unwrap();
    let result = process(string.as_str());
    print!("{}", result.unwrap())
}

#[derive(Debug)]
struct Game {
    id: u32,
    cubes: Vec<Cube>,
}

impl Game {
    fn new(id: u32, cubes: Vec<Cube>) -> Game {
        Game { id, cubes }
    }
    fn calculate_summary(&self) -> ColorSummary {
        // I wonder how I make this generic in rust :/
        let blue = self.cubes.iter().map(|cube: &Cube| cube.blue_amount).max().or(Option::from(0)).unwrap();
        let red = self.cubes.iter().map(|cube: &Cube| cube.red_amount).max().or(Option::from(0)).unwrap();
        let green = self.cubes.iter().map(|cube: &Cube| cube.green_amount).max().or(Option::from(0)).unwrap();

        return ColorSummary { game_id: self.id, blue_amount: blue, green_amount: green, red_amount: red };
    }
}

#[derive(Debug)]
struct ColorSummary {
    game_id: u32,
    red_amount: u32,
    blue_amount: u32,
    green_amount: u32,
}

impl ColorSummary {
    fn valid_against(&self, rule_set: &RuleSet) -> bool {
        return self.red_amount <= rule_set.max_red
            && self.blue_amount <= rule_set.max_blue
            && self.green_amount <= rule_set.max_green;
    }
}

#[derive(Debug, Clone)]
struct RuleSet {
    max_red: u32,
    max_blue: u32,
    max_green: u32,
}

#[derive(Debug)]
struct Cube {
    red_amount: u32,
    blue_amount: u32,
    green_amount: u32,
}


fn process(input: &str) -> Result<usize, ParseIntError> {
    let rule_set = &RuleSet {
        max_green: 13,
        max_blue: 14,
        max_red: 12,
    };

    let result = parse_games(input)
        .iter()
        .map(|game| {
            return if game.calculate_summary().valid_against(rule_set) {
                dbg!(game.id);
                return game.id;
            } else { 0 };
        })
        .sum::<u32>();
    return Ok(result as usize);
}

fn parse_games(input: &str) -> Vec<Game> {
    return input
        .lines()
        .map(|(line)| {
            let (game_str, cube_lines) = line.split_once(": ").unwrap();
            let (_, game_id) = game_str.split_once("Game ").unwrap();
            dbg!(game_id);
            dbg!(cube_lines);
            let cubes = cube_lines
                .split("; ")
                .map(|cube_entries| {
                    let mut red = 0;
                    let mut blue = 0;
                    let mut green = 0;
                    let amount_with_color = cube_entries.split(", ").collect_vec();
                    amount_with_color.iter().for_each(|str| {
                        let (amount, color) = str.split_once(" ").unwrap();
                        let amount = amount.parse::<u32>().expect("Be a freaking number");

                        match color {
                            "red" => red = red + amount,
                            "blue" => blue = blue + amount,
                            "green" => green = green + amount,
                            _ => {}
                        }
                    });
                    return Cube { red_amount: red, blue_amount: blue, green_amount: green };
                })
                .collect();
            return Game { id: game_id.parse().unwrap(), cubes };
        })
        .collect_vec();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(8, process("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")
            .unwrap());
    }
}