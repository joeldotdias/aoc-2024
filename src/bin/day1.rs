use std::{collections::HashMap, fs};

fn main() {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    fs::read_to_string("input/day1")
        .unwrap()
        .trim()
        .split('\n')
        .for_each(|line| {
            let mut parts = line.split_whitespace();
            let (n1, n2) = (parts.next().unwrap(), parts.next().unwrap());
            left.push(n1.parse().unwrap());
            right.push(n2.parse().unwrap());
        });

    left.sort();
    right.sort();

    let part1 = part1(&left, &right);
    println!("Part1: {part1}");

    let part2 = part2(&left, &right);
    println!("Part2: {part2}");
}

fn part1(left: &[u32], right: &[u32]) -> u32 {
    let diff_sum: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| (a.max(b) - a.min(b)))
        .sum();

    diff_sum
}

fn part2(left: &[u32], right: &[u32]) -> u32 {
    let r_counts = right.iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });

    let sim_sum: u32 = left.iter().fold(0, |mut acc, num| {
        acc += match r_counts.get(num) {
            Some(count) => num * *count,
            None => 0,
        };

        acc
    });

    sim_sum
}
