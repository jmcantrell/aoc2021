use std::collections::{HashMap, HashSet};
use std::fs;

type Path<'a> = Vec<&'a str>;

#[derive(Debug, Default)]
pub struct CaveSystem<'a> {
    connections: HashMap<&'a str, HashSet<&'a str>>,
}

impl<'a> CaveSystem<'a> {
    fn add_connection(&mut self, a: &'a str, b: &'a str) {
        self.connections.entry(a).or_default().insert(b);
        self.connections.entry(b).or_default().insert(a);
    }

    fn find_paths(&'a self) -> Vec<Path> {
        fn explore_path<'a>(
            cave_system: &'a CaveSystem,
            cave: &'a str,
            paths: &mut Vec<Path<'a>>,
            path: &mut Path<'a>,
            visited: &mut HashSet<&'a str>,
        ) {
            if !visited.contains(cave) {
                if cave.contains(char::is_lowercase) {
                    visited.insert(cave);
                }

                path.push(cave);

                if cave == "end" {
                    paths.push(path.clone());
                    return;
                }

                if let Some(adj_caves) = cave_system.connections.get(cave) {
                    for adj_cave in adj_caves.iter() {
                        let mut path = path.clone();
                        let mut visited = visited.clone();
                        explore_path(cave_system, adj_cave, paths, &mut path, &mut visited);
                    }
                }
            }
        }

        let mut path: Path = Default::default();
        let mut paths: Vec<Path> = Default::default();
        let mut visited: HashSet<&'a str> = Default::default();

        explore_path(self, "start", &mut paths, &mut path, &mut visited);

        paths
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

        rule parse_connection() -> [&'input str; 2]
            = a:parse_cave() "-" b:parse_cave() {
                [a, b]
            }

        rule parse_cave() -> &'input str
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

    fn parse_expected_output(s: &str) -> HashSet<Path> {
        s.trim()
            .split('\n')
            .map(|line| line.split(',').collect())
            .collect()
    }

    fn all_in_set<T: std::hash::Hash + Eq>(a: HashSet<T>, b: Vec<T>) -> bool {
        let b: HashSet<T> = b.into_iter().collect();
        a.symmetric_difference(&b).count() == 0
    }

    #[test]
    fn test_small_example() {
        let input = fs::read_to_string("input-test-small").unwrap();
        let output = fs::read_to_string("output-test-small").unwrap();
        let cave_system = cave_system_parser::parse(&input).unwrap();
        let expected_paths = parse_expected_output(&output);
        assert!(all_in_set(expected_paths, cave_system.find_paths()));
    }

    #[test]
    fn test_medium_example() {
        let input = fs::read_to_string("input-test-medium").unwrap();
        let output = fs::read_to_string("output-test-medium").unwrap();
        let cave_system = cave_system_parser::parse(&input).unwrap();
        let expected_paths = parse_expected_output(&output);
        assert!(all_in_set(expected_paths, cave_system.find_paths()));
    }

    #[test]
    fn test_large_example() {
        let input = fs::read_to_string("input-test-large").unwrap();
        let cave_system = cave_system_parser::parse(&input).unwrap();
        assert_eq!(cave_system.find_paths().len(), 226);
    }
}
