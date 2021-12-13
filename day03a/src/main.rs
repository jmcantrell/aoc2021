use std::fs;

fn get_bit(number: isize, i: usize) -> bool {
    let mask = 1 << i;
    number & mask == mask
}

fn set_bit(number: &mut isize, i: usize) {
    *number |= 1 << i;
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

fn get_power_consumption(numbers: &[isize]) -> isize {
    let width = get_max_bit_width(numbers);
    let mut balance = vec![0; width];

    for &number in numbers {
        for (i, b) in balance.iter_mut().enumerate() {
            if get_bit(number, i) {
                *b += 1;
            } else {
                *b -= 1;
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    for (i, b) in balance.iter().enumerate() {
        if *b >= 0 {
            set_bit(&mut gamma, i);
        } else {
            set_bit(&mut epsilon, i);
        }
    }

    gamma * epsilon
}

fn parse_numbers(s: &str) -> Vec<isize> {
    s.lines()
        .map(|s| isize::from_str_radix(s, 2).unwrap())
        .collect()
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let numbers = parse_numbers(&input);
    dbg!(get_power_consumption(&numbers));
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
    fn test_set_bit() {
        let mut number = 0;
        set_bit(&mut number, 2);
        assert_eq!(number, 4);
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
        assert_eq!(get_max_bit_width(&[0, 1, 2, 3, 4]), 3);
    }

    #[test]
    fn test_example() {
        let input = fs::read_to_string("input-test").unwrap();
        let numbers = parse_numbers(&input);
        assert_eq!(get_power_consumption(&numbers), 198);
    }
}
