use std::collections::HashSet;
use itertools::Itertools;

// ------- DAY 1
pub fn day1_part1(sequence: impl Iterator<Item = i32>) -> i32 {
    sequence.sum()
}

struct Device {
    frequency: i32,
    encountered: HashSet<i32>,
}

impl Device {
    pub fn new() -> Self {
        let mut encountered: HashSet<i32> = HashSet::new();
        encountered.insert(0);
        Self {
            frequency: 0,
            encountered,
        }
    }

    pub fn calibrate<'a>(&mut self, sequence: impl Iterator<Item = &'a i32>) -> Option<i32> {
        for item in sequence {
            self.frequency += item;
            if !self.encountered.insert(self.frequency) {
                return Some(self.frequency);
            }
        }
        None
    }
}

pub fn day1_part2(sequence: impl Iterator<Item = i32>) -> i32 {
    let mut device = Device::new();
    let sequence = sequence.collect::<Vec<i32>>();
    loop {
        if let Some(f) = device.calibrate(sequence.iter()) {
            return f;
        }
    }
}
// ------- DAY 2

fn process_line(line: &str) -> (bool, bool) {
    let mut result = (false, false);
    const NUM_CHARACTERS: u8 = 26;
    let mut character_count: Vec<u8> = vec![0; NUM_CHARACTERS as usize];
    for c in line.to_lowercase().chars() {
        if c.is_ascii_lowercase() {
            let c = c as u8 % NUM_CHARACTERS;
            character_count[c as usize] += 1;
        }
    }
    let mut any_two = character_count.iter().filter(|x| **x == 2);
    if any_two.next().is_some() {
        result.0 = true;
    }
    let mut any_three = character_count.iter().filter(|x| **x == 3);
    if any_three.next().is_some() {
        result.1 = true;
    }
    result
}

pub fn day2_part1(lines: impl Iterator<Item = String>) -> i32 {
    let mut num_two: i32 = 0;
    let mut num_three: i32 = 0;
    for line in lines {
        let (has_two, has_three) = process_line(&line);
        num_two += has_two as i32;
        num_three += has_three as i32;
    }
    num_two * num_three
}

pub fn day2_part2(lines: impl Iterator<Item = String>) -> Option<String> {
    for pair in lines.combinations(2) {
        let probe = &pair[0];
        let candidate = &pair[1];
        let result = probe.chars().zip(candidate.chars()).filter(|(a, b)| a == b).map(|(a, _)| a).collect::<String>();
        if result.len() == probe.len() - 1 {
            return Some(result)
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use super::*;

    #[test]
    fn part1_small() {
        let examples = vec![
            (vec![1, 1, 1], 3),
            (vec![1, 1, -2], 0),
            (vec![-1, -2, -3], -6),
        ];
        for example in examples {
            assert_eq!(day1_part1(example.0.iter().cloned()), example.1);
        }
    }

    #[test]
    fn part1_actual() {
        let reader = BufReader::new(File::open("input_day1.txt").unwrap());
        let result = day1_part1(
            reader
                .lines()
                .map(|l| l.unwrap())
                .map(|l| l.parse::<i32>().unwrap()),
        );
        assert_eq!(result, 522);
    }

    #[test]
    fn part2_small() {
        let examples = vec![
            (vec![1, -1], 0),
            (vec![-6, 3, 8, 5, -6], 5),
            (vec![3, 3, 4, -2, -4], 10),
            (vec![7, 7, -2, -7, -4], 14),
            (vec![1, -2, 3, 1, 1, -2], 2),
        ];
        for example in examples {
            assert_eq!(day1_part2(example.0.iter().cloned()), example.1);
        }
    }

    #[test]
    fn part2_actual() {
        let reader = BufReader::new(File::open("input_day1.txt").unwrap());
        let sequence = reader
            .lines()
            .map(|l| l.unwrap())
            .map(|l| l.parse::<i32>().unwrap());
        assert_eq!(day1_part2(sequence), 73364);
    }

    #[test]
    fn day2_part1_small() {
        let input = vec![
            "abcdef".to_owned(),
            "bababc".to_owned(),
            "abbcde".to_owned(),
            "abcccd".to_owned(),
            "aabcdd".to_owned(),
            "abcdee".to_owned(),
            "ababab".to_owned(),
        ];
        assert_eq!(day2_part1(input.iter().cloned()), 12);
    }

    #[test]
    fn day2_part1_actual() {
        let input = BufReader::new(File::open("input_day2.txt").unwrap())
            .lines()
            .map(|l| l.unwrap());
        assert_eq!(day2_part1(input), 4693);
    }

    #[test]
    fn day2_part2_small() {
        let input = vec![
            "abcde".to_owned(),
            "fghij".to_owned(),
            "klmno".to_owned(),
            "pqrst".to_owned(),
            "fguij".to_owned(),
            "axcye".to_owned(),
            "wvxyz".to_owned(),
        ];
        assert_eq!(day2_part2(input.iter().cloned()).unwrap(), "fgij");
    }

    #[test]
    fn day2_part2_actual() {
        let input = BufReader::new(File::open("input_day2.txt").unwrap())
            .lines()
            .map(|l| l.unwrap());
        assert_eq!(day2_part2(input).unwrap(), "pebjqsalrdnckzfihvtxysomg");
    }
}
