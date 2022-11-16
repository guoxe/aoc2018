use itertools::Itertools;

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

pub fn day2_part2<I: Iterator<Item = String>>(lines: I) -> Option<String> {
    for pair in lines.combinations(2) {
        let probe = &pair[0];
        let candidate = &pair[1];
        let result = probe
            .chars()
            .zip(candidate.chars())
            .filter(|(a, b)| a == b)
            .map(|(a, _)| a)
            .collect::<String>();
        if result.len() == probe.len() - 1 {
            return Some(result);
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
    fn part1_actual() {
        let input = BufReader::new(File::open("input_day2.txt").unwrap())
            .lines()
            .map(|l| l.unwrap());
        assert_eq!(day2_part1(input), 4693);
    }

    #[test]
    fn part2_small() {
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
    fn part2_actual() {
        let input = BufReader::new(File::open("input_day2.txt").unwrap())
            .lines()
            .map(|l| l.unwrap());
        assert_eq!(day2_part2(input).unwrap(), "pebjqsalrdnckzfihvtxysomg");
    }
}
