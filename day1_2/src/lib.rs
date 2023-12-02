use std::{iter::Map, collections::HashMap};

use phf::phf_map;

pub const PUZZLE: Puzzle = Puzzle { name: "day1_2" };

pub struct Puzzle {
    name: &'static str
}

impl aoc_common::Puzzle for Puzzle {
    fn get_name(&self) -> &'static str {
        self.name
    }

    fn get_expected_test_result(&self) -> &str {
        "281"
    }

    fn get_result(&self, lines: Vec<String>) -> String {
        let mut acc: u32 = 0;
        for line in &lines {
            acc += parse_line(line) as u32
        }
        acc.to_string()
    }
}

const NUMBER_MAP: phf::Map<&'static str, u8> = phf_map!(
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9
);

fn parse_line(line: &str) -> u8 {
    let mut first_number: Option<u8> = None;
    let mut last_number: Option<u8> = None;

    for c in line.char_indices() {
        let converted = match c.1.to_digit(10) {
            Some(d) => Some(d as u8),
            None => {
                let mut out: Option<u8> = None;
                for s in NUMBER_MAP.into_iter() {
                    let substr = line.get(c.0..(c.0 + s.0.len()));
                    if substr.is_some() {
                        let substr = substr.unwrap();
                        if substr == *s.0 {
                            out = Some(*s.1);
                        }
                    }
                }
                out
            }
        };
        if converted.is_some() {
            if first_number.is_none() {
                first_number = converted;
                last_number = converted;
            } else {
                last_number = converted;
            }
        }
    }

    if first_number.is_some() && last_number.is_some() {
        return first_number.unwrap() * 10 + last_number.unwrap();
    } else {
        panic!();
    }
}