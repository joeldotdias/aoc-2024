use std::fs;

use regex::Regex;

fn main() {
    let data = fs::read_to_string("input/day3").unwrap();

    println!("Part1: {}", part1(&data));

    println!("Part2: {}", part2(data.as_bytes()));
}

fn part1(data: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let sum = re.captures_iter(data).fold(0, |acc, capt| {
        acc + capt[1].parse::<u32>().unwrap() * capt[2].parse::<u32>().unwrap()
    });

    sum
}

fn part2(data: &[u8]) -> u32 {
    let mut sum = 0;
    let mut enabled = true;
    let mut idx = 0;

    while idx < data.len() {
        if data[idx..].starts_with(b"do()") {
            enabled = true;
            idx += 4;
        } else if data[idx..].starts_with(b"don't()") {
            enabled = false;
            idx += 7;
        } else if data[idx..].starts_with(b"mul(") {
            idx += 4;

            let (mut n1, mut n1_len) = (0, 0);
            while data[idx].is_ascii_digit() {
                n1 = 10 * n1 + (data[idx] - b'0') as u32;
                n1_len += 1;
                idx += 1;
            }

            // comma
            if n1_len == 0 || data[idx] != b',' {
                continue;
            }
            idx += 1;

            let (mut n2, mut n2_len) = (0, 0);
            while data[idx].is_ascii_digit() {
                n2 = 10 * n2 + (data[idx] - b'0') as u32;
                n2_len += 1;
                idx += 1;
            }

            if n2_len == 0 || data[idx] != b')' {
                continue;
            }
            idx += 1;

            if enabled {
                sum += n1 * n2;
            }
        } else {
            idx += 1;
            continue;
        }
    }

    sum
}
