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
    aim: usize,
}

impl Position {
    fn new() -> Self {
        Self {
            distance: 0,
            depth: 0,
            aim: 0,
        }
    }
}

impl Add<Movement> for Position {
    type Output = Self;

    fn add(self, movement: Movement) -> Self {
        match movement.direction {
            Direction::Forward => Self {
                distance: self.distance + movement.magnitude,
                depth: self.depth + self.aim * movement.magnitude,
                aim: self.aim,
            },
            Direction::Down => Self {
                distance: self.distance,
                depth: self.depth,
                aim: self.aim + movement.magnitude,
            },
            Direction::Up => Self {
                distance: self.distance,
                depth: self.depth,
                aim: self.aim - movement.magnitude,
            },
        }
    }
}

impl AddAssign<Movement> for Position {
    fn add_assign(&mut self, movement: Movement) {
        *self = *self + movement
    }
}

peg::parser! {
    grammar command_parser() for str {
        pub rule parse() -> Vec<Command>
            = c:command() ** "\n" "\n"? { c }

        rule command() -> Command
            = movement()

        rule movement() -> Command
            = d:direction() " " m:magnitude() { Command::Move(Movement{ direction: d, magnitude: m }) }

        rule direction() -> Direction
            = forward() / up() / down()

        rule forward() -> Direction
            = "forward" { Direction::Forward }

        rule down() -> Direction
            = "down" { Direction::Down }

        rule up() -> Direction
            = "up" { Direction::Up }

        rule magnitude() -> usize
            = s:$(['0'..='9']+) { s.parse().unwrap() }
    }
}

fn run(position: &mut Position, commands: &[Command]) {
    for command in commands {
        match command {
            Command::Move(movement) => {
                *position += *movement;
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let commands = command_parser::parse(&input).unwrap();
    let mut position = Position::new();
    run(&mut position, &commands);
    dbg!(position);
    dbg!(position.distance * position.depth);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_parser() {
        assert_eq!(command_parser::parse("").unwrap(), &[]);
        assert_eq!(
            command_parser::parse("forward 1\ndown 2\nup 3\n").unwrap(),
            &[
                Command::Move(Movement {
                    direction: Direction::Forward,
                    magnitude: 1
                }),
                Command::Move(Movement {
                    direction: Direction::Down,
                    magnitude: 2
                }),
                Command::Move(Movement {
                    direction: Direction::Up,
                    magnitude: 3
                })
            ]
        );
    }

    #[test]
    fn test_position_add() {
        let position = Position {
            distance: 1,
            depth: 2,
            aim: 3,
        };
        assert_eq!(
            position
                + Movement {
                    direction: Direction::Down,
                    magnitude: 1
                },
            Position {
                distance: 1,
                depth: 2,
                aim: 4
            }
        );
        assert_eq!(
            position
                + Movement {
                    direction: Direction::Up,
                    magnitude: 1
                },
            Position {
                distance: 1,
                depth: 2,
                aim: 2
            }
        );
        assert_eq!(
            position
                + Movement {
                    direction: Direction::Forward,
                    magnitude: 5
                },
            Position {
                distance: 6,
                depth: 17,
                aim: 3
            }
        );
    }
}
