/*
 I'm really pretty unhappy with this solution. I should've come up with better methods for
 Connection and Board to make this problem easier to grapple with, e.g. the `connects_X`
 methods I implemented at the end to get the starting pipe shape figured out.
*/

use std::iter::zip;

type LOCATION = (usize, usize);

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Connection {
    NS,
    EW,
    NE,
    NW,
    SE,
    SW,
    Empty,
    Start,
}

impl Connection {
    fn new(c: char) -> Connection {
        match c {
            '|' => Connection::NS,
            '-' => Connection::EW,
            'L' => Connection::NE,
            'J' => Connection::NW,
            '7' => Connection::SW,
            'F' => Connection::SE,
            '.' => Connection::Empty,
            'S' => Connection::Start,
            s => panic!("Unrecognized character in input: {}", s),
        }
    }
    fn connects_south(&self) -> bool {
        return [Connection::NS, Connection::SE, Connection::SW].contains(self);
    }
    fn connects_north(&self) -> bool {
        return [Connection::NS, Connection::NE, Connection::NW].contains(self);
    }
    fn connects_east(&self) -> bool {
        return [Connection::EW, Connection::SE, Connection::NE].contains(self);
    }
    fn connects_west(&self) -> bool {
        return [Connection::EW, Connection::SW, Connection::NW].contains(self);
    }
}

struct Board {
    characters: Vec<Vec<Connection>>,
    dists: Vec<Vec<i64>>,
}

#[derive(Copy, Clone, Debug)]
struct ExploreStep {
    // Distance leading to this point
    dist_in: i64,

    // Location of the point to be explored
    location: LOCATION,
}

impl ExploreStep {
    fn new(dist_in: i64, location: LOCATION) -> ExploreStep {
        ExploreStep { dist_in, location }
    }
}

pub fn part1(lines: Vec<String>) -> i64 {
    let board = make_board(lines);
    let board = populate_dists(board);

    let mut dists = board.dists;

    let result = dists
        .iter_mut()
        .reduce(|a, b| {
            a.append(b);
            a
        })
        .unwrap()
        .iter()
        .filter(|v| v.is_positive())
        .max()
        .unwrap();

    return result.to_owned();
}

fn populate_dists(mut board: Board) -> Board {
    let start = find_start(&board);
    let mut explore_queue = generate_start_queue(&board, start);

    loop {
        let step = explore_queue[0];

        // This updates in-place
        update_board_queue(&mut board, step, &mut explore_queue);

        if explore_queue.len() == 1 {
            break;
        } else {
            explore_queue = explore_queue[1..].to_vec();
        }
    }

    return board;
}

fn update_board_queue(board: &mut Board, step: ExploreStep, queue: &mut Vec<ExploreStep>) {
    let (row, col) = step.location;
    let board_dist = board.dists[row][col];
    let cur_connection = board_index(board, row, col).unwrap().to_owned();

    if board_dist.is_negative() || (!board_dist.is_negative() && step.dist_in < board_dist) {
        board.dists[row][col] = step.dist_in;

        if let Some((a, b)) = next_step(step, cur_connection) {
            queue.push(a);
            queue.push(b);
        }
    }
    return ();
}

fn next_step(
    cur_step: ExploreStep,
    cur_connection: Connection,
) -> Option<(ExploreStep, ExploreStep)> {
    let ndist = cur_step.dist_in + 1;
    let (crow, ccol) = cur_step.location;
    let n = (crow - 1, ccol);
    let s = (crow + 1, ccol);
    let e = (crow, ccol + 1);
    let w = (crow, ccol - 1);

    return match cur_connection {
        Connection::EW => Some((ExploreStep::new(ndist, e), ExploreStep::new(ndist, w))),
        Connection::NS => Some((ExploreStep::new(ndist, n), ExploreStep::new(ndist, s))),
        Connection::NE => Some((ExploreStep::new(ndist, n), ExploreStep::new(ndist, e))),
        Connection::NW => Some((ExploreStep::new(ndist, n), ExploreStep::new(ndist, w))),
        Connection::SE => Some((ExploreStep::new(ndist, s), ExploreStep::new(ndist, e))),
        Connection::SW => Some((ExploreStep::new(ndist, s), ExploreStep::new(ndist, w))),
        _ => None,
    };
}

fn generate_start_queue(board: &Board, start: LOCATION) -> Vec<ExploreStep> {
    let w = (start.0, start.1 - 1);
    let e = (start.0, start.1 + 1);
    let s = (start.0 + 1, start.1);
    let n = (start.0 - 1, start.1);

    let mut queue: Vec<ExploreStep> = vec![];
    match board_index(&board, n.0, n.1) {
        None => (),
        Some(c) => {
            if [Connection::NS, Connection::SW, Connection::SE].contains(&c) {
                queue.push(ExploreStep::new(1, n))
            }
        }
    }
    match board_index(&board, s.0, s.1) {
        None => (),
        Some(c) => {
            if [Connection::NS, Connection::NW, Connection::NE].contains(&c) {
                queue.push(ExploreStep::new(1, s))
            }
        }
    }
    match board_index(&board, e.0, e.1) {
        None => (),
        Some(c) => {
            if [Connection::EW, Connection::SW, Connection::NW].contains(&c) {
                queue.push(ExploreStep::new(1, e))
            }
        }
    }
    match board_index(&board, w.0, w.1) {
        None => (),
        Some(c) => {
            if [Connection::EW, Connection::NE, Connection::SE].contains(&c) {
                queue.push(ExploreStep::new(1, w))
            }
        }
    }

    return queue;
}

