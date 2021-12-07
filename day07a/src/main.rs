use std::fs;

fn get_median<T>(values: &[T]) -> T
where
    T: Ord + Copy,
{
    let mut values: Vec<&T> = values.iter().collect();
    values.sort_unstable();
    *values[values.len() / 2]
}

fn get_fuel_cost(positions: &[isize]) -> isize {
    let median = get_median(positions);
    let mut fuel = 0;

    for &position in positions.iter() {
        fuel += (position - median).abs();
    }

    fuel
}

fn parse_positions(s: &str) -> Vec<isize> {
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
    fn test_example() {
        let input = fs::read_to_string("input-test").unwrap();
        let positions = parse_positions(&input);
        assert_eq!(get_fuel_cost(&positions), 37);
    }
}
