use std::fs;
use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Movement {
    direction: Direction,
    magnitude: usize,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Move(Movement),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    distance: usize,
    depth: usize,
}

impl Position {
    fn new() -> Self {
        Self {
            distance: 0,
            depth: 0,
        }
    }
}

impl Add<Movement> for Position {
    type Output = Self;

    fn add(self, movement: Movement) -> Self {
        match movement.direction {
            Direction::Forward => Self {
                distance: self.distance + movement.magnitude,
                depth: self.depth,
            },
            Direction::Down => Self {
                distance: self.distance,
                depth: self.depth + movement.magnitude,
            },
            Direction::Up => Self {
                distance: self.distance,
                depth: self.depth - movement.magnitude,
            },
        }
    }
}

impl AddAssign<Movement> for Position {
    fn add_assign(&mut self, movement: Movement) {
        *self = *self + movement
    }
}

fn get_position(commands: &[Command]) -> Position {
    let mut position = Position::new();

    for command in commands {
        match command {
            Command::Move(movement) => {
                position += *movement;
            }
        }
    }

    position
}

peg::parser! {
    grammar command_parser() for str {
        pub rule parse() -> Vec<Command>
            = commands:(parse_command() ** "\n") "\n"?{
                commands
            }

        rule parse_command() -> Command
            = parse_movement()

        rule parse_movement() -> Command
            = direction:parse_direction() " " magnitude:parse_uint() {
                Command::Move(Movement{ direction, magnitude })
            }

        rule parse_direction() -> Direction
            = parse_forward() / parse_up() / parse_down()

        rule parse_forward() -> Direction
            = "forward" { Direction::Forward }

        rule parse_down() -> Direction
            = "down" { Direction::Down }

        rule parse_up() -> Direction
            = "up" { Direction::Up }

        rule parse_uint() -> usize
            = s:$(['0'..='9']+) {
                s.parse().unwrap()
            }
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let commands = command_parser::parse(&input).unwrap();
    let position = get_position(&commands);
    dbg!(position.distance * position.depth);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_add() {
        assert_eq!(
            Position {
                distance: 0,
                depth: 1
            } + Movement {
                direction: Direction::Forward,
                magnitude: 5
            },
            Position {
                distance: 5,
                depth: 1,
            }
        );
        assert_eq!(
            Position {
                distance: 1,
                depth: 0
            } + Movement {
                direction: Direction::Down,
                magnitude: 5
            },
            Position {
                distance: 1,
                depth: 5
            }
        );
        assert_eq!(
            Position {
                distance: 1,
                depth: 10
            } + Movement {
                direction: Direction::Up,
                magnitude: 5
            },
            Position {
                distance: 1,
                depth: 5
            }
        );
    }

    #[test]
    fn test_example() {
        let input = fs::read_to_string("input-test").unwrap();
        let commands = command_parser::parse(&input).unwrap();
        let position = get_position(&commands);
        assert_eq!(position.distance * position.depth, 150);
    }
}
