use std::fs;

fn count_increases(depths: &[u32]) -> u32 {
    let mut increases = 0;
    let mut prev_depth = u32::MAX;

    for &depth in depths {
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
}
