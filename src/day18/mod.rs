#![allow(unused)]

use std::cmp::max;
use std::collections::HashSet;
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct VerticalLine {
    top: i64,
    bot: i64,
    x: i64,
    bot_dir: Direction,
    top_dir: Direction,
}

impl VerticalLine {
    fn intersects(&self, y: i64) -> bool {
        return self.bot <= y && y <= self.top;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct HorizontalLine {
    length: i64,
}

/*
 Entry
*/
pub fn part1(lines: Vec<String>) -> i64 {
    let cmds: Vec<Command> = lines.iter().map(parse_line).collect();
    return solve(cmds);
}

pub fn part2(lines: Vec<String>) -> i64 {
    // let cmds = lines.iter().map(parse_line_color_command).collect();
    return 1;
}

/*
    Working with lines
*/

fn solve(cmds: Vec<Command>) -> i64 {
    let (horz, mut verts) = cmds_to_lines(cmds);
    let maxy = verts.iter().map(|v| v.top).max().unwrap();
    let miny = verts.iter().map(|v| v.bot).min().unwrap();

    let mut adjacency = HashSet::new();
    adjacency.insert((*verts.first().unwrap(), *verts.last().unwrap()));
    adjacency.insert((*verts.last().unwrap(), *verts.first().unwrap()));
    for i in 0..(verts.len() - 1) {
        let a = verts[i];
        let b = verts[i + 1];
        adjacency.insert((a, b));
        adjacency.insert((b, a));
    }

    verts.sort_by(|a, b| a.x.cmp(&b.x));

    let mut acc = horz.iter().map(|h| h.length).sum();

    for row in miny..=maxy {
        let mut cacc = 0;
        let mut next_step_counts = true; // We start outside
        let mut prev_step_counted = false;
        let lverts: Vec<&VerticalLine> = verts.iter().filter(|v| v.intersects(row)).collect();
        for i in 0..lverts.len() {
            if i == lverts.len() - 1 {
                cacc += 1
            } else {
                let a = lverts[i];
                let b = lverts[i + 1];
                let adjacent = adjacency.get(&(*a, *b)).is_some();

                // By default, count the first
                cacc += 1;

                if adjacent
                    && ((a.top == row && a.top == b.top) || (a.bot == row && a.bot == b.bot))
                {
                    next_step_counts = !next_step_counts;
                } else if adjacent
                    && ((a.top == row && a.top == b.bot) || (a.bot == row && a.bot == b.top))
                {
                } else {
                    if next_step_counts {
                        cacc += b.x - a.x - 1;
                        next_step_counts = false;
                    } else {
                        next_step_counts = true;
                    }
                }
            }
        }
        acc += cacc;
    }

    return acc;
}

fn cmds_to_lines(cmds: Vec<Command>) -> (Vec<HorizontalLine>, Vec<VerticalLine>) {
    let mut cpos = (0 as i64, 0 as i64);
    let mut verts = vec![];
    let mut horiz = vec![];

    for i in 0..cmds.len() {
        let c = cmds[i];
        let next_cmd = cmds[i + 1 % cmds.len()];
        let prev_cmd = cmds[(i + cmds.len() - 1) % cmds.len()];
        let old_pos = cpos.clone();
        let dist = c.distance as i64;
        match c.direction {
            'U' => {
                cpos.1 += dist;
                if dist > 1 {
                    verts.push(VerticalLine {
                        top: cpos.1,
                        bot: old_pos.1,
                        x: cpos.0,
                        bot_dir: Direction::from_char(prev_cmd.direction),
                        top_dir: Direction::from_char(next_cmd.direction),
                    })
                }
            }
            'D' => {
                cpos.1 -= dist;
                if dist > 1 {
                    verts.push(VerticalLine {
                        top: old_pos.1,
                        bot: cpos.1,
                        x: cpos.0,
                        bot_dir: Direction::from_char(next_cmd.direction),
                        top_dir: Direction::from_char(prev_cmd.direction),
                    })
                }
            }
            'L' => {
                cpos.0 -= dist;
                horiz.push(HorizontalLine { length: dist - 1 })
            }
            'R' => {
                cpos.0 += dist;
                horiz.push(HorizontalLine { length: dist - 1 })
            }
            _ => (),
        }
    }

    return (horiz, verts);
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

/*
 Tests
*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::string_to_lines;

    #[test]
    fn part1_test() {
        let string_input = "R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)\nR 2 (#59c680)\nD 2 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)\nL 1 (#1b58a2)\nU 2 (#caa171)\nR 2 (#7807d2)\nU 3 (#a77fa3)\nL 2 (#015232)\nU 2 (#7a21e3)".to_string();
        let line_input = string_to_lines(&string_input);

        let result = part1(line_input);

        assert_eq!(result, 62);
    }

    #[test]
    fn part1_test_by_hand() {
        let string_input = "R 9 (#70c710)\nD 3 (#0dc571)\nL 3 (#5713f0)\nD 3 (#d2c081)\nL 3 (#59c680)\nU 3 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)\nL 1 (#1b58a2)\nU 2 (#caa171)\nR 2 (#7807d2)\nU 3 (#a77fa3)\nL 2 (#015232)\nU 2 (#7a21e3)".to_string();
        let line_input = string_to_lines(&string_input);

        let result = part1(line_input);

        assert_eq!(result, 62);
    }

    // #[test]
    // fn part2_test() {
    //     let string_input = "R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)\nR 2 (#59c680)\nD 2 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)\nL 1 (#1b58a2)\nU 2 (#caa171)\nR 2 (#7807d2)\nU 3 (#a77fa3)\nL 2 (#015232)\nU 2 (#7a21e3)".to_string();
    //     let line_input = string_to_lines(&string_input);

    //     let result = part2(line_input);

    //     assert_eq!(result, 952408144115);
    // }
}
