use std::{collections::HashSet, fs};

pub fn part1() {
    let path = "data/day4.txt";
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    let mut sum = 0;
    for line in contents.lines() {
        let (_, cards) = line.split_once(": ").unwrap();
        let (winning, hand) = cards.split_once(" | ").unwrap();
        let winning = winning
            .split(" ")
            .filter(|win| !win.is_empty())
            .collect::<Vec<_>>();
        let hand = hand
            .split(" ")
            .filter(|win| !win.is_empty())
            .collect::<Vec<_>>();

        let mut score: Option<i32> = None;
        for h in hand {
            if winning.contains(&h) {
                score = Some(score.map(|s| s * 2).unwrap_or(1));
                // if let Some(ref mut s) = score {
                //     *s = *s * 2;
                // } else {
                //     score = Some(1);
                // }
            }
        }
        if let Some(score) = score {
            sum += score;
        }
    }
    println!("{:?}", sum);
}

pub fn part2() {
    let path = "data/day4.txt";
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    let mut multiplier = vec![1; contents.lines().count()];
    for (index, line) in contents.lines().enumerate() {
        let (_, cards) = line.split_once(": ").unwrap();
        let (winning, hand) = cards.split_once(" | ").unwrap();
        let winning = winning
            .split(" ")
            .filter(|win| !win.is_empty())
            .collect::<HashSet<_>>();
        let hand = hand
            .split(" ")
            .filter(|win| !win.is_empty())
            .collect::<HashSet<_>>();

        let matches = winning.intersection(&hand).count();
        for i in index..index + matches {
            multiplier[i + 1] += multiplier[index];
        }
    }
    println!("{:?}", multiplier.into_iter().sum::<usize>());
}
