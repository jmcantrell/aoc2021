use std::collections::HashSet;
use std::fs;

fn get_bit(number: isize, i: usize) -> bool {
    let mask = 1 << i;
    number & mask == mask
}

fn get_width(number: isize) -> usize {
    let mut n = number;
    let mut width = 0;

    while n != 0 {
        width += 1;
        n >>= 1;
    }

    width
}

fn get_width_all(numbers: &[isize]) -> usize {
    let mut mask: isize = 0;
    for number in numbers {
        mask |= number;
    }
    get_width(mask)
}

fn get_rating<F>(numbers: &[isize], width: usize, judge: F) -> isize
where
    F: Fn(isize) -> bool,
{
    let mut indexes: HashSet<usize> = (0..numbers.len()).collect();
    for i in (0..width).rev() {
        let mut balance: isize = 0;
        for index in indexes.iter() {
            if get_bit(numbers[*index], i) {
                balance += 1;
            } else {
                balance -= 1;
            }
        }
        let bit = judge(balance);
        indexes.retain(|index| get_bit(numbers[*index], i) == bit);
        if indexes.len() == 1 {
            break;
        }
    }
    let index = indexes.drain().next().unwrap();
    numbers[index]
}

fn get_oxygen_generator_rating(numbers: &[isize], width: usize) -> isize {
    get_rating(numbers, width, |balance| balance >= 0)
}

fn get_co2_scrubber_rating(numbers: &[isize], width: usize) -> isize {
    get_rating(numbers, width, |balance| balance < 0)
}

fn parse_input(s: &str) -> Vec<isize> {
    s.trim()
        .split_whitespace()
        .map(|s| isize::from_str_radix(s, 2).unwrap())
        .collect()
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let numbers = parse_input(&input);
    let width = get_width_all(&numbers);
    let ogr = get_oxygen_generator_rating(&numbers, width);
    let csr = get_co2_scrubber_rating(&numbers, width);

    dbg!(ogr * csr);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

    #[test]
    fn test_get_bit() {
        assert!(!get_bit(0, 10));
        assert!(get_bit(1, 0));
        assert!(get_bit(4, 2));
    }

    #[test]
    fn test_rating() {
        let numbers = parse_input(TEST_INPUT);
        let width = get_width_all(&numbers);
        let ogr = get_oxygen_generator_rating(&numbers, width);
        let csr = get_co2_scrubber_rating(&numbers, width);
        assert_eq!(ogr, 23);
        assert_eq!(csr, 10);
    }
}
