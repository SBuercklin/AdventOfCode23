use crate::matrix::AoCMatrix;
use std::collections::HashSet;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

type HEAP = BinaryHeap<Reverse<DijkstraState>>;
type BOARD = AoCMatrix<usize>;
type POSITION = (usize, usize);

/*
    Types
*/

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Move {
    position: POSITION,
    direction: Direction,
    min: usize,
    max: usize,
    current: usize,
}

impl Move {
    fn new(
        current: usize,
        position: POSITION,
        direction: Direction,
        min: usize,
        max: usize,
    ) -> Move {
        return Move {
            current,
            position,
            direction,
            min,
            max,
        };
    }
    fn new_from_zero(position: POSITION, direction: Direction, min: usize, max: usize) -> Move {
        return Move {
            current: 0,
            position,
            direction,
            min,
            max,
        };
    }
    fn step(&self) -> Option<Move> {
        if self.current == self.max {
            return None;
        }
        match self.direction {
            Direction::North => {
                if self.position.0 == 0 {
                    return None;
                }
            }
            Direction::West => {
                if self.position.1 == 0 {
                    return None;
                }
            }
            _ => (),
        };
        let cur_pos = self.position;
        let new_pos = match self.direction {
            Direction::North => (cur_pos.0 - 1, cur_pos.1),
            Direction::South => (cur_pos.0 + 1, cur_pos.1),
            Direction::East => (cur_pos.0, cur_pos.1 + 1),
            Direction::West => (cur_pos.0, cur_pos.1 - 1),
        };

        return Some(Move::new(
            self.current + 1,
            new_pos,
            self.direction,
            self.min,
            self.max,
        ));
    }
    fn left(&self) -> Option<Move> {
        return if self.can_stop() {
            Move::new_from_zero(self.position, self.direction.left(), self.min, self.max).step()
        } else {
            None
        };
    }
    fn right(&self) -> Option<Move> {
        return if self.can_stop() {
            Move::new_from_zero(self.position, self.direction.right(), self.min, self.max).step()
        } else {
            None
        };
    }
    fn can_stop(&self) -> bool {
        return self.min <= self.current;
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy, Ord, PartialOrd)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }
    fn right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct DijkstraState {
    /// val_to_here is the total walk to here **including the current square**
    /// position is the current index
    /// move_to_here is the movement that brought us into this square
    val_to_here: usize,
    current_move: Move,
}

impl DijkstraState {
    fn new(val_to_here: usize, move_to_here: Move) -> DijkstraState {
        return DijkstraState {
            val_to_here,
            current_move: move_to_here,
        };
    }
}

/*
    Entry
*/

pub fn part1(lines: Vec<String>) -> usize {
    let board = parse_input(&lines);
    let mut heap = HEAP::new();
    push_to_heap(
        DijkstraState::new(0, Move::new_from_zero((0, 0), Direction::East, 1, 3)),
        &mut heap,
    );

    let tgt = (board.n_rows() - 1, board.n_cols() - 1);

    return dijkstra(&mut heap, &board, tgt);
}

pub fn part2(lines: Vec<String>) -> usize {
    let board = parse_input(&lines);
    let mut heap = HEAP::new();
    push_to_heap(
        DijkstraState::new(0, Move::new_from_zero((0, 0), Direction::East, 4, 10)),
        &mut heap,
    );
    push_to_heap(
        DijkstraState::new(0, Move::new_from_zero((0, 0), Direction::South, 4, 10)),
        &mut heap,
    );

    let tgt = (board.n_rows() - 1, board.n_cols() - 1);

    return dijkstra(&mut heap, &board, tgt);
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

/*
 Business Logic
*/

fn dijkstra(heap: &mut HEAP, board: &BOARD, target: POSITION) -> usize {
    let mut visited: HashSet<Move> = HashSet::new();
    while let Some(Reverse(state)) = heap.pop() {
        let cmove = state.current_move;
        let pos = cmove.position;

        if pos == target && cmove.can_stop() {
            return state.val_to_here;
        }
        let left = cmove.left();
        let right = cmove.right();
        let forward = cmove.step();

        let mvs = [left, right, forward];

        for opt_mv in mvs {
            if let Some(mv) = opt_mv {
                let new_pos = mv.position;
                if board.in_mat(new_pos) && visited.get(&mv).is_none() {
                    visited.insert(mv);
                    let new_val = state.val_to_here + board[new_pos];
                    push_to_heap(DijkstraState::new(new_val, mv), heap);
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
        let line_input = string_to_lines(&string_input);

        let result = part1(line_input);

        assert_eq!(result, 102);
    }

    #[test]
    fn part2_test() {
        let string_input = "2413432311323\n3215453535623\n3255245654254\n3446585845452\n4546657867536\n1438598798454\n4457876987766\n3637877979653\n4654967986887\n4564679986453\n1224686865563\n2546548887735\n4322674655533".to_string();
        let line_input = string_to_lines(&string_input);

        let result = part2(line_input);

        assert_eq!(result, 94);
    }

    #[test]
    fn part2_test_extra() {
        let string_input =
            "111111111111\n999999999991\n999999999991\n999999999991\n999999999991".to_string();
        let line_input = string_to_lines(&string_input);

        let result = part2(line_input);

        assert_eq!(result, 71);
    }
}
