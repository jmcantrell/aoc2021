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

fn get_highest_successful_launch(target: &Rectangle) -> isize {
    // If you think about it long enough, you'll realize that the lowest successful launch is a y
    // velocity that is exactly the bottom of the target. It can't be any lower, or there would be
    // no x velocity that could possibly hit the target, because with any choice, the probe would
    // overshoot it. This makes an appropriate lower bound for a brute force solution.

    let velocity = Vec2 {
        x: target.left(),
        y: target.bottom(),
    };
    let launch = LaunchSimulation::new(*target, velocity);
    dbg!(velocity);
    println!("{}", &launch);

    // If you test enough launches, starting at the lower bound above, and increasing steadily,
    // you'll see that there is a point where any more y velocity causes the probe to overshoot the
    // target as it's sinking. This is also happens to be the highest the probe can be launched,
    // therefore it's the solution.

    let velocity = Vec2 { x: 6, y: 9 };
    let launch = LaunchSimulation::new(*target, velocity);
    dbg!(velocity);
    println!("{}", &launch);

    // After plotting many launches, a few facts become enter the realm of intuition, and, looking
    // at the physics rules, becomes obvious and concrete:
    //
    // 1. The y velocity is not dependent on the x velocity, or anything, for that matter. This
    //    means that the solution for part 1 can be determined without the need to find the
    //    velocity required to reach the maximum height.
    // 2. Because the y velocity decreases by a constant amount at every step, the probe intersects
    //    the same y coordinates on the way up as it does on the way down. It's velocity when it
    //    reaches y=0 (the same as its starting point) is the same as it was when it launched,
    //    which means that the next step will be the y velocity plus one.
    // 3. Since the velocity decreases by 1 on each step, this can be represented as a triangular
    //    number, which can be calculated with `n * (n + 1) / 2`.
    // 4. The accelleration of a dropped object in this universe follows the sequence of triangular
    //    numbers: 1, 3, 6, 10, 15...
    //
    // If that's the case, and the highest peak corresponds to the highest y velocity that lands
    // the probe on the target, the highest y velocity will be the one that lands it on the lowest
    // point of the target in a single step after it reaches y=0. I'll define the velocity we're
    // looking for as `v`, and since it launches at that velocity, it will reach y=0 on the way
    // down at the same velocity, putting it at the optimal spot on the next step with a velocity
    // of `v+1`.
    //
    // Since a triangular number is calculated with `n` being the length of a side, we can
    // calculate the distance the probe dropped by calculating the triangular number where `n` is
    // the y distance from the starting point to the bottom of the target. If you offset that
    // distance by how far the bottom of the target is from y=0, you'll get the highest point. This
    // is the same value that you would get if you get the triangular number of `n-1` instead.

    let n = target.bottom().abs() - 1;
    n * (n + 1) / 2
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let target = target_parser::parse(&input).unwrap();
    dbg!(get_highest_successful_launch(&target));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("input-test").unwrap();
        let target = target_parser::parse(&input).unwrap();
        assert_eq!(get_highest_successful_launch(&target), 45);
    }
}
