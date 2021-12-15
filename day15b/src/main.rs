use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt;
use std::fs;
use std::ops::{Add, Mul};

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

impl Mul<isize> for Vec2 {
    type Output = Self;

    fn mul(self, m: isize) -> Self::Output {
        Self {
            x: self.x * m,
            y: self.y * m,
        }
    }
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
const MAX_RISK_LEVEL: u8 = 9;

#[derive(Debug)]
struct Cavern {
    start: Location,
    exit: Location,
    risk_levels: HashMap<Location, RiskLevel>,
}

impl fmt::Display for Cavern {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut grid = vec![vec![' '; self.exit.column + 1]; self.exit.row + 1];

        for (location, risk_level) in self.risk_levels.iter() {
            grid[location.row][location.column] = (48 + risk_level) as char;
        }

        for row in grid {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
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

    let size = 5;
    let tile_height = lines.len();
    let tile_width = lines.first().unwrap().len();

    let height = tile_height * size;
    let width = tile_width * size;

    let start = Location { row: 0, column: 0 };
    let exit = Location {
        row: height - 1,
        column: width - 1,
    };

    // Using the tiling rules from the puzzle page, a pattern can be seen where identical tiles lie
    // on diagonals, so we can calculate the first tile and then insert it along the locations that
    // lie on the diagonal.

    // 0 1 2 3 4
    // 1 2 3 4 5
    // 2 3 4 5 6
    // 3 4 5 6 7
    // 4 5 6 7 8

    // This is the number of diagonal line that need to be filled. This can also be thought of as
    // the number of unique tiles.
    let num_diags = size * 2 - 2;

    // I chose to start the diagonal at the top/right of the grid and apply it going down and to
    // the left. This is just an increment that can be added to a location to get it's adjacent
    // diagonal position.
    let diag_step = Vec2 {
        x: -(tile_width as isize),
        y: tile_height as isize,
    };

    let mut risk_levels = HashMap::new();

    for (row, line) in lines.into_iter().enumerate() {
        for (column, c) in line.chars().enumerate() {
            let location = Location { row, column };
            let risk_level = c.to_digit(10).unwrap() as u8;

            // Insert the value for the first tile (the original location from part 1).
            risk_levels.insert(location, risk_level);

            // Calculate the tile for each diagonal line.
            for diag in 1..=num_diags {
                // Keep the risk level between 1 and MAX_RISK_LEVEL, wrapping.
                let diag_risk_level = if risk_level < MAX_RISK_LEVEL {
                    risk_level + 1
                } else {
                    1
                };

                // The number of tiles that lie on this diagonal line.
                let num_diag_steps = if diag < size { diag } else { num_diags - diag } + 1;

                // The starting tile will be along the top for the first half, and along the right
                // for the rest.
                let start_tile_location = if diag < size {
                    Location {
                        row: location.row,
                        column: diag * tile_width + location.column,
                    }
                } else {
                    Location {
                        row: (diag - size + 1) * tile_height + location.row,
                        // The same column as the last diagonal along the top.
                        column: (size - 1) * tile_width + location.column,
                    }
                };

                for i in 0..num_diag_steps {
                    let step = diag_step * i as isize;
                    let tile_location = start_tile_location + step;
                    risk_levels.insert(tile_location, diag_risk_level);
                }
            }
        }
    }

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
        let input_full = fs::read_to_string("input-test-full").unwrap();
        assert_eq!(format!("{}", &cavern), input_full);
        assert_eq!(cavern.calculate_minimum_risk().unwrap(), 315);
    }
}
