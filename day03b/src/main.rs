use std::collections::HashSet;
use std::fs;

fn get_bit(number: isize, i: usize) -> bool {
    let mask = 1 << i;
    number & mask == mask
}

fn get_bit_width(number: isize) -> usize {
    if number == 0 {
        return 0;
    }
    let mut n = number;
    let mut index = 0;
    while n != 0 {
        index += 1;
        n >>= 1;
    }
    index
}

fn get_max_bit_width(numbers: &[isize]) -> usize {
    let mut composite = 0;
    for number in numbers {
        composite |= number;
    }
    get_bit_width(composite)
}

fn get_rating<F>(numbers: &[isize], judge: F) -> isize
where
    F: Fn(isize) -> bool,
{
    let width = get_max_bit_width(numbers);

    if width == 0 {
        return 0;
    }

    let mut indexes: HashSet<usize> = (0..numbers.len()).collect();

    for i in (0..width).rev() {
        let mut balance: isize = 0;

        for &index in indexes.iter() {
            if get_bit(numbers[index], i) {
                balance += 1;
            } else {
                balance -= 1;
            }
        }

        indexes.retain(|index| get_bit(numbers[*index], i) == judge(balance));

        if indexes.len() == 1 {
            break;
        }
    }

    let index = indexes.drain().next().unwrap();
    numbers[index]
}

fn get_oxygen_generator_rating(numbers: &[isize]) -> isize {
    get_rating(numbers, |balance| balance >= 0)
}

fn get_co2_scrubber_rating(numbers: &[isize]) -> isize {
    get_rating(numbers, |balance| balance < 0)
}

fn get_life_support_rating(numbers: &[isize]) -> isize {
    get_oxygen_generator_rating(numbers) * get_co2_scrubber_rating(numbers)
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
    dbg!(get_life_support_rating(&numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bit() {
        assert!(!get_bit(0, 10));
        assert!(get_bit(1, 0));
        assert!(get_bit(4, 2));
    }

    #[test]
    fn test_get_bit_width() {
        assert_eq!(get_bit_width(0), 0);
        assert_eq!(get_bit_width(1), 1);
        assert_eq!(get_bit_width(4), 3);
    }

    #[test]
    fn test_get_max_bit_width() {
        assert_eq!(get_max_bit_width(&[0]), 0);
        assert_eq!(get_max_bit_width(&[0,1,2,3,4]), 3);
    }

    #[test]
    fn test_example() {
        let input = fs::read_to_string("input-test").unwrap();
        let numbers = parse_input(&input);
        dbg!(get_life_support_rating(&numbers));
    }
}
