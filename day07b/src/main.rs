use std::fs;

fn get_diff(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}

fn get_n_triangle_numbers(n: usize) -> Vec<usize> {
    let mut numbers = vec![0];
    let mut prev = numbers[0];

    for i in 1..=n {
        let number = prev + i;
        prev = number;
        numbers.push(number);
    }

    numbers
}

fn get_fuel_cost(positions: &[usize]) -> usize {
    if positions.is_empty() {
        return 0;
    }

    let min_target = *positions.iter().min().unwrap();
    let max_target = *positions.iter().max().unwrap();
    let mut min_cost = usize::MAX;

    let triangle_numbers = get_n_triangle_numbers(max_target as usize);

    for target in min_target..=max_target {
        let mut target_cost = 0;

        for &position in positions.iter() {
            target_cost += triangle_numbers[get_diff(position, target)];
        }

        if target_cost < min_cost {
            min_cost = target_cost;
        }
    }

    min_cost
}

fn parse_positions(s: &str) -> Vec<usize> {
    s.trim().split(',').map(|s| s.parse().unwrap()).collect()
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let positions = parse_positions(&input);
    dbg!(get_fuel_cost(&positions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_diff() {
        assert_eq!(get_diff(0, 0), 0);
        assert_eq!(get_diff(1, 0), 1);
        assert_eq!(get_diff(1, 1), 0);
        assert_eq!(get_diff(0, 1), 1);
    }

    #[test]
    fn test_get_n_triangle_numbers() {
        assert_eq!(get_n_triangle_numbers(0), vec![0]);
        assert_eq!(get_n_triangle_numbers(1), vec![0, 1]);
        assert_eq!(get_n_triangle_numbers(4), vec![0, 1, 3, 6, 10]);
    }

    #[test]
    fn test_example() {
        let input = fs::read_to_string("input-test").unwrap();
        let positions = parse_positions(&input);
        assert_eq!(get_fuel_cost(&positions), 168);
    }
}
