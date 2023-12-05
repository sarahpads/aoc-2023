use std::{fs::read_to_string, collections::HashSet};
use itertools::Itertools; // 0.9.0

fn main() {
    let answer: i32 = read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(format_line)
        .sum();

    println!("answer {:?}", answer);
}

fn format_line(line: &str) -> i32 {
    let mut test = line.to_string();
    test.replace_range(..line.find(":").unwrap() + 2, "");

    let (winning_numbers, our_numbers) = test.split("|")
        .map(format_segment)
        .next_tuple().unwrap();

    let mut points = 0;

    for _ in winning_numbers.intersection(&our_numbers) {
        if points == 0 { points = 1 } else { points *= 2 };
    }

    return points;
}

fn format_segment(segment: &str) -> HashSet<u32> {
    return segment
        .split_whitespace()
        .map(|s| s.to_owned().parse().unwrap()).collect::<HashSet<u32>>();
}
