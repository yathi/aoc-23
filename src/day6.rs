use std::fs;

pub fn part1() {
    // let path = "data/day5-sample.txt";
    let path = "data/day5.txt";
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    println!("{:?}", contents);
}
