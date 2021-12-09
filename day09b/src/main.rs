use std::collections::HashSet;
use std::fs;
use std::ops::Add;

type Height = u8;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    row: usize,
    column: usize,
}

impl Add<Direction> for Location {
    type Output = Self;

    fn add(self, direction: Direction) -> Self::Output {
        let mut row = self.row;
        let mut column = self.column;
        match direction {
            Direction::North => row -= 1,
            Direction::East => column += 1,
            Direction::South => row += 1,
            Direction::West => column -= 1,
        };
        Self { row, column }
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

    fn next(&mut self) -> Option<Self::Item> {
        if self.row == self.height {
            return None;
        }

        let location = Location {
            row: self.row,
            column: self.column,
        };

        self.column += 1;

        if self.column == self.width {
            self.column = 0;
            self.row += 1;
        }

        Some(location)
    }
}

#[derive(Debug, Default)]
struct Heightmap {
    height: usize,
    width: usize,
    points: Vec<Height>,
}

impl Heightmap {
    fn iter_locations(&self) -> IterLocations {
        IterLocations {
            height: self.height,
            width: self.width,
            row: 0,
            column: 0,
        }
    }

    fn contains(&self, location: Location) -> bool {
        location.row < self.height && location.column < self.width
    }

    fn adjacent_locations(&self, location: Location) -> Vec<Location> {
        let mut locations: Vec<Location> = Default::default();

        if self.contains(location) {
            if location.row > 0 {
                locations.push(location + Direction::North);
            }
            if location.row < self.height - 1 {
                locations.push(location + Direction::South);
            }
            if location.column > 0 {
                locations.push(location + Direction::West);
            }
            if location.column < self.width - 1 {
                locations.push(location + Direction::East);
            }
        }

        locations
    }

    fn get(&self, location: Location) -> Option<Height> {
        if self.contains(location) {
            Some(self.points[location.row * self.width + location.column])
        } else {
            None
        }
    }

    fn low_points(&self) -> Vec<Location> {
        self.iter_locations()
            .filter(|&location| {
                let height = self.get(location).unwrap();
                self.adjacent_locations(location)
                    .iter()
                    .all(|&adj_location| height < self.get(adj_location).unwrap())
            })
            .collect()
    }

    fn explore_basin(&self, basin: &mut HashSet<Location>, location: Location) {
        basin.insert(location);

        let height = self.get(location).unwrap();

        for adj_location in self.adjacent_locations(location) {
            if basin.contains(&adj_location) {
                continue;
            }

            let adj_height = self.get(adj_location).unwrap();

            if height < adj_height && adj_height != 9 {
                self.explore_basin(basin, adj_location);
            }
        }
    }

    fn basins(&self) -> Vec<Vec<Location>> {
        self.low_points()
            .iter()
            .map(|&location| {
                let mut basin = Default::default();
                self.explore_basin(&mut basin, location);
                basin.into_iter().collect()
            })
            .collect()
    }
}

fn multiply_n_largest_basins(heightmap: &Heightmap, n: usize) -> usize {
    let mut basin_sizes: Vec<usize> = heightmap.basins().iter().map(|basin| basin.len()).collect();
    basin_sizes.sort_unstable();
    basin_sizes.reverse();
    basin_sizes.iter().take(n).product()
}

fn parse_heightmap(s: &str) -> Heightmap {
    let lines: Vec<&str> = s.trim().split('\n').collect();

    assert!(!lines.is_empty());

    let height = lines.len();
    let width = lines[0].len();

    let points = lines
        .into_iter()
        .map(|line| {
            assert_eq!(line.len(), width);
            line.chars().map(|c| c as u8 - 48)
        })
        .flatten()
        .collect();

    Heightmap {
        height,
        width,
        points,
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let heightmap = parse_heightmap(&input);
    dbg!(multiply_n_largest_basins(&heightmap, 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("input-test").unwrap();
        let heightmap = parse_heightmap(&input);
        assert_eq!(multiply_n_largest_basins(&heightmap, 3), 1134);
    }
}
