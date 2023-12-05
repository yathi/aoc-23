use std::fs;

pub fn part1() {
    let path = "data/day2.txt";
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    let mut sum = 0;
    for line in contents.lines() {
        let (game_info, game) = line.split_once(": ").unwrap();
        let (_, id) = game_info.split_once(" ").unwrap();
        if is_possible(game) {
            sum += id.parse::<i32>().unwrap();
        }
    }
    println!("{:?}", sum);
}

fn is_possible(game: &str) -> bool {
    let sets = game.split("; ").collect::<Vec<_>>();
    for set in sets {
        let pulls = set.split(", ").collect::<Vec<_>>();
        // println!("{:?}", pulls);
        for pull in pulls {
            // println!("{:?}", pull);
            let (count, color) = pull.split_once(" ").unwrap();
            let count = count.parse::<i32>().unwrap();
            if !is_valid_cube(color, count) {
                return false;
            }
        }
    }
    true
}

// only 12 red cubes, 13 green cubes, and 14 blue cubes
fn is_valid_cube(color: &str, count: i32) -> bool {
    match color {
        "blue" => count <= 14,
        "red" => count <= 12,
        "green" => count <= 13,
        _ => panic!("Unexpected color"),
    }
}
