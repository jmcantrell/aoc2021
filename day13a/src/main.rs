use std::collections::HashSet;
use std::fs;
use std::ops::Add;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Location {
    row: usize,
    column: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fold {
    Row(usize),
    Column(usize),
}

impl Add<Fold> for Location {
    type Output = Location;

    fn add(self, fold: Fold) -> Self::Output {
        let Location {
            mut row,
            mut column,
        } = self;

        match fold {
            Fold::Row(fold_row) => {
                if row > fold_row {
                    row -= (row - fold_row) * 2;
                }
            }
            Fold::Column(fold_column) => {
                if column > fold_column {
                    column -= (column - fold_column) * 2;
                }
            }
        }

        Location { row, column }
    }
}

#[derive(Debug)]
pub struct Grid {
    dots: HashSet<Location>,
}

impl Add<Fold> for Grid {
    type Output = Grid;

    fn add(self, fold: Fold) -> Self::Output {
        let mut dots: HashSet<Location> = Default::default();

        for location in self.dots {
            dots.insert(location + fold);
        }

        Grid { dots }
    }
}

peg::parser! {
    grammar transparent_paper_parser() for str {
        pub rule parse() -> (Grid, Vec<Fold>)
            = dots:parse_dots() "\n\n" fold_instructions:parse_fold_instructions() "\n"? {
                (Grid {dots}, fold_instructions)
            }

        rule parse_dots() -> HashSet<Location>
            = locations:(parse_location() ** "\n") {
                locations.into_iter().collect()
            }

        rule parse_fold_instructions() -> Vec<Fold>
            = parse_fold_instruction() ** "\n"

        rule parse_fold_instruction() -> Fold
            = "fold along " axis:$([xy]) "=" value:parse_number() {
                match axis {
                    "y" => Fold::Row(value),
                    "x" => Fold::Column(value),
                    _ => unreachable!()
                }
            }

        rule parse_location() -> Location
            = column:parse_number() "," row:parse_number() {
                Location { row, column }
            }

        rule parse_number() -> usize
            = s:$(['0'..='9']+) {
                s.parse().unwrap()
            }
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let (grid, folds) = transparent_paper_parser::parse(&input).unwrap();
    let grid = grid + folds[0];
    dbg!(grid.dots.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_add_fold() {
        assert_eq!(
            Location { row: 0, column: 0 } + Fold::Row(1),
            Location { row: 0, column: 0 }
        );
        assert_eq!(
            Location { row: 14, column: 0 } + Fold::Row(7),
            Location { row: 0, column: 0 }
        );
        assert_eq!(
            Location { row: 0, column: 0 } + Fold::Column(1),
            Location { row: 0, column: 0 }
        );
        assert_eq!(
            Location { row: 0, column: 10 } + Fold::Column(5),
            Location { row: 0, column: 0 }
        );
    }

    #[test]
    fn test_example() {
        let input = fs::read_to_string("input-test").unwrap();
        let (grid, folds) = transparent_paper_parser::parse(&input).unwrap();
        let grid = grid + folds[0];
        assert_eq!(grid.dots.len(), 17);
    }
}