fn board_index(b: &Board, row: usize, col: usize) -> Option<&Connection> {
    let rows = b.characters.len();
    let cols = b.characters[0].len();

    if row < rows && col < cols {
        return Some(&b.characters[row][col]);
    } else {
        return None;
    };
}

fn make_board(lines: Vec<String>) -> Board {
    let characters = lines.iter().map(|l| parse_line(l)).collect();
    let dists = dist_mat(lines);

    return Board { characters, dists };
}

fn parse_line(l: &str) -> Vec<Connection> {
    return l.chars().map(|c| Connection::new(c)).collect();
}

fn find_start(b: &Board) -> LOCATION {
    for (row_idx, row) in b.characters.iter().enumerate() {
        match row.iter().position(|p| p.eq(&Connection::Start)) {
            Some(col_idx) => return (row_idx, col_idx),
            None => (),
        };
    }
    panic!("Couldn't find start on board");
}

fn dist_mat(l: Vec<String>) -> Vec<Vec<i64>> {
    return l
        .iter()
        .map(|l| {
            let length: usize = l.len();
            let mut dist_row: Vec<i64> = vec![];
            for _ in 0..length {
                dist_row.push(-1);
            }
            dist_row
        })
        .collect();
}

/*
   *************************************************************************************
    Part 2
   *************************************************************************************
*/

pub fn part2(lines: Vec<String>) -> i64 {
    let board = make_board(lines);
    let mut board = populate_dists(board);

    let start = find_start(&board);
    // let explore_queue = generate_start_queue(&board, start);
    // let (a, b) = (explore_queue[0], explore_queue[1]);

    let replace_connection = find_replace_connection(&board, start);

    board.characters[start.0][start.1] = replace_connection;

    let dists = board.dists;

    // 0s are empty space, 1s are actual space
    let in_out_board: Vec<Vec<i64>> = dists
        .into_iter()
        .map(|row| row.iter().map(|v| !v.is_negative() as i64).collect())
        .collect();

    let in_out: i64 = zip(in_out_board, board.characters)
        .map(|(bools, chars)| count_interior(&bools, &chars))
        .sum();

    return in_out;
}

fn find_replace_connection(board: &Board, start: LOCATION) -> Connection {
    let north = board.characters[start.0 - 1][start.1].connects_south();
    let south = board.characters[start.0 + 1][start.1].connects_north();
    let west = board.characters[start.0][start.1 - 1].connects_east();
    let east = board.characters[start.0][start.1 + 1].connects_west();

    if north && south {
        return Connection::NS;
    } else if north && east {
        return Connection::NE;
    } else if north && west {
        return Connection::NW;
    } else if east && west {
        return Connection::EW;
    } else if east && south {
        return Connection::SE;
    } else if west && south {
        return Connection::SW;
    } else {
        panic!("Unknown starting shape");
    }
}

fn count_interior(bool_entries: &Vec<i64>, pipe_entries: &Vec<Connection>) -> i64 {
    // When true, we want to accumulate
    let mut mode = false;
    let mut last_edge = Connection::Empty;
    let mut acc = 0;

    for (e, edge) in zip(bool_entries, pipe_entries) {
        // If accumulation is on and we're on a non-surrounding-pipe, accumulate
        if mode && *e == 0 {
            acc += 1;
        } else if *e == 1 {
            match edge {
                Connection::NS => mode = !mode,
                Connection::SE => {
                    last_edge = Connection::SE;
                    mode = !mode;
                }
                Connection::SW => {
                    if last_edge == Connection::SE {
                        mode = !mode;
                    }
                }
                Connection::NE => {
                    last_edge = Connection::NE;
                    mode = !mode
                }
                Connection::NW => {
                    if last_edge == Connection::NE {
                        mode = !mode;
                    }
                }
                _ => mode = mode,
            };
        }
    }

    return acc;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::string_to_lines;

    #[test]
    fn part1_test() {
        let string_input = ".....\n.S-7.\n.|.|.\n.L-J.\n.....".to_string();
        let line_input = string_to_lines(&string_input);

        let result = part1(line_input);

        assert_eq!(result, 4);
    }
    #[test]
    fn part2_test() {
        let string_input = ".....\n.S-7.\n.|.|.\n.L-J.\n.....".to_string();
        let line_input = string_to_lines(&string_input);

        let result = part2(line_input);

        assert_eq!(result, 1);
    }
}
