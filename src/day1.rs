use std::collections::HashSet;

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
}
