#![allow(unused)]

use crate::matrix::AoCMatrix;

use std::cmp::{max, min};
use std::iter::zip;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Square {
    Ash,
    Rock,
}

impl Square {
    fn new(c: char) -> Square {
        match c {
            '#' => Square::Rock,
            '.' => Square::Ash,
            _ => panic!("Unrecognized character: {}", c),
        }
    }
}

pub fn part1(lines: Vec<String>) -> u64 {
    let a = input_to_mats(&lines);
    return a
        .into_iter()
        .map(|mat| reflect_and_check_cols(&mat, 0) + 100 * reflect_and_check_rows(&mat, 0))
        .sum();
}

pub fn part2(lines: Vec<String>) -> u64 {
    let a = input_to_mats(&lines);
    return a
        .into_iter()
        .map(|mat| reflect_and_check_cols(&mat, 1) + 100 * reflect_and_check_rows(&mat, 1))
        .sum();
}

fn reverse_and_check_vectors(vs: Vec<Vec<&Square>>, tgt: u64) -> u64 {
    let l = vs.len();

    for r in 1..l {
        let delta = min(r, l - r);
        let before = vs[(r - delta)..r].iter().rev();
        let after = &vs[r..(r + delta)];

        if zip(before, after)
            .map(|(a, b)| count_diff_vec(a.to_vec(), b.to_vec()))
            .sum::<u64>()
            .eq(&tgt)
        {
            return r.try_into().unwrap();
        }
    }

    return 0;
}

fn reflect_and_check_rows(m: &AoCMatrix<Square>, tgt: u64) -> u64 {
    let rows = m.rows();
    return reverse_and_check_vectors(rows, tgt);
}

fn reflect_and_check_cols(m: &AoCMatrix<Square>, tgt: u64) -> u64 {
    let cols = m.cols();
    return reverse_and_check_vectors(cols, tgt);
}

fn compare_vec_eq<T: Eq>(a: Vec<&T>, b: Vec<&T>) -> bool {
    return count_diff_vec(a, b).eq(&0);
}

fn count_diff_vec<T: Eq>(a: Vec<&T>, b: Vec<&T>) -> u64 {
    return zip(a, b)
        .into_iter()
        .map(|(a, b)| if a.eq(b) { 0 } else { 1 })
        .sum();
}

fn input_to_mats(lines: &Vec<String>) -> impl Iterator<Item = AoCMatrix<Square>> + '_ {
    return lines.split(|s| s.is_empty()).map(|lines| {
        let square_vecs: Vec<Vec<Square>> = lines
            .iter()
            .map(|line| {
                line.chars()
                    .into_iter()
                    .map(Square::new)
                    .collect::<Vec<Square>>()
            })
            .collect();
        AoCMatrix::from_rows(square_vecs)
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::string_to_lines;

    #[test]
    fn part1_test() {
        let string_input = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.\n\n#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#"
                .to_string();
        let line_input = string_to_lines(&string_input);

        let result = part1(line_input);

        assert_eq!(result, 405);
    }

    #[test]
    fn part2_test() {
        let string_input = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.\n\n#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#"
                .to_string();
        let line_input = string_to_lines(&string_input);

        let result = part2(line_input);

        assert_eq!(result, 400);
    }
}
