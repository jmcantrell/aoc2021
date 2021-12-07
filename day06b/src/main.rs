use std::fs;

const LIFESPAN: usize = 9;

fn simulate_lanternfish(lanternfish: &[u8], days: usize) -> usize {
    let mut counts: [usize; LIFESPAN] = Default::default();

    for &fish in lanternfish.iter() {
        counts[fish as usize] += 1;
    }

    for _ in 0..days {
        let new = counts[0];
        for i in 1..LIFESPAN {
            counts[i - 1] = counts[i];
        }
        counts[6] += new;
        counts[8] = new;
    }

    counts.iter().sum()
}

fn parse_lanternfish(s: &str) -> Vec<u8> {
    s.trim().split(',').map(|s| s.parse().unwrap()).collect()
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let lanternfish = parse_lanternfish(&input);
    dbg!(simulate_lanternfish(&lanternfish, 256));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_lanternfish(days: usize, expected_count: usize) {
        let input = fs::read_to_string("input-test").unwrap();
        let lanternfish = parse_lanternfish(&input);
        assert_eq!(simulate_lanternfish(&lanternfish, days), expected_count);
    }

    #[test]
    fn test_example() {
        assert_lanternfish(18, 26);
        assert_lanternfish(80, 5934);
        assert_lanternfish(256, 26984457539);
    }
}
