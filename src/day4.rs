use regex::Regex;

struct Guard {
    id: u16,
    // use the indices and then just increment at each location
    // finding the total asleep time then becomes just summing over the array
    // finding the most asleep minute is a matter of finding the maximum element and
    // returning the index of it
    asleep_at: [u8; 60],
}

impl Guard {
    pub fn new(id: u16) -> Self {
        Guard {
            id,
            asleep_at: [0; 60],
        }
    }

    pub fn add_sleep(&mut self, asleep_at: usize, awake_at: usize) {
        for i in asleep_at..awake_at {
            self.asleep_at[i] += 1;
        }
    }

    pub fn total_sleep(self) -> u16 {
        self.asleep_at.iter().map(|&e| e as u16).sum()
    }
}

fn part1(input: impl Iterator<Item = String>) -> u16 {
    let begin_shift = Regex::new(
        r"^\[\d+-\d+-\d+\s\d+:(?P<minutes>\d+)\]\sGuard\s\#(?P<guard_id>\d+)\sbegins\sshift$",
    )
    .expect("Invalid regex");
    let falls_asleep = Regex::new(r"^\[\d+-\d+-\d+\s\d+:(?P<minutes>\d+)\]\sfalls\sasleep$")
        .expect("Invalid regex");
    let wakes_up =
        Regex::new(r"^\[\d+-\d+-\d+\s\d+:(?P<minutes>\d+)\]\swakes\sup$").expect("Invalid regex");

    // Consider throwing in the towel and just using a map...
    let mut guards: Vec<&mut Guard> = Vec::new();
    let mut guard_id: Option<u16> = None;
    let mut asleep_at: Option<usize> = None;
    let mut awake_at: Option<usize> = None;
    for line in input {
        if let Some(begin_caps) = begin_shift.captures(&line) {
            guard_id = Some(
                begin_caps
                    .name("guard_id")
                    .unwrap()
                    .as_str()
                    .parse::<u16>()
                    .unwrap(),
            );
        } else if let Some(asleep_caps) = falls_asleep.captures(&line) {
            if let Ok(minute) = asleep_caps
                .name("minutes")
                .unwrap()
                .as_str()
                .parse::<usize>()
            {
                asleep_at = Some(minute);
            } else {
                continue;
            }
        } else if let Some(awake_caps) = wakes_up.captures(&line) {
            if let Ok(minute) = awake_caps
                .name("minutes")
                .unwrap()
                .as_str()
                .parse::<usize>()
            {
                awake_at = Some(minute);
                if let Some(&mut guard) = guards.iter().filter(|g| g.id == guard_id.unwrap()).next()
                {
                    // process guard here, can assume other fields are set
                    guard.add_sleep(asleep_at.unwrap(), awake_at.unwrap())
                } else {
                    let mut guard = Guard::new(guard_id.unwrap());
                    guard.add_sleep(asleep_at.unwrap(), awake_at.unwrap());
                }
                asleep_at = None;
                awake_at = None;
                guard_id = None;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_small() {
        let input = vec![
            "[1518-11-01 00:00] Guard #10 begins shift".to_owned(),
            "[1518-11-01 00:05] falls asleep".to_owned(),
            "[1518-11-01 00:25] wakes up".to_owned(),
            "[1518-11-01 00:30] falls asleep".to_owned(),
            "[1518-11-01 00:55] wakes up".to_owned(),
            "[1518-11-01 23:58] Guard #99 begins shift".to_owned(),
            "[1518-11-02 00:40] falls asleep".to_owned(),
            "[1518-11-02 00:50] wakes up".to_owned(),
            "[1518-11-03 00:05] Guard #10 begins shift".to_owned(),
            "[1518-11-03 00:24] falls asleep".to_owned(),
            "[1518-11-03 00:29] wakes up".to_owned(),
            "[1518-11-04 00:02] Guard #99 begins shift".to_owned(),
            "[1518-11-04 00:36] falls asleep".to_owned(),
            "[1518-11-04 00:46] wakes up".to_owned(),
            "[1518-11-05 00:03] Guard #99 begins shift".to_owned(),
            "[1518-11-05 00:45] falls asleep".to_owned(),
            "[1518-11-05 00:55] wakes up".to_owned(),
        ];
        assert_eq!(part1(input.iter().cloned()), 1);
    }
}
