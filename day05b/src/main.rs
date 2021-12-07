use std::collections::HashMap;
use std::fs;
use std::ops::{Add, AddAssign, Div, Sub};

type Coord = isize;

fn gcf(a: isize, b: isize) -> isize {
    if a == 0 {
        return b;
    }

    if b == 0 {
        return a;
    }

    gcf(b, a % b)
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub struct Vec2 {
    x: Coord,
    y: Coord,
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Div<Coord> for Vec2 {
    type Output = Self;

    fn div(self, divisor: Coord) -> Self::Output {
        Self {
            x: self.x / divisor,
            y: self.y / divisor,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    start: Vec2,
    end: Vec2,
}

impl Line {
    fn slope(&self) -> Vec2 {
        let dir = self.end - self.start;
        let div = gcf(dir.x, dir.y).abs();
        dir / div
    }
}

struct LineIterator {
    cur: Vec2,
    end: Vec2,
    slope: Vec2,
    done: bool,
}

impl LineIterator {
    fn new(line: &Line) -> Self {
        Self {
            cur: line.start,
            end: line.end,
            slope: line.slope(),
            done: false,
        }
    }
}

impl Iterator for LineIterator {
    type Item = Vec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let ret = self.cur;
        self.cur += self.slope;
        self.done = ret == self.end;

        Some(ret)
    }
}

impl Line {
    fn iter(&self) -> LineIterator {
        LineIterator::new(self)
    }
}

fn get_overlapping_points(lines: &[Line]) -> Vec<Vec2> {
    let mut points: HashMap<Vec2, usize> = Default::default();

    for line in lines {
        for point in line.iter() {
            *points.entry(point).or_insert(0) += 1;
        }
    }

    points
        .iter()
        .filter_map(|(point, &hits)| if hits >= 2 { Some(point) } else { None })
        .copied()
        .collect()
}

peg::parser! {
    pub grammar line_parser() for str {
        pub rule parse() -> Vec<Line>
            = lines:parse_lines() "\n" {
                lines
            }

        rule parse_lines() -> Vec<Line>
            = parse_line() ** "\n"

        rule parse_line() -> Line
            = start:parse_point() " -> " end:parse_point() {
                Line {start, end}
            }

        rule parse_point() -> Vec2
            = x:parse_isize() "," y:parse_isize() {
                Vec2 {x, y}
            }

        rule parse_isize() -> isize
            = s:$(['0'..='9']+) {
                s.parse().unwrap()
            }
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let lines = line_parser::parse(&input).unwrap();
    dbg!(get_overlapping_points(&lines).len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcf() {
        assert_eq!(gcf(0, 0), 0);
        assert_eq!(gcf(1, 0), 1);
        assert_eq!(gcf(0, 1), 1);
        assert_eq!(gcf(10, 25), 5);
    }

    #[test]
    fn test_line_slope() {
        assert_eq!(
            Line {
                start: Vec2 { x: 0, y: 0 },
                end: Vec2 { x: 5, y: 0 }
            }
            .slope(),
            Vec2 { x: 1, y: 0 }
        );
        assert_eq!(
            Line {
                start: Vec2 { x: 0, y: 0 },
                end: Vec2 { x: 0, y: 5 }
            }
            .slope(),
            Vec2 { x: 0, y: 1 }
        );
        assert_eq!(
            Line {
                start: Vec2 { x: 5, y: 0 },
                end: Vec2 { x: 0, y: 0 }
            }
            .slope(),
            Vec2 { x: -1, y: 0 }
        );
        assert_eq!(
            Line {
                start: Vec2 { x: 0, y: 5 },
                end: Vec2 { x: 0, y: 0 }
            }
            .slope(),
            Vec2 { x: 0, y: -1 }
        );
        assert_eq!(
            Line {
                start: Vec2 { x: 0, y: 0 },
                end: Vec2 { x: 5, y: 5 }
            }
            .slope(),
            Vec2 { x: 1, y: 1 }
        );
        assert_eq!(
            Line {
                start: Vec2 { x: 5, y: 5 },
                end: Vec2 { x: 0, y: 0 }
            }
            .slope(),
            Vec2 { x: -1, y: -1 }
        );
    }

    #[test]
    fn test_line_iter() {
        assert_eq!(
            Line {
                start: Vec2 { x: 0, y: 0 },
                end: Vec2 { x: 2, y: 2 }
            }
            .iter()
            .collect::<Vec<Vec2>>(),
            vec![
                Vec2 { x: 0, y: 0 },
                Vec2 { x: 1, y: 1 },
                Vec2 { x: 2, y: 2 }
            ]
        );
        assert_eq!(
            Line {
                start: Vec2 { x: 2, y: 2 },
                end: Vec2 { x: 0, y: 0 }
            }
            .iter()
            .collect::<Vec<Vec2>>(),
            vec![
                Vec2 { x: 2, y: 2 },
                Vec2 { x: 1, y: 1 },
                Vec2 { x: 0, y: 0 },
            ]
        );
    }

    #[test]
    #[ignore]
    fn test_example() {
        let input = fs::read_to_string("input-test").unwrap();
        let lines = line_parser::parse(&input).unwrap();
        assert_eq!(get_overlapping_points(&lines).len(), 12);
    }
}
