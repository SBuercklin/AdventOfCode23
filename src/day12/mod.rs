#![allow(unused)]

use crate::parsers::{comma_separated, space_separated};
use std::collections::HashSet;

/*
 Entry
*/

pub fn part1(lines: Vec<String>) -> u64 {
    let line_results: Vec<(Vec<Spring>, Vec<u64>)> = lines.iter().map(|l| parse_line(l)).collect();

    let result = solve_problem(line_results);

    return result;
}
pub fn part2(lines: Vec<String>) -> u64 {
    let line_results: Vec<(Vec<Spring>, Vec<u64>)> = lines.iter().map(|l| parse_line(l)).collect();

    let expanded_results = line_results
        .iter()
        .map(|(springs, counts)| {
            let mut msprings = springs.to_owned();
            msprings.push(Spring::Unknown);
            let l = msprings.len();
            let rep_springs: Vec<Spring> = msprings.into_iter().cycle().take(l * 5 - 1).collect();

            let mut mcounts = counts.to_owned();
            let l = mcounts.len();
            let rep_counts: Vec<u64> = mcounts.into_iter().cycle().take(l * 5).collect();
            return (rep_springs, rep_counts);
        })
        .collect();

    let result = solve_problem(expanded_results);

    return 1;
}

/*
 Types
*/

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Spring {
    fn from_char(c: char) -> Spring {
        match c {
            '.' => Spring::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!("Unrecognized character {}", c),
        }
    }
    fn to_char(&self) -> char {
        match self {
            Spring::Operational => '.',
            Spring::Damaged => '#',
            Spring::Unknown => '?',
        }
    }
    fn could_be_broken(&self) -> bool {
        match self {
            Spring::Operational => false,
            _ => true,
        }
    }
    fn could_be_working(&self) -> bool {
        match self {
            Spring::Damaged => false,
            _ => true,
        }
    }
    fn must_be_broken(&self) -> bool {
        !self.could_be_working()
    }
    fn must_be_working(&self) -> bool {
        !self.could_be_broken()
    }
}

/*
    Business logic
*/

fn solve_problem(line_results: Vec<(Vec<Spring>, Vec<u64>)>) -> u64 {
    let line_counts: Vec<u64> = line_results
        .iter()
        .map(|(springs, count)| {
            let mut visisted: HashSet<String> = HashSet::new();
            let prefix: Vec<Spring> = vec![];
            solve_line(springs, count, &prefix)
        })
        .collect();

    // dbg!(&line_counts);

    return line_counts.iter().sum();
}

fn solve_line(springs: &Vec<Spring>, counts: &Vec<u64>, prefix: &Vec<Spring>) -> u64 {
    // debug_springs_counts(springs, counts);
    // debug_springs(prefix);

    // If there are no more streaks to fit, and the springs remaining are all workable, we're good
    // This is the GOOD base case.
    if counts.is_empty() {
        if (springs.is_empty() || springs.iter().all(|s| !s.must_be_broken())) {
            // let mut local = springs.to_owned();
            // let mut lp = prefix.to_owned();
            // lp.append(&mut local);
            // debug_springs(&lp);
            return 1;
        } else {
            return 0;
        }
    };

    let n_counts = counts.len();
    let n_springs = springs.len();

    let c = *counts.iter().next().unwrap() as usize;

    // If the current streak exceeds the length of remaining springs, return 0, it doesn't fit
    if c > n_springs {
        return 0;
    };

    let slice = &springs[0..c];

    let mut acc = 0;
    let streak_fits = slice.iter().all(|s| s.could_be_broken());
    let must_fit = streak_fits && slice[0].must_be_broken();
    if streak_fits {
        // Solve the rest of the line
        if n_counts == 1 {
            // no streaks left, solve the remaining using the base case

            let mut mprefix = springs[0..c].to_vec();
            mprefix.fill_with(|| Spring::Damaged);

            let mut nprefix = prefix.to_owned();
            nprefix.append(&mut mprefix);

            acc += solve_line(&springs[c..].to_vec(), &vec![], &nprefix);
        } else if n_springs >= c + 2 && !springs[c].must_be_broken() {
            // need at least 2 more springs to make a second streak fit, and we need the next
            // spring to be working
            let mut mprefix = springs[0..c].to_vec();

            mprefix.fill(Spring::Damaged);
            mprefix.push(Spring::Operational);
            let mut nprefix = prefix.to_owned();
            nprefix.append(&mut mprefix);

            acc += solve_line(
                &springs[(c + 1)..].to_vec(),
                &counts[1..].to_vec(),
                &nprefix,
            );
        }
    }

    // Even if the streak doesn't fit, we want to increment the spring list and try again
    if n_springs >= c + 1 && !must_fit && !springs[0].must_be_broken() {
        let mut mprefix = springs[0..1].to_vec();
        mprefix.fill_with(|| Spring::Operational);
        let mut nprefix = prefix.to_owned();
        nprefix.append(&mut mprefix);

        acc += solve_line(&springs[1..].to_vec(), counts, &nprefix);
    }
    return acc;
}

fn visualize_springs(springs: &Vec<Spring>) -> String {
    let s = String::from_iter(springs.iter().map(|s| s.to_char()));
    return s;
}

fn debug_springs_counts(springs: &Vec<Spring>, counts: &Vec<u64>) {
    println!("{:?}", counts);
    debug_springs(springs);
}

fn debug_springs(springs: &Vec<Spring>) {
    let s = visualize_springs(springs);
    println!("{:?}", s);
}

/*
 Problem Ingestion
*/

fn parse_line(l: &str) -> (Vec<Spring>, Vec<u64>) {
    let ssres = space_separated(l);

    let split_slin = match ssres {
        Err(e) => panic!("Space Separated errored with: {}", e.to_string()),
        Ok((_, l)) => l,
    };
    let springs = split_slin[0];
    let counts_str = split_slin[1];

    let springs_enums: Vec<Spring> = springs.chars().map(Spring::from_char).collect();

    let csres = comma_separated(counts_str);
    let count_slin = match csres {
        Err(e) => panic!("Space Separated errored with: {}", e.to_string()),
        Ok((_, l)) => l,
    };
    let counts: Vec<u64> = count_slin.iter().map(|s| s.parse().unwrap()).collect();

    return (springs_enums, counts);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::string_to_lines;

    // #[test]
    // fn part1_test() {
    //     let string_input = "???.### 1,1,3\n.??..??...?##. 1,1,3\n?#?#?#?#?#?#?#? 1,3,1,6\n????.#...#... 4,1,1\n????.######..#####. 1,6,5\n?###???????? 3,2,1"
    //         .to_string();
    //     let line_input = string_to_lines(&string_input);

    //     let result = part1(line_input);

    //     assert_eq!(result, 21);
    // }
    #[test]
    fn part1_test_tough_str() {
        let string_input = "??#?#????#..???????? 5,1,4,2".to_string();
        let line_input = string_to_lines(&string_input);

        let result = part1(line_input);

        assert_eq!(result, 9);
    }

    // #[test]
    // fn part2_test() {
    //     let string_input = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....".to_string();
    //     let line_input = string_to_lines(&string_input);

    //     let result = logic(line_input.clone(), 9);
    //     assert_eq!(result, 1030);

    //     let result = logic(line_input, 99);
    //     assert_eq!(result, 8410);
    // }
}
