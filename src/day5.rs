use std::fs;

pub fn part1() {
    // let path = "data/day5-sample.txt";
    let path = "data/day5.txt";
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    if let Some((seeds, maps)) = contents.split("\n\n").collect::<Vec<_>>().split_first() {
        println!("{:?}", seeds);
        let entry_maps = maps.iter().map(|m| Mapping::new(m)).collect::<Vec<_>>();
        // println!("{:?}", entry_maps);
        let (_, seed_line) = seeds.split_once(": ").unwrap();
        let result = seed_line
            .split(" ")
            .map(|seed| seed.parse::<i64>().unwrap())
            .map(|seed| get_final_mapping(seed, &entry_maps))
            .min()
            .unwrap();
        println!("{:?}", result);
    }
}
pub fn part2() {
    // let path = "data/day5-sample.txt";
    let path = "data/day5.txt";
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    if let Some((seeds, maps)) = contents.split("\n\n").collect::<Vec<_>>().split_first() {
        println!("{:?}", seeds);
        let entry_maps = maps.iter().map(|m| Mapping::new(m)).collect::<Vec<_>>();
        // println!("{:?}", entry_maps);
        let (_, seed_line) = seeds.split_once(": ").unwrap();
        let mut curr_min = std::i64::MAX;

        let mut iter_seeds = seed_line.split_whitespace();

        while let Some(start_str) = iter_seeds.next() {
            let start: i64 = start_str.parse().unwrap();

            if let Some(count_str) = iter_seeds.next() {
                let count = count_str.parse::<i64>().unwrap();

                for num in start..(start+count) {
                    let map = get_final_mapping(num, &entry_maps);
                    curr_min = curr_min.min(map);
                }
            }
        }

        println!("{:?}", curr_min);
    }
}

fn get_final_mapping(seed: i64, entry_maps: &Vec<Mapping>) -> i64 {
    entry_maps.iter().fold(seed, |acc, map| map.get_map(acc))
}

#[derive(Debug)]
struct Entry {
    destination_start: i64,
    source_start: i64,
    range: i64,
}

impl Entry {
    // line: "50 98 2"
    fn new(line: &str) -> Self {
        let split_line = line
            .split(" ")
            .map(|f| f.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        Self {
            destination_start: split_line[0],
            source_start: split_line[1],
            range: split_line[2],
        }
    }

    fn is_match(&self, coordinate: i64) -> bool {
        coordinate.ge(&self.source_start) && coordinate.lt(&(self.source_start + self.range))
    }

    fn get_match(&self, coordinate: i64) -> i64 {
        self.destination_start + (coordinate - self.source_start)
    }
}

#[derive(Debug)]
struct Mapping {
    entries: Vec<Entry>,
}

impl Mapping {
    // line: "seed-to-soil map:\n50 98 2\n52 50 48"
    fn new(line: &str) -> Self {
        let lines = line.split("\n").collect::<Vec<_>>()[1..].to_vec();
        Self {
            entries: lines.iter().map(|l| Entry::new(l)).collect::<Vec<_>>(),
        }
    }

    fn get_map(&self, coordinate: i64) -> i64 {
        if let Some(c) = self.check_entries(coordinate) {
            return c;
        }
        coordinate
    }

    fn check_entries(&self, coordinate: i64) -> Option<i64> {
        for entry in &self.entries {
            if entry.is_match(coordinate) {
                return Some(entry.get_match(coordinate));
            }
        }
        None
    }
}
