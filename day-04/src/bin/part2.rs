use std::{fs::read_to_string, collections::HashSet};
use itertools::Itertools; // 0.9.0

fn main() {
    let file = read_to_string("../input.txt").unwrap();
    let len = file.lines().count();

    let mut cards = vec![1; len];

    for (index, wins) in read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(format_line)
        .enumerate() {
            let i: i32 = index.try_into().unwrap();

            for x in 0..wins {
                cards[(i + x + 1) as usize] += cards[index];  
            }

    }

    let answer: u32 = cards.iter().sum();

    println!("answer {:?}", answer);
}

fn format_line(line: &str) -> i32 {
    let mut test = line.to_string();
    test.replace_range(..line.find(":").unwrap() + 2, "");

    let (winning_numbers, our_numbers) = test.split("|")
        .map(format_segment)
        .next_tuple().unwrap();

    return winning_numbers.intersection(&our_numbers).count().try_into().unwrap();
}

fn format_segment(segment: &str) -> HashSet<u32> {
    return segment
        .split_whitespace()
        .map(|s| s.to_owned().parse().unwrap()).collect::<HashSet<u32>>();
}
