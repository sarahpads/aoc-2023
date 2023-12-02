use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let mut index = 1;
    let mut total = 0;

    let answer: u32 = read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(format_line)
        .inspect(|line| {
            total += line;
            println!("Line {number}: {total}", number = index, total = total);
            index += 1;
        })
        .sum();

    println!("{:?}", answer)
}

fn format_line(line: &str) -> u32 {
    let mut test = HashMap::new();
    test.insert("1", '1');
    test.insert("2", '2');
    test.insert("3", '3');
    test.insert("4", '4');
    test.insert("5", '5');
    test.insert("6", '6');
    test.insert("7", '7');
    test.insert("8", '8');
    test.insert("9", '9');
    test.insert("one", '1');
    test.insert("two", '2');
    test.insert("three", '3');
    test.insert("four", '4');
    test.insert("five", '5');
    test.insert("six", '6');
    test.insert("seven", '7');
    test.insert("eight", '8');
    test.insert("nine", '9');

    // this is bad - don't want to allocate all of that memory if we don't need it
    let mut letters = vec![None; 50];

    for (key, value) in &test {
        for (index, _) in line.match_indices(key) {
            letters[index] = Some(value.to_owned());
        }
    }

    let numbers = letters.iter()
        .filter_map(|char| char.to_owned())
        .collect::<Vec<char>>();

    return format!("{0}{1}", numbers.first().unwrap(), numbers.last().unwrap())
        .parse()
        .unwrap();
}

