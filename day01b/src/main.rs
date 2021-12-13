use std::fs;

fn count_window_increases(values: &[u32], size: usize) -> u32 {
    let mut increases = 0;
    let mut prev_value = u32::MAX;

    for window in values.windows(size) {
        let value: u32 = window.iter().sum();
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
    dbg!(count_window_increases(&depths, 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_window_increases() {
        assert_eq!(count_window_increases(&[], 2), 0);
        assert_eq!(count_window_increases(&[1, 1, 1, 1], 2), 0);
        assert_eq!(count_window_increases(&[1, 1, 2, 2], 2), 2);
    }

    #[test]
    fn test_example() {
        let input = fs::read_to_string("input-test").unwrap();
        let depths = parse_depths(&input);
        assert_eq!(count_window_increases(&depths, 3), 5);
    }
}
