use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::ops::Add;
use std::str::FromStr;

fn add(a: usize, b: isize) -> usize {
    if b.is_negative() {
        a - b.wrapping_abs() as usize
    } else {
        a + b as usize
    }
}

type EnergyLevel = u8;

const MAX_ENERGY_LEVEL: EnergyLevel = 10;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Vec2 {
    x: isize,
    y: isize,
}

const DIRECTIONS: [Vec2; 8] = [
    Vec2 { x: 0, y: -1 },  // north
    Vec2 { x: 1, y: -1 },  // northeast
    Vec2 { x: 1, y: 0 },   // east
    Vec2 { x: 1, y: 1 },   // southeast
    Vec2 { x: 0, y: 1 },   // south
    Vec2 { x: -1, y: 1 },  // southwest
    Vec2 { x: -1, y: 0 },  // west
    Vec2 { x: -1, y: -1 }, // northwest
];

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    row: usize,
    column: usize,
}

impl Add<Vec2> for Location {
    type Output = Self;

    fn add(self, vec2: Vec2) -> Self::Output {
        Self {
            row: add(self.row, vec2.y),
            column: add(self.column, vec2.x),
        }
    }
}

struct IterLocations {
    height: usize,
    width: usize,
    row: usize,
    column: usize,
}

impl Iterator for IterLocations {
    type Item = Location;

    fn next(&mut self) -> Option<Location> {
        if self.row == self.height {
            return None;
        }

        let location = Location {
            row: self.row,
            column: self.column,
        };

        self.column += 1;

        if self.column == self.width {
            self.row += 1;
            self.column = 0;
        }

        Some(location)
    }
}

#[derive(Debug, Default)]
struct OctopusGrid {
    height: usize,
    width: usize,
    energy_levels: Vec<EnergyLevel>,
}

impl OctopusGrid {
    fn locations(&self) -> IterLocations {
        IterLocations {
            height: self.height,
            width: self.width,
            row: 0,
            column: 0,
        }
    }

    fn contains(&self, location: &Location) -> bool {
        location.row < self.height && location.column < self.width
    }

    fn get_index(&self, location: &Location) -> Option<usize> {
        if self.contains(location) {
            Some(location.row * self.width + location.column)
        } else {
            None
        }
    }

    fn get_energy_level(&self, location: &Location) -> Option<EnergyLevel> {
        if self.contains(location) {
            Some(self.energy_levels[self.get_index(location).unwrap()])
        } else {
            None
        }
    }

    fn increase_energy_level(&mut self, location: &Location) {
        if self.contains(location) {
            let index = self.get_index(location).unwrap();
            self.energy_levels[index] += 1;
        }
    }

    fn reset_energy_level(&mut self, location: &Location) {
        if self.contains(location) {
            let index = self.get_index(location).unwrap();
            self.energy_levels[index] = 0;
        }
    }

    fn adjacent_locations(&self, location: &Location) -> Vec<Location> {
        let mut locations: Vec<Location> = Default::default();

        if self.contains(location) {
            for direction in DIRECTIONS {
                if (direction.y.is_negative() && location.row == 0)
                    || (direction.y.is_positive() && location.row == self.height - 1)
                    || (direction.x.is_negative() && location.column == 0)
                    || (direction.x.is_positive() && location.column == self.width - 1)
                {
                    continue;
                }
                locations.push(*location + direction);
            }
        }

        locations
    }

    fn step(&mut self) -> HashSet<Location> {
        let mut flashes: HashSet<Location> = Default::default();

        fn flash(
            octopus_grid: &mut OctopusGrid,
            flashes: &mut HashSet<Location>,
            location: &Location,
        ) {
            if !flashes.contains(location)
                && octopus_grid.get_energy_level(location).unwrap() >= MAX_ENERGY_LEVEL
            {
                flashes.insert(*location);
                for adj_location in octopus_grid.adjacent_locations(location) {
                    octopus_grid.increase_energy_level(&adj_location);
                    flash(octopus_grid, flashes, &adj_location);
                }
            }
        }

        for location in self.locations() {
            self.increase_energy_level(&location);
        }

        for location in self.locations() {
            flash(self, &mut flashes, &location);
        }

        for location in flashes.iter() {
            self.reset_energy_level(location);
        }

        flashes
    }
}

