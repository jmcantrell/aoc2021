use crate::matrix::Matrix;
use crate::vector::{Value, Vector};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub type Fingerprint = HashMap<Vector, HashSet<Vec<Vector>>>;

#[derive(Debug, Default, Clone)]
pub struct Scanner(pub HashSet<Vector>);

impl Scanner {
    pub fn iter(&self) -> impl Iterator<Item = &Vector> {
        self.0.iter()
    }

    pub fn contains(&self, beacon: &Vector) -> bool {
        self.0.contains(beacon)
    }

    pub fn rotate(&self, r: &Matrix) -> Self {
        Self(self.0.iter().map(|p| *p * *r).collect())
    }

    pub fn translate(&self, t: &Vector) -> Self {
        Self(self.0.iter().map(|p| *p + *t).collect())
    }

    pub fn fingerprint(&self) -> Fingerprint {
        let mut result: Fingerprint = Default::default();

        for combo in self.0.iter().combinations(2) {
            let distance = *combo[0] - *combo[1];

            let mut components: Vec<Value> = distance.iter().map(|c| c.abs()).collect();
            components.sort_unstable();

            let normalized_distance = Vector::from(components);

            let pair: Vec<_> = combo.into_iter().cloned().collect();
            result.entry(normalized_distance).or_default().insert(pair);
        }

        result
    }
}

impl FromStr for Scanner {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        Ok(scanner_parser::parse(s)?)
    }
}

peg::parser! {
    grammar scanner_parser() for str {
        pub rule parse() -> Scanner
            = "--- scanner " parse_number() " ---\n" beacons:parse_locations() "\n"? {
                Scanner(beacons.into_iter().collect())
            }

        rule parse_locations() -> Vec<Vector>
            = parse_location() ** "\n"

        rule parse_location() -> Vector
            = x:parse_number() "," y:parse_number() "," z:parse_number() {
                Vector([x,y,z])
            }

        rule parse_number() -> isize
            = s:$("-"? ['0'..='9']+) {
                s.parse().unwrap()
            }
    }
}
