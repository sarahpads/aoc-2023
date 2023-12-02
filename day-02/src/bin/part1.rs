use std::fs::read_to_string;
use std::cmp::max;

struct Game {
    id: u32,
    red: u32,
    blue: u32,
    green: u32
}

fn main() {
    let test: u32 = read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(format_game)
        .filter(filter_game)
        .map(|game| game.id)
        .sum();

    println!("{}", test);
}


fn filter_game(game: &Game) -> bool {
    let red_count = 12;
    let green_count = 13;
    let blue_count = 14;
    println!("is possible");
    println!("Game {id} red: {red} green: {green} blue: {blue}",
        id = game.id,
        green = game.green,
        red = game.red,
        blue = game.blue
    );

    return game.blue <= blue_count
        && game.red <= red_count
        && game.green <= green_count;
}

fn format_game(line: &str) -> Game {
    println!("format_game");
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