impl FromStr for OctopusGrid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let height = lines.len();
        let width = lines[0].len();

        let energy_levels: Vec<EnergyLevel> = lines
            .iter()
            .map(|line| line.trim().chars().map(|c| c.to_digit(10).unwrap() as u8))
            .flatten()
            .collect();

        Ok(Self {
            height,
            width,
            energy_levels,
        })
    }
}

impl fmt::Display for OctopusGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for location in self.locations() {
            write!(f, "{}", self.get_energy_level(&location).unwrap())?;
            if location.column == self.width - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

fn flash_count_after_n_steps(octopus_grid: &mut OctopusGrid, n: usize) -> usize {
    let mut flash_count = 0;

    for _ in 0..n {
        flash_count += octopus_grid.step().len();
    }

    flash_count
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut octopus_grid = OctopusGrid::from_str(&input).unwrap();
    dbg!(flash_count_after_n_steps(&mut octopus_grid, 100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_locations() {
        let mut iter = IterLocations {
            height: 0,
            width: 0,
            row: 0,
            column: 0,
        };

        assert_eq!(iter.next(), None);

        let mut iter = IterLocations {
            height: 1,
            width: 1,
            row: 0,
            column: 0,
        };

        assert_eq!(iter.next(), Some(Location { row: 0, column: 0 }));
        assert_eq!(iter.next(), None);

        let mut iter = IterLocations {
            height: 2,
            width: 3,
            row: 0,
            column: 0,
        };

        assert_eq!(iter.next(), Some(Location { row: 0, column: 0 }));
        assert_eq!(iter.next(), Some(Location { row: 0, column: 1 }));
        assert_eq!(iter.next(), Some(Location { row: 0, column: 2 }));
        assert_eq!(iter.next(), Some(Location { row: 1, column: 0 }));
        assert_eq!(iter.next(), Some(Location { row: 1, column: 1 }));
        assert_eq!(iter.next(), Some(Location { row: 1, column: 2 }));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_location_add() {
        assert_eq!(
            Location { row: 0, column: 0 } + Vec2 { x: 1, y: 2 },
            Location { row: 2, column: 1 }
        );
        assert_eq!(
            Location { row: 2, column: 3 } + Vec2 { x: -1, y: -2 },
            Location { row: 0, column: 2 }
        );
    }

    fn generate_octopus_grid(height: usize, width: usize) -> OctopusGrid {
        OctopusGrid {
            height,
            width,
            energy_levels: vec![0; height * width],
        }
    }

    #[test]
    fn test_octopus_grid_contains() {
        let octopus_grid = generate_octopus_grid(20, 10);
        assert!(octopus_grid.contains(&Location { row: 0, column: 0 }));
        assert!(octopus_grid.contains(&Location { row: 5, column: 5 }));
        assert!(octopus_grid.contains(&Location { row: 19, column: 9 }));
        assert!(!octopus_grid.contains(&Location {
            row: 20,
            column: 10
        }));
    }

    #[test]
    fn test_octopus_grid_get_index() {
        let octopus_grid = generate_octopus_grid(20, 10);
        assert_eq!(
            octopus_grid.get_index(&Location { row: 0, column: 0 }),
            Some(0)
        );
        assert_eq!(
            octopus_grid.get_index(&Location { row: 2, column: 5 }),
            Some(25)
        );
        assert_eq!(
            octopus_grid.get_index(&Location {
                row: 20,
                column: 10
            }),
            None
        );
    }

    #[test]
    fn test_octopus_grid_adjacent_locations() {
        let octopus_grid = generate_octopus_grid(20, 10);
        // top-left corner
        assert_eq!(
            octopus_grid.adjacent_locations(&Location { row: 0, column: 0 }),
            vec![
                Location { row: 0, column: 1 },
                Location { row: 1, column: 1 },
                Location { row: 1, column: 0 },
            ]
        );
        // top-middle edge
        assert_eq!(
            octopus_grid.adjacent_locations(&Location { row: 0, column: 1 }),
            vec![
                Location { row: 0, column: 2 },
                Location { row: 1, column: 2 },
                Location { row: 1, column: 1 },
                Location { row: 1, column: 0 },
                Location { row: 0, column: 0 },
            ]
        );
        // top-right corner
        assert_eq!(
            octopus_grid.adjacent_locations(&Location { row: 0, column: 9 }),
            vec![
                Location { row: 1, column: 9 },
                Location { row: 1, column: 8 },
                Location { row: 0, column: 8 },
            ]
        );
        // middle-right edge
        assert_eq!(
            octopus_grid.adjacent_locations(&Location { row: 0, column: 9 }),
            vec![
                Location { row: 1, column: 9 },
                Location { row: 1, column: 8 },
                Location { row: 0, column: 8 },
            ]
        );
        // bottom-right corner
        assert_eq!(
            octopus_grid.adjacent_locations(&Location { row: 10, column: 9 }),
            vec![
                Location { row: 9, column: 9 },
                Location { row: 11, column: 9 },
                Location { row: 11, column: 8 },
                Location { row: 10, column: 8 },
                Location { row: 9, column: 8 },
            ]
        );
        // bottom-middle edge
        assert_eq!(
            octopus_grid.adjacent_locations(&Location { row: 19, column: 5 }),
            vec![
                Location { row: 18, column: 5 },
                Location { row: 18, column: 6 },
                Location { row: 19, column: 6 },
                Location { row: 19, column: 4 },
                Location { row: 18, column: 4 },
            ]
        );
        // bottom-left corner
        assert_eq!(
            octopus_grid.adjacent_locations(&Location { row: 19, column: 0 }),
            vec![
                Location { row: 18, column: 0 },
                Location { row: 18, column: 1 },
                Location { row: 19, column: 1 },
            ]
        );
        // middle-left edge
        assert_eq!(
            octopus_grid.adjacent_locations(&Location { row: 10, column: 0 }),
            vec![
                Location { row: 9, column: 0 },
                Location { row: 9, column: 1 },
                Location { row: 10, column: 1 },
                Location { row: 11, column: 1 },
                Location { row: 11, column: 0 },
            ]
        );
        // inner location
        assert_eq!(
            octopus_grid.adjacent_locations(&Location { row: 10, column: 5 }),
            vec![
                Location { row: 9, column: 5 },
                Location { row: 9, column: 6 },
                Location { row: 10, column: 6 },
                Location { row: 11, column: 6 },
                Location { row: 11, column: 5 },
                Location { row: 11, column: 4 },
                Location { row: 10, column: 4 },
                Location { row: 9, column: 4 },
            ]
        );
    }

    #[test]
    fn test_octopus_grid_step() {
        let mut octopus_grid = OctopusGrid::from_str("11111\n19991\n19191\n19991\n11111").unwrap();
        let flashes = octopus_grid.step();
        assert_eq!(
            format!("{}", &octopus_grid),
            "34543\n40004\n50005\n40004\n34543\n"
        );
        assert!(flashes.contains(&Location { row: 1, column: 1 }));
        assert!(flashes.contains(&Location { row: 1, column: 2 }));
        assert!(flashes.contains(&Location { row: 1, column: 3 }));
        assert!(flashes.contains(&Location { row: 2, column: 1 }));
        assert!(flashes.contains(&Location { row: 2, column: 2 }));
        assert!(flashes.contains(&Location { row: 2, column: 3 }));
        assert!(flashes.contains(&Location { row: 3, column: 1 }));
        assert!(flashes.contains(&Location { row: 3, column: 2 }));
        assert!(flashes.contains(&Location { row: 3, column: 3 }));

        let flashes = octopus_grid.step();
        assert_eq!(
            format!("{}", &octopus_grid),
            "45654\n51115\n61116\n51115\n45654\n"
        );
        assert!(flashes.is_empty());
    }

    #[test]
    fn test_example() {
        let input = fs::read_to_string("input-test").unwrap();
        let mut octopus_grid = OctopusGrid::from_str(&input).unwrap();
        assert_eq!(flash_count_after_n_steps(&mut octopus_grid, 100), 1656);
    }
}
