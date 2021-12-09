use std::collections::HashSet;
use std::fs;

type Segment = char;
type Pattern = HashSet<Segment>;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Entry {
    input: Vec<Pattern>,
    output: Vec<Pattern>,
}

fn count_unique_digits(entries: &[Entry]) -> usize {
    entries
        .iter()
        .map(|entry| {
            entry
                .output
                .iter()
                .filter(|digit| {
                    let len = digit.len();
                    len == 2 || len == 3 || len == 4 || len == 7
                })
                .count()
        })
        .sum()
}

peg::parser! {
    pub grammar entry_parser() for str {
        pub rule parse() -> Vec<Entry>
            = entries:parse_entries() "\n" {
                entries
            }

        rule parse_entries() -> Vec<Entry>
            = parse_entry() ** "\n"

        rule parse_entry() -> Entry
            = input:parse_input() " | " output:parse_output() {
                Entry { input, output }
            }

        rule parse_input() -> Vec<Pattern>
            = parse_pattern() **<10> " "

        rule parse_output() -> Vec<Pattern>
            = parse_pattern() **<4> " "

        rule parse_pattern() -> Pattern
            = segments:(parse_segment()*<2,7>) {
                segments.iter().copied().collect()
            }

        rule parse_segment() -> Segment
            = s:$(['a'..='g']) {
                s.chars().next().unwrap()
            }
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let entries = entry_parser::parse(&input).unwrap();
    dbg!(count_unique_digits(&entries));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("input-test").unwrap();
        let entries = entry_parser::parse(&input).unwrap();
        assert_eq!(count_unique_digits(&entries), 26);
    }
}
