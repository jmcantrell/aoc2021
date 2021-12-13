use std::fs;

fn count_increases(values: &[u32]) -> u32 {
    let mut increases = 0;
    let mut prev_value = u32::MAX;

    for &value in values {
        if value > prev_value {
            increases += 1;
        }
        prev_value = value;
    }

    increases
}

fn parse_depths(s: &str) -> Vec<u32> {
    s.lines().map(|line| line.parse().unwrap()).collect()
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let depths = parse_depths(&input);
    dbg!(count_increases(&depths));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_increases() {
        assert_eq!(count_increases(&[]), 0);
        assert_eq!(count_increases(&[0, 1, 2]), 2);
        assert_eq!(count_increases(&[0, 0, 0]), 0);
        assert_eq!(count_increases(&[2, 1, 0]), 0);
    }

    #[test]
    fn test_example() {
        let input = fs::read_to_string("input-test").unwrap();
        let depths = parse_depths(&input);
        assert_eq!(count_increases(&depths), 7);
    }
}
