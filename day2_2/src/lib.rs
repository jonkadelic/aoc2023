use lazy_static::lazy_static;
use regex::Regex;

pub const PUZZLE: Puzzle = Puzzle { name: "day2_2" };

pub struct Puzzle {
    name: &'static str
}

impl aoc_common::Puzzle for Puzzle {
    fn get_name(&self) -> &'static str {
        self.name
    }

    fn get_expected_test_result(&self) -> &str {
        "2286"
    }

    fn get_result(&self, lines: Vec<String>) -> String {
        let mut acc: u32 = 0;
        for line in &lines {
            acc += parse_line(line) as u32
        }
        acc.to_string()
    }
}

lazy_static! {
    static ref COLOR_COUNT_REGEX: Regex = Regex::new(r"(\d+) (red|green|blue)").unwrap();
}

fn parse_line(line: &str) -> u32 {
    let (mut min_red, mut min_green, mut min_blue): (u32, u32, u32) = (0, 0, 0);

    let parts = line.split(";");
    for part in parts {
        let (red, green, blue) = parse_handful(part);
        if red > min_red {
            min_red = red;
        }
        if green > min_green {
            min_green = green;
        }
        if blue > min_blue {
            min_blue = blue;
        }
    }

    min_red * min_green * min_blue
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