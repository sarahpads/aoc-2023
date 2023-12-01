use std::fs::read_to_string;

fn main() {
    let test: u32 = read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(format_line)
        .sum();

    println!("{:?}", test)
}

fn format_line(line: &str) -> u32 {
    let numbers: Vec<char> = line
        .chars()
        .filter(|char| char.to_digit(10).is_some())
        .collect::<Vec<char>>();

    return format!("{0}{1}", numbers.first().unwrap(), numbers.last().unwrap()).parse().unwrap();
}

