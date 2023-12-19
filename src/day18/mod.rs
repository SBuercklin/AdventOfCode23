#![allow(unused)]

use std::cmp::max;
use std::collections::HashSet;
use std::fmt::Display;
use std::iter::zip;

use crate::matrix::AoCMatrix;

use hex;
use nom::InputIter;

type BOARD = AoCMatrix<Square>;
type POSITION = (i64, i64);

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
    distance: i64,
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
    return solve(&cmds);
}

pub fn part2(lines: Vec<String>) -> i64 {
    let cmds = lines.iter().map(parse_line_color_command).collect();
    return solve(&cmds);
}

/*
    Let's try shoelace again
*/

fn solve(cmds: &Vec<Command>) -> i64 {
    let (pts, l) = pts_from_cmds(cmds);
    let area = shoelace(pts);

    // UNDERSTANDING EACH TERM:
    // Assume that shoelace traces out the **bottom right** vertex of each square.
    //      This means that any square which is ABOVE or LEFT of the current movement
    //      is not INSIDE the border being traced, but anything BELOW or RIGHT of the
    //      current movement is.
    //
    //      Since half of our movement has to be DOWN and RIGHT, and half has to be UP
    //      and LEFT, we only need to add HALF of the length, since the shoelace catches
    //      the other half.
    //
    //      Plus 1 accounts for first square
    return area + l / 2 + 1;
}

fn pts_from_cmds(cmds: &Vec<Command>) -> (Vec<POSITION>, i64) {
    let mut cpos = (0, 0);
    let mut pvec = vec![cpos];
    let mut l_acc = 0;
    for c in cmds.iter() {
        let d = c.distance;
        l_acc += d;
        cpos = match c.direction {
            'U' => (cpos.0, cpos.1 + d),
            'D' => (cpos.0, cpos.1 - d),
            'L' => (cpos.0 - d, cpos.1),
            'R' => (cpos.0 + d, cpos.1),
            _ => panic!(),
        };
        pvec.push(cpos);
    }

    return (pvec, l_acc);
}

fn shoelace(pts: Vec<POSITION>) -> i64 {
    let mut area_acc = 0;
    for i in 0..pts.len() - 1 {
        let a = pts[i];
        let b = pts[i + 1];
        area_acc += cross_det(a, b);
    }

    return area_acc.abs() / 2;
}

fn cross_det(a: POSITION, b: POSITION) -> i64 {
    a.0 * b.1 - b.0 * a.1
}

fn length(a: POSITION, b: POSITION) -> i64 {
    return (b.0 - a.0).abs() + (a.1 - b.1).abs();
}

/*
 Parsing Input
*/

fn parse_line(l: &String) -> Command {
    let mut pieces = l.split(' ');
    let dir = pieces.next().unwrap().chars().next().unwrap();
    let count: i64 = pieces.next().unwrap().parse().unwrap();

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

    let count = i64::from_str_radix(&color_str[2..7], 16).unwrap();

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
    fn part2_test() {
        let string_input = "R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)\nR 2 (#59c680)\nD 2 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)\nL 1 (#1b58a2)\nU 2 (#caa171)\nR 2 (#7807d2)\nU 3 (#a77fa3)\nL 2 (#015232)\nU 2 (#7a21e3)".to_string();
        let line_input = string_to_lines(&string_input);

        let result = part2(line_input);

        assert_eq!(result, 952408144115);
    }
}
