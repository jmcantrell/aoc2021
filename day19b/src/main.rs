mod matrix;
mod scanner;
mod vector;

use crate::matrix::{Matrix, IDENTITY};
use crate::scanner::{Fingerprint, Scanner};
use crate::vector::Vector;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

fn manhattan_distance(a: &Vector, b: &Vector) -> usize {
    (*a - *b).iter().map(|c| c.abs() as usize).sum()
}

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
    // As scanners are confirmed, they will be added this map. As the first scanner is serving as
    // the origin, its position is already known.
    let mut scanner_map: HashSet<Vector> = Default::default();
    scanner_map.insert(Vector([0, 0, 0]));

    // Also, when a scanner is confirmed, add its beacons to this map, to keep track of them all.
    // Since the first scanner is already confirmed, initialize this map with its beacons.
    let mut beacon_map: HashSet<Vector> = scanners[0].iter().cloned().collect();

    // Maintain a set of the indexes of the scanners whose orientations have been confirmed, but
    // still need to be used to check the unknown scanners.
    let mut known: Vec<usize> = vec![0];

    // Maintain a set of the indexes of the scanners whose orientations are unknown. When a
    // scanner is confirmed, it's moved from this set into the known set, to be used for checking
    // other unknown scanners.
    let mut unknown: Vec<usize> = (1..scanners.len()).collect();

    // Keep track of the rotation index used to reorient each scanner as they are resolved.
    let mut orientations: Vec<usize> = std::iter::repeat(0).take(scanners.len()).collect();

    // Cache every rotation for each scanner and its corresponding fingerprint.
    let rotations = generate_rotations();
    let mut scanner_orientations: Vec<Vec<Scanner>> = Default::default();
    let mut fingerprint_orientations: Vec<Vec<Fingerprint>> = Default::default();

    // Also cache a set version of the normalized distances to allow for faster checking of
    // presence of overlapping beacons.
    let mut fingerprints: Vec<HashSet<Vector>> = Default::default();

    for scanner in scanners.iter() {
        let mut rotated_scanners: Vec<Scanner> = Default::default();
        let mut rotated_fingerprints: Vec<Fingerprint> = Default::default();

        for rotation in rotations.iter() {
            let rotated_scanner = scanner.rotate(rotation);
            rotated_fingerprints.push(rotated_scanner.fingerprint());
            rotated_scanners.push(rotated_scanner);
        }

        // The normalized distances are the same for every rotation, so only look at one.
        fingerprints.push(
            rotated_fingerprints
                .first()
                .unwrap()
                .keys()
                .cloned()
                .collect(),
        );

        scanner_orientations.push(rotated_scanners);
        fingerprint_orientations.push(rotated_fingerprints);
    }

    while !unknown.is_empty() {
        // At the end of this loop iteration, the known and unknown sets need to be updated to
        // reflect updates to the scanners' statuses. Since a mutable collection cannot be updated
        // as it's being iterated, they will be updated at the end of the loops based on these
        // temporary collections.

        // This collection holds the scanner indexes of the previously unknown scanners whose
        // orientations have been confirmed and are available to use for checks against the
        // remaining scanners.
        let mut updated_scanners: HashMap<usize, Scanner> = Default::default();

        while !known.is_empty() {
            let i = known.pop().unwrap();

            let known_orientation = orientations[i];
            let known_scanner = &scanner_orientations[i][known_orientation];

            'unknown: for &j in unknown.iter() {
                let common_distances: Vec<Vector> = fingerprints[i]
                    .intersection(&fingerprints[j])
                    .cloned()
                    .collect();

                // I'm not sure if there's a mathematical reason for it, but the puzzle has
                // guaranteed that overlapping scanners have at least 12 common beacons.
                // Considering we're dealing with all possible pairs of points, 12 choose 2 = 66.
                if common_distances.len() < 66 {
                    continue;
                }

                for normalized_distance in common_distances {
                    let known_pairs = fingerprint_orientations[i][known_orientation]
                        .get(&normalized_distance)
                        .unwrap();

                    for k in 0..rotations.len() {
                        let unknown_pairs = fingerprint_orientations[j][k]
                            .get(&normalized_distance)
                            .unwrap();

                        let pairs = known_pairs.iter().cartesian_product(unknown_pairs);

                        for (known_pair, unknown_pair) in pairs {
                            let beacons = known_pair.iter().cartesian_product(unknown_pair);

                            for (known_beacon, unknown_beacon) in beacons {
                                let rotated_scanner = &scanner_orientations[j][k];
                                let scanner_position = *known_beacon - *unknown_beacon;
                                let transformed_scanner =
                                    rotated_scanner.translate(&scanner_position);

                                let overlap = transformed_scanner
                                    .iter()
                                    .filter(|beacon| known_scanner.contains(beacon))
                                    .count();

                                if overlap == 12 {
                                    scanner_map.insert(scanner_position);
                                    for beacon in transformed_scanner.iter() {
                                        beacon_map.insert(*beacon);
                                    }
                                    updated_scanners.insert(j, transformed_scanner);
                                    orientations[j] = k;
                                    continue 'unknown;
                                }
                            }
                        }
                    }
                }
            }

            unknown.retain(|index| !updated_scanners.contains_key(index));
        }

        for (index, scanner) in updated_scanners.into_iter() {
            let orientation = orientations[index];
            fingerprint_orientations[index][orientation] = scanner.fingerprint();
            scanner_orientations[index][orientation] = scanner;
            known.push(index);
        }
    }

    (scanner_map, beacon_map)
}

fn greatest_manhattan_distance(points: &HashSet<Vector>) -> usize {
    points
        .iter()
        .combinations(2)
        .map(|pair| manhattan_distance(pair[0], pair[1]))
        .max()
        .unwrap()
}

fn parse_scanners(s: &str) -> Vec<Scanner> {
    s.split("\n\n")
        .map(|block| block.parse().unwrap())
        .collect()
}

fn main() {
    let s = fs::read_to_string("input").unwrap();
    let scanners = parse_scanners(&s);
    let (scanner_map, _) = assemble_map(&scanners);
    dbg!(greatest_manhattan_distance(&scanner_map));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let s = fs::read_to_string("input-test").unwrap();
        let scanners = parse_scanners(&s);
        let (scanner_map, _) = assemble_map(&scanners);
        assert_eq!(greatest_manhattan_distance(&scanner_map), 3621);
    }
}
