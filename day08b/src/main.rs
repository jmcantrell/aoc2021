use std::collections::HashMap;
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

fn deduce_signal_mapping(patterns: &[Pattern]) -> HashMap<Pattern, Digit> {
    let mut known: [Pattern; 10] = Default::default();
    let mut unknown: Vec<Pattern> = Default::default();

    for &pattern in patterns {
        let digit = match pattern.count_ones() {
            2 => 1,
            3 => 7,
            4 => 4,
            7 => 8,
            _ => {
                unknown.push(pattern);
                continue;
            }
        };
        known[digit] = pattern;
    }

    let mask5 = known[1] ^ known[4];
    let mask6 = known[7] | known[4];

    for pattern in unknown {
        match pattern.count_ones() {
            5 => {
                if (pattern ^ known[8]) & known[1] == 0 {
                    known[3] = pattern;
                } else if pattern & mask5 == mask5 {
                    known[5] = pattern;
                } else {
                    known[2] = pattern;
                }
            }
            6 => {
                if (pattern ^ known[8]) & known[1] != 0 {
                    known[6] = pattern;
                } else if pattern & mask6 == mask6 {
                    known[9] = pattern;
                } else {
                    known[0] = pattern;
                }
            }
            _ => unreachable!(),
        }
    }

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
    fn test_example() {
        let input = fs::read_to_string("input-test").unwrap();
        let entries = entry_parser::parse(&input).unwrap();
        assert_eq!(sum_output_values(&entries), 61229);
    }
}
