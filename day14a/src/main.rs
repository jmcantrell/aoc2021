use std::collections::HashMap;
use std::fmt;
use std::fs;

type Element = char;
type ElementPair = [Element; 2];

#[derive(Debug)]
pub struct Polymer {
    elements: Vec<Element>,
    insertion_rules: HashMap<ElementPair, Element>,
}

impl Polymer {
    fn step(&mut self) {
        let n = self.elements.len();
        let mut last: Option<char> = None;
        let mut elements: Vec<Element> = Vec::with_capacity(n * 2 - 1);

        for pair in self.elements.windows(2) {
            let element = *self.insertion_rules.get(pair).unwrap();
            elements.push(pair[0]);
            elements.push(element);
            last = Some(pair[1])
        }

        if let Some(element) = last {
            elements.push(element);
        }

        self.elements = elements;
    }

    fn elements_by_frequency(&self) -> HashMap<Element, usize> {
        self.elements
            .iter()
            .copied()
            .fold(HashMap::new(), |mut map, element| {
                map.entry(element)
                    .and_modify(|frequency| *frequency += 1)
                    .or_insert(1);
                map
            })
    }

    fn elements_sorted_by_frequency(&self) -> Vec<(Element, usize)> {
        let mut frequencies: Vec<(Element, usize)> =
            self.elements_by_frequency().into_iter().collect();

        frequencies.sort_by_key(|&(_, frequency)| std::cmp::Reverse(frequency));
        frequencies
    }
}

impl fmt::Display for Polymer {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.elements.iter().collect::<String>())?;
        Ok(())
    }
}

fn element_frequency_difference_after_n_steps(polymer: &mut Polymer, n: usize) -> usize {
    for _ in 0..n {
        polymer.step();
    }

    let element_frequencies = polymer.elements_sorted_by_frequency();
    element_frequencies.first().unwrap().1 - element_frequencies.last().unwrap().1
}

peg::parser! {
    grammar polymer_parser() for str {
        pub rule parse() -> Polymer
            = elements:parse_template() "\n\n" insertion_rules:parse_insertion_rules() "\n"? {
                Polymer { elements, insertion_rules }
            }

        rule parse_template() -> Vec<Element>
            = parse_element()+

        rule parse_insertion_rules() -> HashMap<ElementPair, Element>
            = items:(parse_insertion_rule() ** "\n") {
                items.into_iter().collect()
            }

        rule parse_insertion_rule() -> (ElementPair, Element)
            = a:parse_element() b:parse_element() " -> " value:parse_element() {
                ([a, b], value)
            }

        rule parse_element() -> Element
            = s:$(['A'..='Z']) {
                s.chars().next().unwrap()
            }
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut polymer = polymer_parser::parse(&input).unwrap();
    dbg!(element_frequency_difference_after_n_steps(&mut polymer, 10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("input-test").unwrap();
        let mut polymer = polymer_parser::parse(&input).unwrap();

        polymer.step();
        assert_eq!(format!("{}", polymer), "NCNBCHB");

        polymer.step();
        assert_eq!(format!("{}", polymer), "NBCCNBBBCBHCB");

        polymer.step();
        assert_eq!(format!("{}", polymer), "NBBBCNCCNBBNBNBBCHBHHBCHB");

        polymer.step();
        assert_eq!(
            format!("{}", polymer),
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
        );

        // Steps 1-4 done above.
        assert_eq!(
            element_frequency_difference_after_n_steps(&mut polymer, 6),
            1588
        );
    }
}
