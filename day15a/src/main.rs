use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs;
use std::ops::Add;

fn offset(a: usize, b: isize) -> usize {
    if b.is_negative() {
        a - b.abs() as usize
    } else {
        a + b as usize
    }
}

#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    row: usize,
    column: usize,
}

impl Add<Vec2> for Location {
    type Output = Location;

    fn add(self, v: Vec2) -> Self {
        Location {
            row: offset(self.row, v.y),
            column: offset(self.column, v.x),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    location: Location,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

type RiskLevel = u8;

#[derive(Debug)]
struct Cavern {
    start: Location,
    exit: Location,
    risk_levels: HashMap<Location, RiskLevel>,
}

impl Cavern {
    fn adjacent_locations(&self, location: &Location) -> Vec<Location> {
        let mut locations: Vec<Location> = Default::default();

        if location.row > 0 {
            locations.push(Location {
                row: location.row - 1,
                column: location.column,
            });
        }

        if location.column < self.exit.column {
            locations.push(Location {
                row: location.row,
                column: location.column + 1,
            });
        }

        if location.row < self.exit.row {
            locations.push(Location {
                row: location.row + 1,
                column: location.column,
            });
        }

        if location.column > 0 {
            locations.push(Location {
                row: location.row,
                column: location.column - 1,
            });
        }

        locations
    }

    fn calculate_minimum_risk(&self) -> Option<usize> {
        let mut frontier = BinaryHeap::new();

        let mut cost_so_far: HashMap<&Location, usize> = self
            .risk_levels
            .iter()
            .map(|(location, _)| (location, usize::MAX))
            .collect();

        frontier.push(State {
            location: self.start,
            cost: 0,
        });

        cost_so_far.insert(&self.start, 0);

        while let Some(State { location, cost }) = frontier.pop() {
            if location == self.exit {
                return Some(cost);
            }

            if cost > *cost_so_far.get(&location).unwrap() {
                continue;
            }

            for adj_location in self.adjacent_locations(&location) {
                let risk_level = self.risk_levels.get(&adj_location).unwrap();

                let adj_state = State {
                    location: adj_location,
                    cost: cost + *risk_level as usize,
                };

                let adj_cost_so_far = cost_so_far.get_mut(&adj_location).unwrap();

                if adj_state.cost < *adj_cost_so_far {
                    frontier.push(adj_state);
                    *adj_cost_so_far = adj_state.cost;
                }
            }
        }

        None
    }
}

fn parse_cavern(s: &str) -> Cavern {
    let lines: Vec<&str> = s.lines().collect();
    assert!(!lines.is_empty());
    let height = lines.len();
    let width = lines.first().unwrap().len();
    let start = Location { row: 0, column: 0 };
    let exit = Location {
        row: height - 1,
        column: width - 1,
    };
    let risk_levels: HashMap<Location, u8> = lines
        .into_iter()
        .enumerate()
        .map(|(row, line)| {
            line.chars().enumerate().map(move |(column, c)| {
                let risk_level = c.to_digit(10).unwrap() as u8;
                let location = Location { row, column };
                (location, risk_level)
            })
        })
        .flatten()
        .collect();
    Cavern {
        start,
        exit,
        risk_levels,
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let cavern = parse_cavern(&input);
    dbg!(cavern.calculate_minimum_risk().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("input-test").unwrap();
        let cavern = parse_cavern(&input);
        assert_eq!(cavern.calculate_minimum_risk().unwrap(), 40);
    }
}
