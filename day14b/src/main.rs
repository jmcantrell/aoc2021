use std::collections::HashMap;
use std::fs;

type Element = char;
type ElementPair = [Element; 2];

#[derive(Debug)]
pub struct Polymer {
    element_counts: HashMap<Element, usize>,
    element_pairs: HashMap<ElementPair, usize>,
    insertion_rules: HashMap<ElementPair, Element>,
}

impl Polymer {
    fn step(&mut self) {
        let mut new_pairs: HashMap<ElementPair, usize> = Default::default();

        for (pair, pair_total) in self.element_pairs.iter_mut() {
            if *pair_total > 0 {
                let new_element = *self.insertion_rules.get(pair).unwrap();

                new_pairs
                    .entry([pair[0], new_element])
                    .and_modify(|new| *new += *pair_total)
                    .or_insert(*pair_total);
                new_pairs
                    .entry([new_element, pair[1]])
                    .and_modify(|new| *new += *pair_total)
                    .or_insert(*pair_total);

                self.element_counts
                    .entry(new_element)
                    .and_modify(|total| *total += *pair_total)
                    .or_insert(1);

                *pair_total = 0;
            }
        }

        for (pair, new) in new_pairs.into_iter() {
            self.element_pairs
                .entry(pair)
                .and_modify(|count| *count += new)
                .or_insert(new);
        }
    }

    fn elements_sorted_by_frequency(&self) -> Vec<(&Element, &usize)> {
        let mut counts: Vec<(&Element, &usize)> = self.element_counts.iter().collect();
        counts.sort_by_key(|&(_, frequency)| std::cmp::Reverse(frequency));
        counts
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
                let element_counts: HashMap<Element, usize> = elements.iter().fold(HashMap::new(), |mut map, &element| {
                    map.entry(element).and_modify(|count| *count += 1).or_insert(1);
                    map
                });
                let element_pairs: HashMap<ElementPair, usize> = elements.windows(2).fold(HashMap::new(), |mut map, pair| {
                    if let [a, b] = pair {
                        map.entry([*a, *b]).and_modify(|count| *count += 1).or_insert(1);
                    }
                    map
                });

                Polymer { element_counts, element_pairs, insertion_rules }
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
    dbg!(element_frequency_difference_after_n_steps(&mut polymer, 40));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("input-test").unwrap();
        let mut polymer = polymer_parser::parse(&input).unwrap();

        assert_eq!(
            element_frequency_difference_after_n_steps(&mut polymer, 40),
            2188189693529
        );
    }
}
