use std::fs;

#[derive(Debug)]
struct Location {
    line: usize,
    column: usize,
}

#[derive(Debug)]
enum SyntaxError {
    IllegalClose(Location),
    IncompleteClose(Vec<char>),
}

#[derive(Debug)]
struct Score {
    illegal_closes: usize,
    incomplete_closes: usize,
}

fn get_closing_char(open: char) -> Option<char> {
    match open {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        _ => None,
    }
}

fn validate(lines: &[&str]) -> Vec<SyntaxError> {
    let mut errors: Vec<SyntaxError> = Default::default();

    'lines: for (line, s) in lines.iter().enumerate() {
        let mut stack: Vec<char> = Default::default();

        for (column, c) in s.chars().enumerate() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                _ => {
                    if c != get_closing_char(stack.pop().unwrap()).unwrap() {
                        errors.push(SyntaxError::IllegalClose(Location { line, column }));
                        continue 'lines;
                    }
                }
            }
        }

        if !stack.is_empty() {
            let mut closing: Vec<char> = Default::default();

            while !stack.is_empty() {
                closing.push(get_closing_char(stack.pop().unwrap()).unwrap());
            }

            errors.push(SyntaxError::IncompleteClose(closing));
        }
    }

    errors
}

fn score_syntax_errors(lines: &[&str]) -> Score {
    let errors = validate(lines);
    let mut illegal_closes = 0;
    let mut incomplete_closes_scores: Vec<usize> = Default::default();

    for error in errors {
        match error {
            SyntaxError::IllegalClose(location) => {
                illegal_closes += match lines[location.line].chars().nth(location.column).unwrap() {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => unreachable!(),
                }
            }
            SyntaxError::IncompleteClose(closing) => {
                let mut score = 0;
                for c in closing {
                    score *= 5;
                    score += match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    };
                }
                incomplete_closes_scores.push(score);
            }
        }
    }

    incomplete_closes_scores.sort_unstable();
    let incomplete_closes = incomplete_closes_scores[incomplete_closes_scores.len() / 2];

    Score {
        illegal_closes,
        incomplete_closes,
    }
}

fn parse_lines(s: &str) -> Vec<&str> {
    s.lines().collect()
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let lines = parse_lines(&input);
    let score = score_syntax_errors(&lines);
    dbg!(score.illegal_closes, score.incomplete_closes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("input-test").unwrap();
        let lines = parse_lines(&input);
        let score = score_syntax_errors(&lines);
        assert_eq!(score.illegal_closes, 26397);
        assert_eq!(score.incomplete_closes, 288957);
    }
}
