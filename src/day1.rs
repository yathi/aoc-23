use std::fs;

pub fn part2() {
    // let path = "data/day1-2-sample.txt";
    let path = "data/day1-1.txt";
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let mut sum = 0;
    let nums = ["one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7", "eight", "8", "nine", "9"];
    for line in contents.lines() {
        'out: for i in 0..line.len() {
            for (index, num) in nums.iter().enumerate() {
                if i + num.len() > line.len() {
                    continue;
                }
                if &line[i..i+num.len()] == *num {
                    let first = 1+index/2;
                    sum += first*10;
                    break 'out;
                }
            }
        }
        'out: for i in (0..line.len()).rev() {
            for (index, num) in nums.iter().enumerate() {
                if i + num.len() > line.len() {
                    continue;
                }
                if &line[i..i+num.len()] == *num {
                    let last = 1+index/2;
                    sum += last;
                    break 'out;
                }
            }
        }
    //   println!("{:?}", line);
    }
    println!("{:?}", sum);
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