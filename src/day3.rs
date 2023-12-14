use std::{collections::HashSet, fs};

pub fn part1() {
    let path = "data/day3.txt";
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    let mut parts = Vec::new();
    let mut symbols: HashSet<(i32, i32)> = HashSet::new();
    let mut curr_part_number: Option<PartNumber> = None;
    for (row, line) in contents.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                if let Some(ref mut part_number) = curr_part_number {
                    part_number.add_digit(row as i32, col as i32, ch);
                } else {
                    curr_part_number = Some(PartNumber::new(row as i32, col as i32, ch));
                }
                // its a part number
            } else {
                if let Some(part_number) = curr_part_number.take() {
                    parts.push(part_number);
                }
                if ch == '.' {
                    continue;
                }
                symbols.insert((row.try_into().unwrap(), col.try_into().unwrap()));
            }
        }
    }
    let mut sum = 0;
    for part in parts {
        if part.surrounding.intersection(&symbols).next().is_some() {
            sum += part.number;
        }
    }
    println!("{:?}", sum);
}

#[derive(Debug)]
struct PartNumber {
    number: i32,
    surrounding: HashSet<(i32, i32)>,
}

impl PartNumber {
    fn new(row: i32, col: i32, ch: char) -> Self {
        let surrounding = HashSet::from([
            (row - 1, col - 1),
            (row, col - 1),
            (row + 1, col - 1), // left column
            (row - 1, col),
            (row + 1, col), // middle column
            (row - 1, col + 1),
            (row, col + 1),
            (row + 1, col + 1), // right column
        ]);
        Self {
            number: ch.to_digit(10).unwrap() as i32,
            surrounding,
        }
    }

    fn add_digit(&mut self, row: i32, col: i32, ch: char) {
        self.number = self.number * 10 + ch.to_digit(10).unwrap() as i32;
        self.surrounding.extend([
            (row - 1, col + 1),
            (row, col + 1),
            (row + 1, col + 1), // right column
        ]);
    }
}

pub fn part2() {
    let path = "data/day3.txt";
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let mut parts = Vec::new();
    let mut symbols: HashSet<(i32, i32)> = HashSet::new();
    let mut curr_part_number: Option<PartNumber> = None;
    for (row, line) in contents.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                if let Some(ref mut part_number) = curr_part_number {
                    part_number.add_digit(row as i32, col as i32, ch);
                } else {
                    curr_part_number = Some(PartNumber::new(row as i32, col as i32, ch));
                }
                // its a part number
            } else {
                if let Some(part_number) = curr_part_number.take() {
                    parts.push(part_number);
                }
                if ch == '*' {
                    symbols.insert((row.try_into().unwrap(), col.try_into().unwrap()));
                }
            }
        }
    }
    let mut sum = 0;
    for symbol in symbols {
        let gear_parts = parts
            .iter()
            .filter(|part| part.surrounding.contains(&symbol))
            .collect::<Vec<_>>();

        if gear_parts.len() == 2 {
            sum += gear_parts.iter().map(|part| part.number).product::<i32>();
        }
    }
    println!("{:?}", sum);
}
