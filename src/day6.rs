use std::fs;

pub fn part1() {
    let path = "data/day6-s.txt";
    // let path = "data/day6.txt";
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let (times, distances) = contents.split_once("\n").unwrap();
    println!("{:?}", times.split_whitespace().collect::<Vec<_>>());
}
