use itertools::Itertools;

fn erase(first: &char, second: &char) -> bool {
    let a = *first as i16;
    let b = *second as i16;
    a.abs_diff(b) == 32
}

fn compact(input: &mut Vec<char>) -> usize {
    let mut previous_size = input.len();
    loop {
        for (idx, (a, b)) in input.iter().tuple_windows().enumerate() {
            if erase(a, b) {
                input.remove(idx);
                input.remove(idx);
                break;
            }
        }
        if input.len() == previous_size {
            break;
        }
        previous_size = input.len();
    }
    previous_size
}

fn part1(input: &str) -> usize {
    let mut chars = input
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<Vec<char>>();
    compact(&mut chars)
}

fn part2(input: &str) -> usize {
    let process_one = |char_to_remove: char| -> usize {
        let mut chars = input
            .chars()
            .filter(|&c| {
                c.is_ascii_alphabetic()
                    && c != char_to_remove
                    && c != char_to_remove.to_uppercase().next().unwrap()
            })
            .collect::<Vec<char>>();
        compact(&mut chars)
    };
    if let Some(r) = ('a'..='z').map(process_one).min() {
        return r;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_small() {
        let input = "Aa";
        assert_eq!(0, part1(input));
        let input = "dabAcCaCBAcCcaDA";
        assert_eq!(10, part1(input));
    }

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("input_day5.txt").unwrap();
        assert_eq!(11546, part1(&input));
    }

    #[test]
    fn test_part2_small() {
        let input = "dabAcCaCBAcCcaDA";
        assert_eq!(4, part2(input));
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("input_day5.txt").unwrap();
        assert_eq!(5124, part2(&input));
    }
}
