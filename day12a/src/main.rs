use std::collections::{HashMap, HashSet};
use std::fs;

type Cave<'a> = &'a str;
type Path<'a> = Vec<Cave<'a>>;

#[derive(Debug, Default)]
pub struct CaveSystem<'a> {
    connections: HashMap<Cave<'a>, HashSet<Cave<'a>>>,
}

impl<'a> CaveSystem<'a> {
    fn add_connection(&mut self, a: Cave<'a>, b: Cave<'a>) {
        self.connections.entry(a).or_default().insert(b);
        self.connections.entry(b).or_default().insert(a);
    }

    fn find_paths(&'a self) -> Vec<Path> {
        let mut paths: Vec<(Path<'a>, HashSet<Cave<'a>>)> =
            vec![(vec!["start"], Default::default())];

        let mut completed_paths: Vec<Path<'a>> = Default::default();

        while !paths.is_empty() {
            let (path, visited) = paths.pop().unwrap();
            let cur_cave = *path.last().unwrap();

            if cur_cave == "end" {
                completed_paths.push(path);
                continue;
            }

            for &adj_cave in self.connections.get(cur_cave).unwrap() {
                if adj_cave == "start" {
                    continue;
                }

                let is_small = adj_cave.contains(char::is_lowercase);

                if is_small && visited.contains(adj_cave) {
                    continue;
                }

                let mut visited = visited.clone();
                if is_small {
                    visited.insert(adj_cave);
                }

                let mut path = path.clone();
                path.push(adj_cave);

                paths.push((path, visited));
            }
        }

        completed_paths
    }
}

peg::parser! {
    grammar cave_system_parser() for str {
        pub rule parse() -> CaveSystem<'input>
            = connections:(parse_connection() ** "\n") "\n"? {
                let mut cave_system: CaveSystem = Default::default();

                for [a, b] in connections.into_iter() {
                    cave_system.add_connection(a, b);
                }

                cave_system
            }

        rule parse_connection() -> [Cave<'input>; 2]
            = a:parse_cave() "-" b:parse_cave() {
                [a, b]
            }

        rule parse_cave() -> Cave<'input>
            = s:$(['a'..='z' | 'A'..='Z']+)
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let cave_system = cave_system_parser::parse(&input).unwrap();
    dbg!(cave_system.find_paths().len());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_expected_output(s: &str) -> Vec<Path> {
        s.lines().map(|line| line.split(',').collect()).collect()
    }

    fn same_vec_items<T: Eq + std::hash::Hash>(a: Vec<T>, b: Vec<T>) -> bool {
        let a: HashSet<T> = a.into_iter().collect();
        let b: HashSet<T> = b.into_iter().collect();
        a.symmetric_difference(&b).count() == 0
    }

    #[test]
    fn test_small_example() {
        let input = fs::read_to_string("input-test-small").unwrap();
        let output = fs::read_to_string("output-test-small").unwrap();
        let cave_system = cave_system_parser::parse(&input).unwrap();
        let actual_paths = cave_system.find_paths();
        let expected_paths = parse_expected_output(&output);
        assert!(same_vec_items(actual_paths, expected_paths));
    }

    #[test]
    fn test_medium_example() {
        let input = fs::read_to_string("input-test-medium").unwrap();
        let output = fs::read_to_string("output-test-medium").unwrap();
        let cave_system = cave_system_parser::parse(&input).unwrap();
        let actual_paths = cave_system.find_paths();
        let expected_paths = parse_expected_output(&output);
        assert!(same_vec_items(actual_paths, expected_paths));
    }

    #[test]
    fn test_large_example() {
        let input = fs::read_to_string("input-test-large").unwrap();
        let cave_system = cave_system_parser::parse(&input).unwrap();
        assert_eq!(cave_system.find_paths().len(), 226);
    }
}
