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

    fn find_paths(&'a self) -> HashSet<Path> {
        fn explore_path<'a>(
            cave_system: &'a CaveSystem,
            cave: &'a str,
            cave_so_nice: &'a str,
            paths: &mut HashSet<Path<'a>>,
            path: &mut Path<'a>,
            visited: &mut HashMap<&'a str, usize>,
        ) {
            let visits = visited.entry(cave).or_default();
            let is_small = cave.contains(char::is_lowercase);

            if !is_small || *visits == 0 || (cave == cave_so_nice && *visits < 2) {
                if is_small {
                    *visits += 1;
                }

                path.push(cave);

                if cave == "end" {
                    paths.insert(path.clone());
                    return;
                }

                if let Some(adj_caves) = cave_system.connections.get(cave) {
                    for adj_cave in adj_caves.iter() {
                        let mut path = path.clone();
                        let mut visited = visited.clone();
                        explore_path(
                            cave_system,
                            adj_cave,
                            cave_so_nice,
                            paths,
                            &mut path,
                            &mut visited,
                        );
                    }
                }
            }
        }

        let mut paths: HashSet<Path> = Default::default();

        for &cave_so_nice in self.connections.keys() {
            if cave_so_nice != "start"
                && cave_so_nice != "end"
                && !cave_so_nice.contains(char::is_uppercase)
            {
                let mut path: Path = Default::default();
                let mut visited: HashMap<&'a str, usize> = Default::default();
                explore_path(
                    self,
                    "start",
                    cave_so_nice,
                    &mut paths,
                    &mut path,
                    &mut visited,
                );
            }
        }

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

    #[test]
    fn test_small_example() {
        let input = fs::read_to_string("input-test-small").unwrap();
        let output = fs::read_to_string("output-test-small").unwrap();
        let cave_system = cave_system_parser::parse(&input).unwrap();
        let actual_paths = cave_system.find_paths();
        let expected_paths = parse_expected_output(&output);
        assert!(actual_paths.symmetric_difference(&expected_paths).count() == 0);
    }

    #[test]
    fn test_medium_example() {
        let input = fs::read_to_string("input-test-medium").unwrap();
        let cave_system = cave_system_parser::parse(&input).unwrap();
        assert_eq!(cave_system.find_paths().len(), 103);
    }

    #[test]
    fn test_large_example() {
        let input = fs::read_to_string("input-test-large").unwrap();
        let cave_system = cave_system_parser::parse(&input).unwrap();
        assert_eq!(cave_system.find_paths().len(), 3509);
    }
}
