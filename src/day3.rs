use std::num::ParseIntError;
use std::str::FromStr;

use regex::Regex;

struct Claim {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

impl Claim {
    fn union(other: Self) -> u16 {
        0
    }
}

enum ClaimErr {
    BadInput(String),
    ParseIntError,
}

impl From<ParseIntError> for ClaimErr {
    fn from(_: ParseIntError) -> Self {
        Self::ParseIntError
    }
}

impl FromStr for Claim {
    type Err = ClaimErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"#(?P<id>\d+)\s@\s(?P<x>\d+),(?P<y>\d+):\s(?P<w>\d+)x(?P<h>\d+)")
            .expect("Invalid regex");
        let caps = re.captures(s).unwrap();
        if caps.name("x").is_none()
            || caps.name("y").is_none()
            || caps.name("w").is_none()
            || caps.name("h").is_none()
        {
            return Err(ClaimErr::BadInput("Missing fields in line".to_owned()));
        }
        Ok(Self {
            x: caps.name("x").unwrap().as_str().parse::<u16>()?,
            y: caps.name("y").unwrap().as_str().parse::<u16>()?,
            w: caps.name("w").unwrap().as_str().parse::<u16>()?,
            h: caps.name("h").unwrap().as_str().parse::<u16>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_part1_small() {
        let input = vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"];
        for l in input {
            if let Ok(claim) = Claim::from_str(l) {
                println!("{}", claim.x);
            }
        }
        assert!(false);
    }
}
