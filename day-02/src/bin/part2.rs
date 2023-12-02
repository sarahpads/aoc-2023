
use std::fs::read_to_string;
use std::cmp::max;

struct Game {
    id: u32,
    red: u32,
    blue: u32,
    green: u32
}

fn main() {
    let answer: u32 = read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(format_game)
        .filter(filter_game)
        .map(|game| game.green * game.red * game.blue)
        .sum();

    println!("{}", answer);
}

fn filter_game(game: &Game) -> bool {
    println!("Game {id} red: {red} green: {green} blue: {blue} power: {power}",
        id = game.id,
        green = game.green,
        red = game.red,
        blue = game.blue,
        power = game.green * game.red * game.blue
    );

    return true;
}

fn format_game(line: &str) -> Game {
    let mut game = Game {
        id: 0,
        red: 0,
        blue: 0,
        green: 0
    };

    let test = line.split_whitespace()
        .map(|value| value.chars().filter(|char| char.is_alphanumeric()).collect::<String>())
        .collect::<Vec<String>>();
    
    for window in test.chunks(2) {
        if window[0] == "Game" {
            game.id = window[1].parse().unwrap();
        } else if window[1] == "red" {
            game.red = max(game.red, window[0].parse::<u32>().unwrap());
        } else if window[1] == "green" {
            game.green = max(game.green, window[0].parse::<u32>().unwrap());
        } else if window[1] == "blue" {
            game.blue = max(game.blue, window[0].parse::<u32>().unwrap());
        }
    }

    return game;
}
