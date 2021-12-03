use std::fs;

fn count_window_increases(depths: &[u32], size: usize) -> u32 {
    let mut increases = 0;
    let mut prev_depth = u32::MAX;

    for window in depths.windows(size) {
        let depth: u32 = window.iter().sum();
        if depth > prev_depth {
            increases += 1;
        }
        prev_depth = depth;
    }

    increases
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let depths: Vec<u32> = input
        .trim()
        .split_whitespace()
        .map(|line| line.parse().unwrap())
        .collect();

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
}
