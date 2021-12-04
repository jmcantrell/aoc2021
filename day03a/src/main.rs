use std::fs;

const WIDTH: usize = 12;

fn get_bit(number: isize, i: usize) -> bool {
    let mask = 1 << i;
    number & mask == mask
}

fn set_bit(number: &mut isize, i: usize) {
    *number |= 1 << i;
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let numbers: Vec<isize> = input
        .trim()
        .split_whitespace()
        .map(|s| isize::from_str_radix(s, 2).unwrap())
        .collect();

    let mut balance = [0; WIDTH];

    for number in numbers {
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

    dbg!(gamma * epsilon);
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
}
