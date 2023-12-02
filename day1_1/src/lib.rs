use std::{iter::Map, collections::HashMap};

pub const PUZZLE: Puzzle = Puzzle { name: "day1_1" };

pub struct Puzzle {
    name: &'static str
}

impl aoc_common::Puzzle for Puzzle {
    fn get_name(&self) -> &'static str {
        self.name
    }

    fn get_expected_test_result(&self) -> &str {
        "142"
    }

    fn get_result(&self, lines: Vec<String>) -> String {
        let mut acc: u32 = 0;
        for line in &lines {
            acc += parse_line(line) as u32
        }
        acc.to_string()
    }
}

fn parse_line(line: &str) -> u8 {
    let mut first_number: Option<u8> = None;
    let mut last_number: Option<u8> = None;

    for c in line.chars() {
        let converted = match c.to_digit(10) {
            Some(d) => d,
            None => continue
        } as u8;
        if first_number.is_none() {
            first_number = Some(converted);
            last_number = Some(converted);
        } else {
            last_number = Some(converted);
        }
    }

    if first_number.is_some() && last_number.is_some() {
        return first_number.unwrap() * 10 + last_number.unwrap();
    } else {
        panic!();
    }
}