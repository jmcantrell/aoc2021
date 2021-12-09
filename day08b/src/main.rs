use std::collections::{HashMap, HashSet};
use std::fs;

type Digit = u8;
type Pattern = u8;

#[derive(Debug)]
pub struct Entry {
    input: Vec<Pattern>,
    output: Vec<Pattern>,
}

fn create_number(digits: &[Digit]) -> Option<u64> {
    if digits.is_empty() {
        return None;
    }

    let mut number: u64 = 0;

    for &digit in digits {
        number *= 10;
        number += digit as u64;
    }

    Some(number)
}

fn count_pattern_segments(pattern: Pattern) -> usize {
    let mut count = 0;
    let mut n = pattern as usize;

    while n != 0 {
        count += n & 1;
        n >>= 1;
    }

    count
}

fn pattern_difference(patterns: &[Pattern]) -> Pattern {
    if patterns.is_empty() {
        return 0;
    }

    let mut patterns = patterns.iter();
    let mut result = *patterns.next().unwrap();

    for &pattern in patterns {
        result &= !pattern;
    }

    result
}

fn merge_patterns(patterns: &[Pattern]) -> Pattern {
    let mut result = 0;

    for &pattern in patterns {
        result |= pattern;
    }

    result
}

fn deduce_pattern(patterns: &HashSet<Pattern>, size: usize, partial: Pattern) -> Pattern {
    for &pattern in patterns {
        if count_pattern_segments(pattern) == size && pattern_difference(&[partial, pattern]) == 0 {
            return pattern;
        }
    }
    unreachable!()
}

fn deduce_signal_mapping(patterns: &[Pattern]) -> HashMap<Pattern, Digit> {
    let mut known: [Pattern; 10] = Default::default();
    let mut unknown: HashSet<Pattern> = Default::default();

    for &pattern in patterns {
        let digit = match count_pattern_segments(pattern) {
            2 => 1,
            3 => 7,
            4 => 4,
            7 => 8,
            _ => {
                unknown.insert(pattern);
                continue;
            }
        };
        known[digit] = pattern;
    }

    known[9] = deduce_pattern(&unknown, 6, merge_patterns(&[known[4], known[7]]));
    unknown.remove(&known[9]);

    known[3] = deduce_pattern(&unknown, 5, known[7]);
    unknown.remove(&known[3]);

    known[6] = deduce_pattern(&unknown, 6, pattern_difference(&[known[8], known[1]]));
    unknown.remove(&known[6]);

    known[0] = deduce_pattern(&unknown, 6, 0);
    unknown.remove(&known[0]);

    known[2] = deduce_pattern(&unknown, 5, pattern_difference(&[known[8], known[9]]));
    unknown.remove(&known[2]);

    known[5] = deduce_pattern(&unknown, 5, pattern_difference(&[known[8], known[2]]));
    unknown.remove(&known[5]);

    known
        .iter()
        .enumerate()
        .map(|(digit, &pattern)| (pattern, digit as Digit))
        .collect()
}

fn translate_signals(mapping: &HashMap<Pattern, Digit>, patterns: &[Pattern]) -> u64 {
    let digits: Vec<Digit> = patterns.iter().map(|p| *mapping.get(p).unwrap()).collect();
    create_number(&digits).unwrap()
}

fn sum_output_values(entries: &[Entry]) -> u64 {
    let mut sum = 0;

    for entry in entries.iter() {
        let mapping = deduce_signal_mapping(&entry.input);
        sum += translate_signals(&mapping, &entry.output);
    }

    sum
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
                let mut pattern = 0;

                for segment in segments {
                    let i = segment as Digit - 97;
                    pattern |= 1 << i;
                }

                pattern
            }

        rule parse_segment() -> char
            = s:$(['a'..='g']) {
                s.chars().next().unwrap()
            }
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let entries = entry_parser::parse(&input).unwrap();
    dbg!(sum_output_values(&entries));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_number() {
        assert_eq!(create_number(&[]), None);
        assert_eq!(create_number(&[1]), Some(1));
        assert_eq!(create_number(&[1, 2, 3]), Some(123));
    }

    #[test]
    fn test_pattern_difference() {
        assert_eq!(pattern_difference(&[]), 0);
        assert_eq!(pattern_difference(&[1]), 1);
        assert_eq!(pattern_difference(&[0b101, 0b010]), 0b101);
        assert_eq!(pattern_difference(&[0b111, 0b010, 0b001]), 0b100);
    }

    #[test]
    fn test_merge_patterns() {
        assert_eq!(merge_patterns(&[]), 0);
        assert_eq!(merge_patterns(&[1]), 1);
        assert_eq!(merge_patterns(&[0b100, 0b010, 0b001]), 0b111);
    }

    #[test]
    fn test_count_pattern_segments() {
        assert_eq!(count_pattern_segments(0), 0);
        assert_eq!(count_pattern_segments(1), 1);
        assert_eq!(count_pattern_segments(0b101), 2);
    }

    #[test]
    fn test_example() {
        let input = fs::read_to_string("input-test").unwrap();
        let entries = entry_parser::parse(&input).unwrap();
        assert_eq!(sum_output_values(&entries), 61229);
    }
}
