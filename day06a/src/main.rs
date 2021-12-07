use std::fs;

fn simulate_lanternfish(lanternfish: &mut Vec<u8>, days: usize) {
    for _ in 0..days {
        let mut new_fish: usize = 0;

        for fish in lanternfish.iter_mut() {
            if *fish == 0 {
                *fish = 6;
                new_fish += 1;
            } else {
                *fish -= 1;
            }
        }

        for _ in 0..new_fish {
            lanternfish.push(8);
        }
    }
}

fn parse_lanternfish(s: &str) -> Vec<u8> {
    s.trim().split(',').map(|s| s.parse().unwrap()).collect()
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut lanternfish = parse_lanternfish(&input);
    simulate_lanternfish(&mut lanternfish, 80);
    dbg!(lanternfish.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_lanternfish(days: usize, expected_count: usize) {
        let input = fs::read_to_string("input-test").unwrap();
        let mut lanternfish = parse_lanternfish(&input);
        simulate_lanternfish(&mut lanternfish, days);
        assert_eq!(lanternfish.len(), expected_count);
    }

    #[test]
    fn test_example() {
        assert_lanternfish(18, 26);
        assert_lanternfish(80, 5934);
    }
}
