use crate::matrix::AoCMatrix;
use std::collections::HashSet;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

type HEAP = BinaryHeap<Reverse<DijkstraState>>;
type BOARD = AoCMatrix<usize>;
type POSITION = (usize, usize);
type MOVESTATE = (POSITION, Move);

/*
    Types
*/

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy, Ord, PartialOrd)]
enum Move {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    NoMove,
}

impl Move {
    fn left(&self) -> Move {
        match self {
            Move::North(_) => Move::West(1),
            Move::South(_) => Move::East(1),
            Move::East(_) => Move::North(1),
            Move::West(_) => Move::South(1),
            Move::NoMove => todo!(),
        }
    }
    fn right(&self) -> Move {
        match self {
            Move::North(_) => Move::East(1),
            Move::South(_) => Move::West(1),
            Move::East(_) => Move::South(1),
            Move::West(_) => Move::North(1),
            Move::NoMove => todo!(),
        }
    }
    fn forward(&self) -> Option<Move> {
        match self {
            Move::North(i) => {
                if i >= &3 {
                    None
                } else {
                    Some(Move::North(i + 1))
                }
            }
            Move::South(i) => {
                if i >= &3 {
                    None
                } else {
                    Some(Move::South(i + 1))
                }
            }
            Move::East(i) => {
                if i >= &3 {
                    None
                } else {
                    Some(Move::East(i + 1))
                }
            }
            Move::West(i) => {
                if i >= &3 {
                    None
                } else {
                    Some(Move::West(i + 1))
                }
            }
            Move::NoMove => todo!(),
        }
    }
    fn step(&self, p: POSITION) -> Option<POSITION> {
        let (row, col) = p;
        return match self {
            Move::North(_) => {
                if row > 0 {
                    Some((row - 1, col))
                } else {
                    None
                }
            }
            Move::South(_) => Some((row + 1, col)),
            Move::West(_) => {
                if col > 0 {
                    Some((row, col - 1))
                } else {
                    None
                }
            }
            Move::East(_) => Some((row, col + 1)),
            Move::NoMove => todo!(),
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct DijkstraState {
    /// val_to_here is the total walk to here **including the current square**
    /// position is the current index
    /// move_to_here is the movement that brought us into this square
    val_to_here: usize,
    position: POSITION,
    move_to_here: Move,
}

impl DijkstraState {
    fn new(val_to_here: usize, position: POSITION, move_to_here: Move) -> DijkstraState {
        return DijkstraState {
            val_to_here,
            position,
            move_to_here,
        };
    }
}

/*
    Entry
*/

pub fn part1(lines: Vec<String>) -> usize {
    let board = parse_input(&lines);
    let mut heap = HEAP::new();
    push_to_heap(DijkstraState::new(0, (0, 0), Move::East(0)), &mut heap);

    let tgt = (board.n_rows() - 1, board.n_cols() - 1);

    return dijkstra(&mut heap, &board, tgt);
}
pub fn part2(lines: Vec<String>) -> usize {
    return 1;
}

/*
 Parsing, problem setup
*/

fn parse_input(input: &Vec<String>) -> BOARD {
    let row_ints = input
        .iter()
        .map(|r| {
            r.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    return AoCMatrix::from_rows(row_ints);
}

fn heat_loss_init(input: &BOARD) -> BOARD {
    let rows = input.n_rows();
    let cols = input.n_cols();

    let mut row_dummy = vec![];
    for _ in 0..cols {
        row_dummy.push(usize::MAX);
    }

    let mut row_vecs = vec![];
    for _ in 0..rows {
        row_vecs.push(row_dummy.clone());
    }

    return AoCMatrix::from_rows(row_vecs);
}

/*
 Business Logic
*/

fn dijkstra(heap: &mut HEAP, board: &BOARD, target: POSITION) -> usize {
    let mut visited: HashSet<(POSITION, Move)> = HashSet::new();
    while let Some(Reverse(state)) = heap.pop() {
        let pos = state.position;

        if state.position == target {
            return state.val_to_here;
        }
        let left = state.move_to_here.left();
        let right = state.move_to_here.right();
        let forward = state.move_to_here.forward();

        let left_move = left.step(pos);
        let right_move = right.step(pos);
        let forward_move = match forward {
            None => None,
            Some(mv) => mv.step(pos),
        };

        for (mv, newpos) in [
            (left, left_move),
            (right, right_move),
            (forward.unwrap_or(Move::NoMove), forward_move),
        ] {
            if let Some(pos) = newpos {
                if board.in_mat(pos) && visited.get(&(pos, mv)).is_none() {
                    visited.insert((pos, mv));
                    let new_val = state.val_to_here + board[pos];
                    push_to_heap(DijkstraState::new(new_val, pos, mv), heap);
                }
            }
        }
    }

    return 1;
}

fn push_to_heap(state: DijkstraState, heap: &mut HEAP) {
    heap.push(Reverse(state));
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
        let string_input = "2413432311323\n3215453535623\n3255245654254\n3446585845452\n4546657867536\n1438598798454\n4457876987766\n3637877979653\n4654967986887\n4564679986453\n1224686865563\n2546548887735\n4322674655533".to_string();
        // let string_input = "2413\n3215\n3255".to_string();
        let line_input = string_to_lines(&string_input);

        let result = part1(line_input);

        assert_eq!(result, 102);
    }

    // #[test]
    // fn part2_test() {
    //     let string_input = ".|...\\....\n|.-.\\.....\n.....|-...\n........|.\n..........\n.........\\\n..../.\\\\..\n.-.-/..|..\n.|....-|.\\\n..//.|....".to_string();
    //     let line_input = string_to_lines(&string_input);

    //     let result = part2(line_input);

    //     assert_eq!(result, 51);
    // }
}
