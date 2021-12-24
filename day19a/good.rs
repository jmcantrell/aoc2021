mod matrix;
mod scanner;
mod vector;

use crate::matrix::{Matrix, IDENTITY};
use crate::scanner::{Fingerprint, Scanner};
use crate::vector::Vector;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

fn generate_rotations() -> Vec<Matrix> {
    let mut rotations: Vec<Matrix> = Default::default();

    let mut m = IDENTITY;

    let roll = Matrix([Vector([0, 0, -1]), Vector([0, 1, 0]), Vector([1, 0, 0])]);
    let turn_cw = Matrix([Vector([1, 0, 0]), Vector([0, 0, -1]), Vector([0, 1, 0])]);
    let turn_ccw = Matrix([Vector([1, 0, 0]), Vector([0, 0, 1]), Vector([0, -1, 0])]);

    for i in 0..6 {
        rotations.push(m);

        for _ in 0..3 {
            m *= if i % 2 == 0 { turn_cw } else { turn_ccw };
            rotations.push(m);
        }
        m *= roll;
    }

    rotations
}

fn assemble_map(scanners: &[Scanner]) -> (HashSet<Vector>, HashSet<Vector>) {
    let mut scanners = scanners.to_vec();
    let mut scanner_map: HashSet<Vector> = Default::default();
    let mut beacon_map: HashSet<Vector> = scanners[0].iter().cloned().collect();

    let mut done: HashSet<usize> = Default::default();
    let mut known: HashSet<usize> = Default::default();
    let mut unknown: HashSet<usize> = Default::default();

    known.insert(0);
    unknown.extend(1..scanners.len());
    scanner_map.insert(Vector([0, 0, 0]));

    let rotations = generate_rotations();

    let mut fingerprints: Vec<Fingerprint> = scanners.iter().map(|s| s.fingerprint()).collect();

    let distances: Vec<HashSet<Vector>> = fingerprints
        .iter()
        .map(|fp| fp.keys().cloned().collect())
        .collect();

    while !unknown.is_empty() {
        let mut known_unknown: HashSet<usize> = Default::default();

        for &i in known.iter() {
            let mut update_scanners: HashMap<usize, Scanner> = Default::default();
            let mut update_fingerprints: HashMap<usize, Fingerprint> = Default::default();

            for &j in unknown.iter() {
                let common_distances: Vec<Vector> =
                    distances[i].intersection(&distances[j]).cloned().collect();

                if common_distances.len() < 12 {
                    continue;
                }

                for distance in common_distances {
                    let pairs = fingerprints[i]
                        .get(&distance)
                        .unwrap()
                        .iter()
                        .cartesian_product(fingerprints[j].get(&distance).unwrap());

                    for (known_pair, unknown_pair) in pairs {
                        let beacons = known_pair.iter().cartesian_product(unknown_pair);

                        for (known_beacon, unknown_beacon) in beacons {
                            for rotation in rotations.iter() {
                                let scanner_position = *known_beacon - *unknown_beacon * *rotation;
                                let transformed_scanner =
                                    scanners[j].transform(rotation, &scanner_position);
                                let overlap = transformed_scanner
                                    .iter()
                                    .filter(|beacon| scanners[i].contains(beacon))
                                    .count();

                                if overlap == 12 {
                                    update_fingerprints
                                        .insert(j, transformed_scanner.fingerprint());

                                    scanner_map.insert(scanner_position);

                                    for beacon in transformed_scanner.iter() {
                                        beacon_map.insert(*beacon);
                                    }

                                    update_scanners.insert(j, transformed_scanner);
                                    known_unknown.insert(j);
                                }
                            }
                        }
                    }
                }
            }

            unknown.retain(|index| !known_unknown.contains(index));

            for (index, scanner) in update_scanners.into_iter() {
                scanners[index] = scanner;
            }

            for (index, fingerprint) in update_fingerprints.into_iter() {
                fingerprints[index] = fingerprint;
            }

            done.insert(i);
        }

        for index in known_unknown {
            known.insert(index);
        }
    }

    (scanner_map, beacon_map)
}

fn parse_scanners(s: &str) -> Vec<Scanner> {
    s.split("\n\n")
        .map(|block| block.parse().unwrap())
        .collect()
}

fn main() {
    let s = fs::read_to_string("input").unwrap();
    let scanners = parse_scanners(&s);
    let (_, beacon_map) = assemble_map(&scanners);
    dbg!(beacon_map.len(), 79);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let s = fs::read_to_string("input-test").unwrap();
        let scanners = parse_scanners(&s);
        let (_, beacon_map) = assemble_map(&scanners);
        assert_eq!(beacon_map.len(), 79);
    }
}
