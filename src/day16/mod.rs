#![allow(unused)]

use crate::matrix::AoCMatrix;
use std::collections::HashSet;
use std::iter::zip;

/*
    Types
*/

type BEAMSTATE = ((usize, usize), Direction);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(non_camel_case_types)]
enum Space {
    Empty,
    SW_NE,
    NW_SE,
    NS,
    EW,
}

impl Space {
    fn new(c: char) -> Space {
        match c {
            '.' => Space::Empty,
            '/' => Space::SW_NE,
            '\\' => Space::NW_SE,
            '|' => Space::NS,
            '-' => Space::EW,
            _ => panic!("Unrecognized character: {}", c),
        }
    }
}
impl std::fmt::Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Space::Empty => write!(f, "."),
            Space::SW_NE => write!(f, "/"),
            Space::NW_SE => write!(f, "\\"),
            Space::NS => write!(f, "|"),
            Space::EW => write!(f, "-"),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn next_pos(&self, cpos: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::North => {
                if cpos.0 == 0 {
                    None
                } else {
                    Some((cpos.0 - 1, cpos.1))
                }
            }
            Direction::South => Some((cpos.0 + 1, cpos.1)),
            Direction::East => Some((cpos.0, cpos.1 + 1)),
            Direction::West => {
                if cpos.1 == 0 {
                    None
                } else {
                    Some((cpos.0, cpos.1 - 1))
                }
            }
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, "↑"),
            Direction::South => write!(f, "↓"),
            Direction::East => write!(f, "→"),
            Direction::West => write!(f, "←"),
        }
    }
}

/*
    Entry
*/

pub fn part1(lines: Vec<String>) -> usize {
    let (board, mut state) = parse_board(lines);

    return run_from_start(((0, 0), Direction::East), &board, &mut state);
}
pub fn part2(lines: Vec<String>) -> usize {
    let (board, mut state) = parse_board(lines);
    let starts = generate_starts(&board);

    let results = run_from_starts(&starts, &board, &state);

    return *results.iter().max().unwrap();
}

/*
    Business logic
*/

fn generate_starts(board: &AoCMatrix<Space>) -> Vec<BEAMSTATE> {
    let mut bs = vec![];
    for c in 0..board.n_cols() {
        bs.push(((0, c), Direction::South));
        bs.push(((board.n_rows() - 1, c), Direction::North));
    }
    for r in 0..board.n_rows() {
        bs.push(((r, 0), Direction::East));
        bs.push(((r, board.n_cols() - 1), Direction::West));
    }

    return bs;
}

fn run_from_starts(
    starts: &Vec<BEAMSTATE>,
    board: &AoCMatrix<Space>,
    state: &AoCMatrix<bool>,
) -> Vec<usize> {
    return starts
        .iter()
        .map(move |s| {
            let mut state_copy = state.clone();
            run_from_start(*s, &board, &mut state_copy)
        })
        .collect::<Vec<usize>>();
}

fn run_from_start(
    start: BEAMSTATE,
    board: &AoCMatrix<Space>,
    mut state: &mut AoCMatrix<bool>,
) -> usize {
    state[start.0] = true;
    let mut cmd_vec = vec![start];

    let mut visited = HashSet::new();

    while let Some((cur_pos, dir)) = cmd_vec.pop() {
        let new_cmds = process_step((cur_pos, dir), &board, &mut state, &mut visited);
        for c in new_cmds {
            cmd_vec.push(c);
        }
    }
    return state.get_data().iter().map(|b| *b as usize).sum();
}

fn process_step(
    (cur_pos, dir): BEAMSTATE,
    board: &AoCMatrix<Space>,
    state: &mut AoCMatrix<bool>,
    processed: &mut HashSet<BEAMSTATE>,
) -> Vec<BEAMSTATE> {
    if processed.contains(&(cur_pos, dir)) {
        return vec![];
    } else {
        processed.insert((cur_pos, dir));
    }

    // Unwrap Nones with a default value outside of the matrix
    let next_pos = dir
        .next_pos(cur_pos)
        .unwrap_or((board.n_rows() + 1, board.n_cols() + 1));

    if !board.in_mat(next_pos) {
        return vec![];
    }

    state[next_pos] = true;
    let new_space = board[next_pos];

    return match new_space {
        Space::Empty => vec![(next_pos, dir)],
        Space::NS => match dir {
            Direction::South | Direction::North => vec![(next_pos, dir)],
            Direction::East | Direction::West => {
                vec![(next_pos, Direction::North), (next_pos, Direction::South)]
            }
        },
        Space::EW => match dir {
            Direction::East | Direction::West => vec![(next_pos, dir)],
            Direction::South | Direction::North => {
                vec![(next_pos, Direction::East), (next_pos, Direction::West)]
            }
        },
        Space::NW_SE => match dir {
            Direction::North => vec![(next_pos, Direction::West)],
            Direction::South => vec![(next_pos, Direction::East)],
            Direction::East => vec![(next_pos, Direction::South)],
            Direction::West => vec![(next_pos, Direction::North)],
        },
        Space::SW_NE => match dir {
            Direction::North => vec![(next_pos, Direction::East)],
            Direction::South => vec![(next_pos, Direction::West)],
            Direction::East => vec![(next_pos, Direction::North)],
            Direction::West => vec![(next_pos, Direction::South)],
        },
    };
}

/*
    Parsing
*/

fn parse_board(lines: Vec<String>) -> (AoCMatrix<Space>, AoCMatrix<bool>) {
    let board = AoCMatrix::from_rows(
        lines
            .into_iter()
            .map(|l| {
                l.chars()
                    .into_iter()
                    .map(|c| Space::new(c))
                    .collect::<Vec<Space>>()
            })
            .collect::<Vec<Vec<Space>>>(),
    );

    let rows = board.n_rows();
    let cols = board.n_cols();
    let init_rows = vec![vec![false; cols]; rows];

    let state = AoCMatrix::from_rows(init_rows);

    return (board, state);
}

/*
 Tests
*/
#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::string_to_lines;

    #[test]
    fn part1_test() {
        let string_input = ".|...\\....\n|.-.\\.....\n.....|-...\n........|.\n..........\n.........\\\n..../.\\\\..\n.-.-/..|..\n.|....-|.\\\n..//.|....".to_string();
        let line_input = string_to_lines(&string_input);

        let result = part1(line_input);

        assert_eq!(result, 46);
    }

    #[test]
    fn part2_test() {
        let string_input = ".|...\\....\n|.-.\\.....\n.....|-...\n........|.\n..........\n.........\\\n..../.\\\\..\n.-.-/..|..\n.|....-|.\\\n..//.|....".to_string();
        let line_input = string_to_lines(&string_input);

        let result = part2(line_input);

        assert_eq!(result, 51);
    }
}
