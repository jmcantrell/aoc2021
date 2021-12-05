use std::collections::{HashMap, HashSet};
use std::fs;

const LINE_SIZE: usize = 5;
const NUM_SPACES: usize = LINE_SIZE * LINE_SIZE;

type Index = usize;
type Number = u32;

fn get_row(index: Index) -> Index {
    index / LINE_SIZE
}

fn get_column(index: Index) -> Index {
    index % LINE_SIZE
}

fn get_row_line(index: Index) -> Vec<Index> {
    let row = get_row(index) * LINE_SIZE;
    (row..row + LINE_SIZE).collect()
}

fn get_column_line(index: Index) -> Vec<Index> {
    let column = get_column(index);
    (column..NUM_SPACES).step_by(LINE_SIZE).collect()
}

#[derive(Debug, Default, PartialEq)]
pub struct Board {
    numbers: Vec<Number>,
    indexes: HashMap<Number, Index>,
}

impl Board {
    fn new(numbers: Vec<Number>) -> Self {
        let mut indexes: HashMap<Number, Index> = Default::default();
        for (index, &number) in numbers.iter().enumerate() {
            indexes.insert(number, index);
        }
        Self { numbers, indexes }
    }

    fn lines_containing(&self, number: Number) -> Vec<Vec<Number>> {
        let mut indexes_for_lines: Vec<Vec<Index>> = Default::default();

        if let Some(&index) = self.indexes.get(&number) {
            indexes_for_lines.push(get_row_line(index));
            indexes_for_lines.push(get_column_line(index));
        }

        indexes_for_lines
            .iter()
            .map(|indexes| indexes.iter().map(|&index| self.numbers[index]).collect())
            .collect()
    }
}

#[derive(Debug)]
pub struct Session {
    moves: Vec<Number>,
    boards: Vec<Board>,
}

impl Session {
    fn get_last_win_score(&self) -> Option<Number> {
        let mut moves_made: HashSet<Number> = Default::default();
        let mut boards_won: HashSet<usize> = Default::default();
        let mut last_score: Option<Number> = None;

        for &number in self.moves.iter() {
            moves_made.insert(number);
            for (board_index, board) in self.boards.iter().enumerate() {
                if boards_won.contains(&board_index) {
                    continue;
                }
                for line in board.lines_containing(number) {
                    if line.iter().all(|n| moves_made.contains(n)) {
                        boards_won.insert(board_index);
                        let unmarked_score: Number = board
                            .numbers
                            .iter()
                            .filter(|n| !moves_made.contains(n))
                            .sum();
                        last_score = Some(unmarked_score * number);
                    }
                }
            }
        }

        last_score
    }
}

peg::parser! {
    grammar session_parser() for str {
        pub rule parse() -> Session
            = moves:parse_moves() "\n\n" boards:(parse_board() ++ "\n\n") "\n" {
                Session{ moves, boards }
            }

        rule parse_moves() -> Vec<Number>
            = numbers:(parse_uint() ++ ",") {
                numbers
            }

        rule parse_board() -> Board
            = numbers:parse_numbers() {
                Board::new(numbers)
            }

        rule parse_numbers() -> Vec<Number>
            = rows:(parse_row() **<{LINE_SIZE}> "\n") {
                rows.into_iter().flatten().collect::<Vec<Number>>()
            }

        rule parse_row() -> Vec<Number>
            = parse_space() **<{LINE_SIZE}> " "

        rule parse_space() -> Number
            = " "* number:parse_uint() {
                number
            }

        rule parse_uint() -> Number
            = s:$(['0'..='9']+) {
                s.parse().unwrap()
            }
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let session = session_parser::parse(&input).unwrap();
    dbg!(session.get_last_win_score().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_row() {
        for row in 0..LINE_SIZE {
            let start = row * LINE_SIZE;
            let end = start + LINE_SIZE;
            for index in start..end {
                assert_eq!(get_row(index), row);
            }
        }
    }

    #[test]
    fn test_get_column() {
        for column in 0..LINE_SIZE {
            let start = column;
            let end = 20 + start;
            for index in (start..end).step_by(LINE_SIZE) {
                assert_eq!(get_column(index), column);
            }
        }
    }

    #[test]
    fn test_get_row_line() {
        for row in 0..LINE_SIZE {
            let start = row * LINE_SIZE;
            let line: Vec<Index> = (start..start + LINE_SIZE).collect();
            for &index in line.iter() {
                assert_eq!(get_row_line(index), line);
            }
        }
    }

    #[test]
    fn test_get_column_line() {
        for column in 0..LINE_SIZE {
            let start = column;
            let line: Vec<Index> = (start..NUM_SPACES).step_by(LINE_SIZE).collect();
            for &index in line.iter() {
                assert_eq!(get_column_line(index), line);
            }
        }
    }

    #[test]
    fn test_board_lines_containing() {
        let target_number = 9;
        let target_index = 12;
        let row_line = get_row_line(target_index);
        let column_line = get_column_line(target_index);
        let mut numbers = vec![0; NUM_SPACES];
        for &index in row_line.iter() {
            numbers[index] = 1;
        }
        for &index in column_line.iter() {
            numbers[index] = 2;
        }
        numbers[target_index] = target_number;
        let board = Board::new(numbers);
        assert_eq!(
            board.lines_containing(target_number),
            vec![vec![1, 1, 9, 1, 1], vec![2, 2, 9, 2, 2]]
        );
    }

    #[test]
    fn test_example() {
        let input = fs::read_to_string("input-test").unwrap();
        let session = session_parser::parse(&input).unwrap();
        assert_eq!(session.get_last_win_score(), Some(1924));
    }
}
