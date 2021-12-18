use std::fmt;
use std::fs;
use std::ops::AddAssign;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl AddAssign<Self> for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

struct CellIter {
    extents: Rectangle,
    current: Vec2,
}

impl CellIter {
    fn new(extents: Rectangle) -> Self {
        Self {
            extents,
            current: extents.top_left,
        }
    }
}

impl Iterator for CellIter {
    type Item = Vec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.y < self.extents.bottom() {
            return None;
        }

        let current = self.current;

        self.current.x += 1;
        if self.current.x > self.extents.right() {
            self.current.x = 0;
            self.current.y -= 1;
        }

        Some(current)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Rectangle {
    top_left: Vec2,
    bottom_right: Vec2,
}

impl Rectangle {
    fn top(&self) -> isize {
        self.top_left.y
    }

    fn bottom(&self) -> isize {
        self.bottom_right.y
    }

    fn left(&self) -> isize {
        self.top_left.x
    }

    fn right(&self) -> isize {
        self.bottom_right.x
    }

    fn contains(&self, point: &Vec2) -> bool {
        self.top() >= point.y
            && point.y >= self.bottom()
            && self.left() <= point.x
            && point.x <= self.right()
    }

    fn locations(&self) -> CellIter {
        CellIter::new(*self)
    }
}

#[derive(Debug)]
struct Probe {
    position: Vec2,
    velocity: Vec2,
}

impl Probe {
    fn new(velocity: Vec2) -> Self {
        Self {
            position: Default::default(),
            velocity,
        }
    }

    fn step(&mut self) {
        self.position += self.velocity;

        self.velocity.x += match self.velocity.x {
            v if v < 0 => 1,
            v if v > 0 => -1,
            _ => 0,
        };

        self.velocity.y -= 1;
    }
}

struct LaunchSimulation {
    probe: Probe,
    target: Rectangle,
    trajectory: Vec<Vec2>,
}

impl LaunchSimulation {
    fn new(target: Rectangle, velocity: Vec2) -> Self {
        let mut sim = Self {
            target,
            probe: Probe::new(velocity),
            trajectory: Default::default(),
        };

        while !sim.past_target() && !sim.on_target() {
            sim.probe.step();
            sim.trajectory.push(sim.probe.position);
        }

        sim
    }
}

impl LaunchSimulation {
    fn extents(&self) -> Rectangle {
        let mut extents: Rectangle = Default::default();

        let mut points = self.trajectory.clone();
        points.push(self.target.top_left);
        points.push(self.target.bottom_right);

        for point in points {
            if point.x < extents.top_left.x {
                extents.top_left.x = point.x;
            }
            if point.x > extents.bottom_right.x {
                extents.bottom_right.x = point.x;
            }
            if point.y > extents.top_left.y {
                extents.top_left.y = point.y;
            }
            if point.y < extents.bottom_right.y {
                extents.bottom_right.y = point.y;
            }
        }

        extents
    }

    fn on_target(&self) -> bool {
        self.target.contains(&self.probe.position)
    }

    fn past_target(&self) -> bool {
        self.probe.position.x > self.target.right() || self.probe.position.y < self.target.bottom()
    }
}

impl fmt::Display for LaunchSimulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let extents = self.extents();
        let origin: Vec2 = Default::default();

        for location in extents.locations() {
            let c = if location == origin {
                'S'
            } else if self.trajectory.contains(&location) {
                '#'
            } else if self.target.contains(&location) {
                'T'
            } else {
                '.'
            };

            write!(f, "{}", c)?;

            if location.x == extents.right() {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

peg::parser! {
    grammar target_parser() for str {
        pub rule parse() -> Rectangle
            = "target area: x=" xs:parse_range() ", y=" ys:parse_range() "\n"? {
                let [min_x, max_x] = xs;

                // Not sure why the max y value is first, but it seems consistent.
                let [max_y, min_y] = ys;

                let top_left = Vec2 { x: min_x, y: min_y };
                let bottom_right = Vec2 { x: max_x, y: max_y };

                Rectangle { top_left, bottom_right }
            }

        rule parse_range() -> [isize; 2]
            = start:parse_number() ".." end:parse_number() {
                [start, end]
            }

        rule parse_number() -> isize
            = s:$("-"? ['0'..='9']+) {
                s.parse().unwrap()
            }
    }
}

fn get_successful_launches(target: &Rectangle) -> Vec<Vec2> {
    let mut velocities: Vec<Vec2> = Default::default();

    // Building on what we learned from part 1, we just need to do a brute force search with a
    // little bit of optimization on the bounds. We already know the bounds for the y velocity.

    let target_right = target.right();
    let target_bottom = target.bottom();

    for y in target_bottom..target_bottom.abs() {
        // We'll be generous with the x values and start with a minimum of 0, and since any value
        // greater than whatever the right side of the target is will overshoot, we know that's the
        // best upper bound.
        for x in 0..=target_right {
            let velocity = Vec2 { x, y };
            let launch = LaunchSimulation::new(*target, velocity);

            // If, at any point, the probe connected with the target, count it.
            if launch.trajectory.iter().any(|p| target.contains(p)) {
                velocities.push(velocity);
            }
        }
    }

    velocities
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let target = target_parser::parse(&input).unwrap();
    dbg!(get_successful_launches(&target).len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("input-test").unwrap();
        let target = target_parser::parse(&input).unwrap();
        assert_eq!(get_successful_launches(&target).len(), 112);
    }
}
