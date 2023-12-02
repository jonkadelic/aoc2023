use lazy_static::lazy_static;
use regex::Regex;

pub const PUZZLE: Puzzle = Puzzle { name: "day2_1" };

pub struct Puzzle {
    name: &'static str
}

impl aoc_common::Puzzle for Puzzle {
    fn get_name(&self) -> &'static str {
        self.name
    }

    fn get_expected_test_result(&self) -> &str {
        "8"
    }

    fn get_result(&self, lines: Vec<String>) -> String {
        let mut acc: u32 = 0;
        for line in &lines {
            acc += parse_line(line) as u32
        }
        acc.to_string()
    }
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

lazy_static! {
    static ref GAME_NUMBER_REGEX: Regex = Regex::new(r"Game (\d+)").unwrap();
    static ref COLOR_COUNT_REGEX: Regex = Regex::new(r"(\d+) (red|green|blue)").unwrap();
}

fn parse_line(line: &str) -> u32 {
    let game_number_captures = GAME_NUMBER_REGEX.captures(line).unwrap();
    if game_number_captures.len() != 2 {
        panic!("{:?}", game_number_captures);
    }

    let game_number = game_number_captures[1].parse::<u32>().unwrap();
    let mut is_valid = true;
    let parts = line.split(";");
    for part in parts {
        let (red, green, blue) = parse_handful(part);
        if red > MAX_RED || green > MAX_GREEN || blue > MAX_BLUE {
            is_valid = false;
            break;
        }
    }

    return match is_valid {
        true => game_number,
        false => 0
    };
}

fn parse_handful(desc: &str) -> (u32, u32, u32) {
    let mut red: u32 = 0;
    let mut green: u32 = 0;
    let mut blue: u32 = 0;
    let parts = desc.split(",");
    for part in parts {
        let captures = COLOR_COUNT_REGEX.captures(part).unwrap();
        let value = captures[1].parse::<u32>().unwrap();
        match &captures[2] {
            "red" => red = value,
            "green" => green = value,
            "blue" => blue = value,
            _ => panic!()
        };
    }
    (red, green, blue)
}