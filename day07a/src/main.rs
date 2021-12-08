use std::fs;

fn get_diff(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}

fn get_median(values: &[usize]) -> Option<usize> {
    if values.is_empty() {
        return None;
    }

    let mut values: Vec<&usize> = values.iter().collect();
    values.sort_unstable();

    Some(*values[values.len() / 2])
}

fn get_fuel_cost_for_position(initial_positions: &[usize], target_position: usize) -> usize {
    let mut fuel_cost = 0;

    for &position in initial_positions.iter() {
        fuel_cost += get_diff(position, target_position);
    }

    fuel_cost
}

fn get_fuel_cost(positions: &[usize]) -> usize {
    if positions.is_empty() {
        return 0;
    }

    let median = get_median(positions).unwrap();

    get_fuel_cost_for_position(positions, median)
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
    fn test_get_median() {
        assert_eq!(get_median(&[]), None);
        assert_eq!(get_median(&[1]), Some(1));
        assert_eq!(get_median(&[1, 2]), Some(2));
        assert_eq!(get_median(&[0, 4, 2, 1, 3]), Some(2));
    }

    #[test]
    fn test_example() {
        let input = fs::read_to_string("input-test").unwrap();
        let positions = parse_positions(&input);
        assert_eq!(get_fuel_cost(&positions), 37);
    }
}
