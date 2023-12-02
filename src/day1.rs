use std::fs;

pub fn part2() {
    let path = "data/day1-2-sample.txt";
    // let path = "data/day1-2.txt";
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    // let mut sum = 0;
    for line in contents.lines() {
      println!("{:?}", line);
    }
}

pub fn part1() {
    // let path_sample = "data/day1-sample.txt";
    let path_input = "data/day1-1.txt";
    let contents = fs::read_to_string(path_input)
        .expect("Should have been able to read the file");

    let mut sum = 0;
    for line in contents.lines() {
        let nums = line
                                .chars()
                                .filter(|ch| ch.is_ascii_digit())
                                .map(|ch| ch.to_digit(10).unwrap())
                                .collect::<Vec<_>>();
        
        let first = nums.first().unwrap();
        let last = nums.last().unwrap();

        let total = first * 10 + last;
        sum += total
    }
   println!("{:?}", sum);
}