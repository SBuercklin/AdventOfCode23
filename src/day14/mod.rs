use num::Integer;

use crate::matrix::AoCMatrix;
use std::collections::HashMap;
use std::iter::zip;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Square {
    Round,
    Cube,
    Space,
}

impl Square {
    fn new(c: char) -> Square {
        match c {
            '#' => Self::Cube,
            'O' => Self::Round,
            '.' => Self::Space,
            _ => panic!("Unrecognized character {}", c),
        }
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Cube => '#',
                Self::Round => 'O',
                Self::Space => '.',
            }
        )
    }
}

pub fn part1(lines: Vec<String>) -> u64 {
    let mut starting_square = input_to_mat(&lines);

    move_north(&mut starting_square);

    for col in starting_square.cols().iter() {
        let mut nv = vec![];
        for r in col.iter() {
            nv.push(**r);
        }
    }

    return score_square(&starting_square);
}

pub fn part2(lines: Vec<String>) -> u64 {
    let mut starting_square = input_to_mat(&lines);

    return spin_cycle(&mut starting_square, 1000000000);
}

fn score_square(sq: &AoCMatrix<Square>) -> u64 {
    return sq.cols().iter().map(|c| score(c.to_vec())).sum();
}

fn score(col: Vec<&Square>) -> u64 {
    let l = col.len() as u64;
    return col
        .iter()
        .enumerate()
        .map(|(idx, s)| match s {
            Square::Round => l - idx as u64,
            _ => 0,
        })
        .sum();
}

fn spin_cycle(sq: &mut AoCMatrix<Square>, i: u64) -> u64 {
    let mut cache = HashMap::new();
    cache.insert(sq.to_string(), 0);
    let mut scores = vec![score_square(&sq)];
    let mut iter = 0;

    let (iter_num, offset) = loop {
        move_north(sq);
        move_west(sq);
        move_south(sq);
        move_east(sq);

        iter += 1;
        if let Some(idx) = cache.get(&sq.to_string()) {
            // We've hit a cycle, break out with the index where the cycle starts
            // println!("Found cycle at {}", iter);
            break (iter, idx);
        } else {
            cache.insert(sq.to_string(), iter);
            scores.push(score_square(&sq));
        };
        if iter == i {
            // break (iter, &i);
            return score_square(&sq);
        }
    };

    let cycle_length = iter_num - offset;
    let prefix_removed_length = i - offset;
    let cycle_scores = &scores[(*offset as usize)..];

    let solution_idx = prefix_removed_length.mod_floor(&cycle_length);
    return cycle_scores[solution_idx as usize];
}

fn move_north(sq: &mut AoCMatrix<Square>) {
    sq.cols_mut().iter_mut().for_each(move_to_start);
}

fn move_south(sq: &mut AoCMatrix<Square>) {
    sq.cols_mut().iter_mut().for_each(|c| {
        c.reverse();
        move_to_start(c)
    });
}

fn move_west(sq: &mut AoCMatrix<Square>) {
    sq.rows_mut().iter_mut().for_each(move_to_start);
}

fn move_east(sq: &mut AoCMatrix<Square>) {
    sq.rows_mut().iter_mut().for_each(|r| {
        r.reverse();
        move_to_start(r)
    });
}

fn move_to_start(col: &mut Vec<&mut Square>) -> () {
    let mut new_vec: Vec<Square> = vec![];
    let mut i = 0;
    loop {
        if i >= col.len() {
            break;
        }
        let entry = col[i].clone();

        let next_character = match entry {
            Square::Space => {
                let next_cube = col[i..].iter().position(|s| (**s).eq(&Square::Cube));
                let next_round = col[i..].iter().position(|s| (**s).eq(&Square::Round));

                match next_round {
                    None => Square::Space,
                    Some(round_idx) => match next_cube {
                        None => Square::Round,
                        Some(cube_idx) => {
                            if round_idx < cube_idx {
                                Square::Round
                            } else {
                                entry
                            }
                        }
                    },
                }
            }
            Square::Round => Square::Round,
            Square::Cube => Square::Cube,
        };
        match next_character {
            Square::Round => {
                if next_character != entry {
                    let next_round = col[i..]
                        .iter()
                        .position(|s| (**s).eq(&Square::Round))
                        .unwrap();
                    *col[i + next_round] = Square::Space;
                }
            }
            _ => (),
        };
        new_vec.push(next_character);

        i += 1;
    }

    // Copy the new vector into the old vector
    for (col_sq, new_sq) in zip(col, new_vec) {
        **col_sq = new_sq;
    }
}

fn input_to_mat(lines: &Vec<String>) -> AoCMatrix<Square> {
    let square_vecs: Vec<Vec<Square>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .into_iter()
                .map(Square::new)
                .collect::<Vec<Square>>()
        })
        .collect();
    return AoCMatrix::from_rows(square_vecs);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::string_to_lines;

    #[test]
    fn part1_test() {
        let string_input = "O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#...."
            .to_string();
        let line_input = string_to_lines(&string_input);

        let result = part1(line_input);

        assert_eq!(result, 136);
    }

    #[test]
    fn part2_test() {
        let string_input = "O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#...."
            .to_string();
        let line_input = string_to_lines(&string_input);

        let result = part2(line_input);

        assert_eq!(result, 64);
    }
}
