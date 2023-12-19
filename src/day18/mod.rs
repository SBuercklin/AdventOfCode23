#![allow(unused)]

use std::cmp::max;
use std::fmt::Display;
use std::iter::zip;

use crate::matrix::AoCMatrix;

use hex;
use nom::InputIter;

type BOARD = AoCMatrix<Square>;
type POSITION = (usize, usize);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Square {
    Hole(Color),
    Filled,
    Outside,
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Square::Hole(_) => write!(f, "#"),
            Square::Filled => write!(f, "."),
            Square::Outside => write!(f, "O"),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Color {
    r: u16,
    g: u16,
    b: u16,
}

impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Color")
            .field("r", &self.r)
            .field("g", &self.g)
            .field("b", &self.b)
            .finish()
    }
}

impl Color {
    fn from_hex(s: &str) -> Color {
        // Skip first character, starts with #
        let c1 = &s[1..3];
        let c2 = &s[3..5];
        let c3 = &s[5..7];

        let r = u16::from_str_radix(c1, 16).unwrap();
        let g = u16::from_str_radix(c2, 16).unwrap();
        let b = u16::from_str_radix(c3, 16).unwrap();

        return Color { r, g, b };
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Command {
    direction: char,
    distance: usize,
    color: Color,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/*
 Entry
*/
pub fn part1(lines: Vec<String>) -> usize {
    let cmds = lines.iter().map(parse_line).collect();
    return shoelace_cmds(&cmds);
    // let (mut board, p0) = build_board(&cmds);

    // dig_board_from_cmds(&mut board, &cmds, p0);
    // flood_fill_exterior(&mut board);

    // return board
    //     .get_data()
    //     .iter()
    //     .filter(|s| **s != Square::Outside)
    //     .collect::<Vec<&Square>>()
    //     .len();
}

pub fn part2(lines: Vec<String>) -> usize {
    // let cmds = lines.iter().map(parse_line_color_command).collect();
    return 1;
}

/*
    Approach based on shoelace formula
    Thank you, Ken Williams
*/

fn shoelace_cmds(cmds: &Vec<Command>) -> usize {
    let directions: Vec<Direction> = cmds
        .iter()
        .map(|c| match c.direction {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!(),
        })
        .collect();
    let dists: Vec<i64> = cmds.iter().map(|c| c.distance as i64).collect();

    let d0 = dists[0];
    let dir0 = directions[0];

    // A is LHS, B is RHS
    let (mut ptsa, mut ptsb) = match dir0 {
        Direction::Up => (vec![(0, 0), (0, 2 * d0)], vec![(2, 0), (2, 2 * d0)]),
        Direction::Down => (vec![(2, 0), (2, -2 * d0)], vec![(0, 0), (0, -2 * d0)]),
        Direction::Right => (vec![(0, 2), (2 * d0, 2)], vec![(0, 0), (2 * d0, 0)]),
        Direction::Left => (vec![(0, 0), (-2 * d0, 0)], vec![(0, 2), (-2 * d0, 2)]),
    };

    let mut last_dir = dir0;

    let mut dirdist = zip(directions[1..].iter(), dists[1..].iter());

    while let Some((dir, dist)) = dirdist.next() {
        let last_a = *ptsa.last().unwrap();
        let last_b = *ptsb.last().unwrap();

        // Handle the corners
        match dir {
            Direction::Right => match last_dir {
                Direction::Up => {
                    ptsb.push((last_b.0, last_b.1 - 2));
                    ptsa.push((last_a.0 + 2, last_a.1))
                }
                Direction::Down => {
                    ptsb.push((last_b.0 + 2, last_b.1));
                    ptsa.push((last_a.0, last_a.1 + 2));
                }
                _ => panic!(),
            },
            Direction::Up => match last_dir {
                Direction::Right => {
                    ptsb.push((last_b.0, last_b.1 + 2));
                    ptsa.push((last_a.0 - 2, last_a.1));
                }
                Direction::Left => {
                    ptsb.push((last_b.0 + 2, last_b.1));
                    ptsa.push((last_a.0, last_a.1 + 2));
                }
                _ => panic!(),
            },
            Direction::Down => match last_dir {
                Direction::Right => {
                    ptsb.push((last_b.0 - 2, last_b.1));
                    ptsa.push((last_a.0, last_a.1 - 2));
                }
                Direction::Left => {
                    ptsb.push((last_b.0, last_b.1 - 2));
                    ptsa.push((last_a.0 + 2, last_a.1));
                }
                _ => panic!(),
            },
            Direction::Left => match last_dir {
                Direction::Up => {
                    ptsb.push((last_b.0 - 2, last_b.1));
                    ptsa.push((last_a.0, last_a.1 - 2))
                }
                Direction::Down => {
                    ptsb.push((last_b.0, last_b.1 + 2));
                    ptsa.push((last_a.0 - 2, last_a.1));
                }
                _ => panic!(),
            },
        };
        // Handle extrusion
        match dir {
            Direction::Right => {
                ptsb.push((last_b.0 + 2 * dist, last_b.1));
                ptsa.push((last_b.0 + 2 * dist, last_b.1));
            }
            Direction::Left => {
                ptsb.push((last_b.0 - 2 * dist, last_b.1));
                ptsa.push((last_b.0 - 2 * dist, last_b.1));
            }
            Direction::Up => {
                ptsb.push((last_b.0, last_b.1 + 2 * dist));
                ptsa.push((last_b.0, last_b.1 + 2 * dist));
            }
            Direction::Down => {
                ptsb.push((last_b.0, last_b.1 - 2 * dist));
                ptsa.push((last_b.0, last_b.1 - 2 * dist));
            }
        }
        last_dir = *dir;
    }

    return max(shoelace_pts(&ptsa) as usize, shoelace_pts(&ptsb) as usize);
}

pub fn shoelace_pts(points: &Vec<(i64, i64)>) -> i64 {
    let mut pts = points.clone();
    pts.push(points[0]);
    pts.push(points[1]);

    let mut acc = 0;
    let n = points.len();
    for idx in 0..n {
        let p1 = pts[idx];
        let p2 = pts[idx + 1];

        // println!("{}, {} cross {}, {}", p1.0, p1.1, p2.0, p2.1);

        acc += p1.0 * p2.1 - p2.0 * p1.1;
    }

    // println!("Acc: {}", acc);

    return acc.abs() / 4;
}

/*
 Outdated, Flood-Fill Business logic
*/

fn dig_board_from_cmds(board: &mut BOARD, cmds: &Vec<Command>, p0: POSITION) {
    let mut cidx = p0;
    cmds.iter().for_each(|c| {
        let new_square = Square::Hole(c.color);
        for _ in 0..c.distance {
            let delta = match c.direction {
                'U' => cidx.0 -= 1,
                'D' => cidx.0 += 1,
                'R' => cidx.1 += 1,
                'L' => cidx.1 -= 1,
                _ => panic!("Unrecognized direction"),
            };
            board[cidx] = new_square;
        }
    })
}

fn flood_fill_exterior(board: &mut BOARD) {
    let p0 = (0, 0);
    let mut queue = vec![p0];

    while let Some(p) = queue.pop() {
        board[p] = Square::Outside;
        let mut ps = vec![(p.0 + 1, p.1), (p.0, p.1 + 1)];
        if p.0 > 0 {
            ps.push((p.0 - 1, p.1));
        }
        if p.1 > 0 {
            ps.push((p.0, p.1 - 1));
        }

        for new_p in ps {
            if board.in_mat(new_p) && board[new_p] == Square::Filled {
                queue.push(new_p);
            }
        }
    }
}

/*
 Parsing Input
*/

fn parse_line(l: &String) -> Command {
    let mut pieces = l.split(' ');
    let dir = pieces.next().unwrap().chars().next().unwrap();
    let count: usize = pieces.next().unwrap().parse().unwrap();

    let color = Color::from_hex(&pieces.next().unwrap()[1..9]);

    return Command {
        direction: dir,
        distance: count,
        color,
    };
}

fn parse_line_color_command(l: &String) -> Command {
    let mut pieces = l.split(' ');
    let _dir = pieces.next();
    let _count = pieces.next();

    let color_str = &pieces.next().unwrap().to_string();
    let color = Color::from_hex(&color_str[1..9]);

    let dir = match color_str.chars().nth(7).unwrap() {
        '0' => 'R',
        '1' => 'D',
        '2' => 'L',
        '3' => 'U',
        _ => panic!("Unrecognized direction"),
    };

    let count = usize::from_str_radix(&color_str[2..7], 16).unwrap();

    return Command {
        direction: dir,
        distance: count,
        color,
    };
}

fn build_board(cmds: &Vec<Command>) -> (BOARD, (usize, usize)) {
    let mut cpos = (0, 0);

    let mut max_dx = 0;
    let mut max_dy = 0;
    let mut min_dx = 0;
    let mut min_dy = 0;

    cmds.iter().for_each(|c| {
        let delta = match c.direction {
            'U' => (0, 1),
            'D' => (0, -1),
            'R' => (1, 0),
            'L' => (-1, 0),
            _ => panic!("Unrecognized direction"),
        };
        cpos.0 += delta.0 * (c.distance as i64);
        cpos.1 += delta.1 * (c.distance as i64);
        if cpos.0 > max_dx {
            max_dx = cpos.0;
        } else if cpos.0 < min_dx {
            min_dx = cpos.0;
        };
        if cpos.1 > max_dy {
            max_dy = cpos.1;
        } else if cpos.1 < min_dy {
            min_dy = cpos.1;
        };
    });

    let width = (max_dx - min_dx) * 3;
    let height = (max_dy - min_dy) * 3;

    let board = AoCMatrix::filled_matrix(Square::Filled, height as usize, width as usize);

    return (board, (((height as usize) / 2), (width as usize / 2)));
}

/*
 Tests
*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::string_to_lines;

    // #[test]
    // fn part1_test() {
    //     let string_input = "R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)\nR 2 (#59c680)\nD 2 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)\nL 1 (#1b58a2)\nU 2 (#caa171)\nR 2 (#7807d2)\nU 3 (#a77fa3)\nL 2 (#015232)\nU 2 (#7a21e3)".to_string();
    //     let line_input = string_to_lines(&string_input);

    //     let result = part1(line_input);

    //     assert_eq!(result, 62);
    // }

    // #[test]
    // fn shoelace() {
    //     assert_eq!(shoelace_pts(&vec![(0, 0), (2, 0), (2, 2), (0, 2)]), 4);
    // }

    #[test]
    fn hand_test() {
        let string_input = "R 6 (#70c710)\nD 2 (#0dc571)\nL 6 (#5713f0)\nU 2 (#d2c081)".to_string();
        let line_input = string_to_lines(&string_input);

        let result = part1(line_input);

        assert_eq!(result, 4);
    }

    // #[test]
    // fn part2_test() {
    //     let string_input = "R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)\nR 2 (#59c680)\nD 2 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)\nL 1 (#1b58a2)\nU 2 (#caa171)\nR 2 (#7807d2)\nU 3 (#a77fa3)\nL 2 (#015232)\nU 2 (#7a21e3)".to_string();
    //     let line_input = string_to_lines(&string_input);

    //     let result = part2(line_input);

    //     assert_eq!(result, 952408144115);
    // }
}
