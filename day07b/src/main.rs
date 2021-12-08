use std::cmp::min;
use std::fs;

fn get_diff(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}

fn get_mean(values: &[usize]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    Some(values.iter().sum::<usize>() as f64 / values.len() as f64)
}

fn get_nth_triangular_number(n: usize) -> usize {
    n * (n + 1) / 2
}

fn get_fuel_cost_for_position(initial_positions: &[usize], target_position: usize) -> usize {
    let mut fuel_cost = 0;

    for &position in initial_positions.iter() {
        fuel_cost += get_nth_triangular_number(get_diff(position, target_position));
    }

    fuel_cost
}

fn get_fuel_cost(positions: &[usize]) -> usize {
    if positions.is_empty() {
        return 0;
    }

    let mean = get_mean(positions).unwrap();

    min(
        get_fuel_cost_for_position(positions, mean.floor() as usize),
        get_fuel_cost_for_position(positions, mean.ceil() as usize),
    )
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
    fn test_get_mean() {
        assert_eq!(get_mean(&[]), None);
        assert_eq!(get_mean(&[1]), Some(1.0));
        assert_eq!(get_mean(&[1, 2]), Some(1.5));
        assert_eq!(get_mean(&[0, 4, 2, 1, 3]), Some(2.0));
    }

    #[test]
    fn test_get_nth_triangle_numbers() {
        assert_eq!(get_nth_triangular_number(0), 0);
        assert_eq!(get_nth_triangular_number(1), 1);
        assert_eq!(get_nth_triangular_number(4), 10);
    }

    #[test]
    fn test_example() {
        let input = fs::read_to_string("input-test").unwrap();
        let positions = parse_positions(&input);
        assert_eq!(get_fuel_cost(&positions), 168);
    }
}
