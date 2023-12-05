use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::fmt;
use itertools::Itertools;

struct Bounds {
    number: String,
    top: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    bottom: Option<usize>
}

impl fmt::Debug for Bounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Bounds")
         .field("number", &self.number)
         .field("top", &self.top)
         .field("left", &self.left)
         .field("right", &self.right)
         .field("bottom", &self.bottom)
         .finish()
    }
}

fn main() {
    let mut parts: Vec<Bounds> = Vec::new();

    let file = read_to_string("../input.txt").unwrap();
    let schematic = file
        .lines()
        .collect::<Vec<&str>>();
    let file_length = schematic.len();
    let line_length = schematic.first().unwrap().len();

    for (line_index, line) in schematic.iter().enumerate() {
        line.chars()
            .enumerate()
            .group_by(|(_, char)| char.is_digit(10))
            .into_iter()
            // filter out any groups that aren't digits
            .filter(|(key, _)| *key)
            .for_each(|(_, group)| {
                // create a vector so we can use first/last
                let test = group.collect::<Vec<(usize, char)>>();
                let right = test.last().unwrap().0.checked_add(1);
                let bottom = line_index.checked_add(1);

                let bounds = Bounds {
                    number: test.iter().map(|(_, char)| char).collect(),
                    top: line_index.checked_sub(1),
                    left: test.first().unwrap().0.checked_sub(1),
                    right: if right.unwrap_or(line_length) == line_length { None } else { right },
                    bottom: if bottom.unwrap_or(file_length) == file_length { None } else { bottom }
                };

                parts.push(bounds);
            });
    }

    let mut gears: HashMap<(usize, usize), HashSet<u32>> = HashMap::new();

    // for each part, check all adjacent coords and check for non-alphanumeric character
    for bounds in parts.iter() {
        let number = bounds.number.parse().unwrap();
        let mut coords: Vec<(usize, usize)> = Vec::new();

        if bounds.top.is_some() {
            let top = bounds.top.unwrap();
            let left = bounds.left.unwrap_or(0);
            let right = bounds.right.unwrap_or(line_length - 1);
            for x in left..=right {
                coords.push((x, top));
            }
        }

        if bounds.right.is_some() {
            let right = bounds.right.unwrap();
            let top = bounds.top.unwrap_or(0);
            let bottom = bounds.bottom.unwrap_or(file_length - 1);
            for y in top..=bottom {
                coords.push((right, y));
            }
        }

        if bounds.bottom.is_some() {
            let bottom = bounds.bottom.unwrap();
            let left = bounds.left.unwrap_or(0);
            let right = bounds.right.unwrap_or(line_length - 1);
            for x in left..=right {
                coords.push((x, bottom));
            }
        }

        if bounds.left.is_some() {
            let left = bounds.left.unwrap();
            let top = bounds.top.unwrap_or(0);
            let bottom = bounds.bottom.unwrap_or(file_length - 1);
            for y in top..=bottom {
                coords.push((left, y));
            }
        }

        for coord in coords {
            let char = schematic[coord.1].chars().nth(coord.0).unwrap();

            if char == '*' {
                if let Some(entry) = gears.get_mut(&coord) {
                    entry.insert(number);
                } else {
                    gears.insert(coord, HashSet::from([number]));
                }
            }
        }
    }

    let answer: u32 = gears.into_iter()
        .filter_map(|(coord, parts)| {
            if parts.len() != 2 {
                return None
            } else {
                return Some(parts.into_iter().product::<u32>());
            }

        })
        .sum();

    println!("Sum {:?}", answer);
}

