#![allow(unused)]

use std::collections::hash_map::HashMap;

use num::integer::lcm;

type MAPID = (char, char, char);
type MAP = HashMap<MAPID, MAPID>;

use nom::{
    bytes::complete::tag,
    character::complete::alpha1,
    sequence::{separated_pair, tuple},
    IResult,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum MapSide {
    Left,
    Right,
}

fn compose(a: &MAP, other: &MAP) -> MAP {
    // Maps keys from a through their output being fed into other
    let mut new_map: MAP = HashMap::new();

    for (k, v) in a {
        let new_v = other.get(&v).unwrap();
        new_map.insert(*k, *new_v);
    }

    return new_map;
}

fn get_composite_map(cmds: Vec<MapSide>, left: MAP, right: MAP) -> MAP {
    let mut cmd_iter = cmds.iter();
    let mut comp_map = match cmd_iter.next().unwrap() {
        MapSide::Left => left.clone(),
        MapSide::Right => right.clone(),
    };

    for cmd in cmd_iter {
        let next_map = match cmd {
            MapSide::Left => left.clone(),
            MapSide::Right => right.clone(),
        };
        comp_map = compose(&comp_map, &next_map);
    }

    return comp_map;
}

pub fn part1(lines: Vec<String>) -> u64 {
    let binding = parse_map_sides(&lines[0]);
    let mut commands = binding.iter().cycle();

    let map_lines = lines[2..]
        .iter()
        .map(|l| parse_map_line(l).unwrap().1)
        .collect();

    let (left_map, right_map, _) = get_maps(map_lines);

    let mut cur_id = ('A', 'A', 'A');
    let mut ct = 0;
    while cur_id != ('Z', 'Z', 'Z') {
        cur_id = match commands.next().unwrap() {
            MapSide::Left => *left_map.get(&cur_id).unwrap(),
            MapSide::Right => *right_map.get(&cur_id).unwrap(),
        };
        ct += 1;
    }

    return ct;
}

pub fn part2(lines: Vec<String>) -> u64 {
    let binding = parse_map_sides(&lines[0]);
    let commands = binding.clone();

    let map_lines = lines[2..]
        .iter()
        .map(|l| parse_map_line(l).unwrap().1)
        .collect();

    let (left_map, right_map, line_ids) = get_maps(map_lines);

    let start_locs: Vec<MAPID> = line_ids
        .clone()
        .into_iter()
        .filter(|(_, _, c)| c.eq(&'A'))
        .collect();

    return start_locs
        .into_iter()
        .map(|loc| find_first_z(loc, &commands, &left_map, &right_map))
        .reduce(|a, b| lcm(a, b))
        .unwrap();

    // start_locs.into_iter().for_each(|loc| {
    //     cycle_n_times(loc, &commands, &left_map, &right_map, 200);
    // });
    // return 1;
}

fn find_first_z(start: MAPID, cmds: &Vec<MapSide>, leftmap: &MAP, rightmap: &MAP) -> u64 {
    let mut cur = start;

    for (i, c) in cmds.iter().cycle().enumerate() {
        cur = match c {
            MapSide::Left => *leftmap.get(&cur).unwrap(),
            MapSide::Right => *rightmap.get(&cur).unwrap(),
        };
        if cur.2 == 'Z' {
            return (i + 1).try_into().unwrap();
        }
    }
    return 0;
}

fn cycle_n_times(
    start: MAPID,
    cmds: &Vec<MapSide>,
    leftmap: &MAP,
    rightmap: &MAP,
    n: usize,
) -> u64 {
    let mut cur = start;

    let mut ctr = 0;
    for iter in 0..n {
        for (i, c) in cmds.iter().enumerate() {
            cur = match c {
                MapSide::Left => *leftmap.get(&cur).unwrap(),
                MapSide::Right => *rightmap.get(&cur).unwrap(),
            };
            ctr += 1;
            if cur.2 == 'Z' {
                println!(
                    "Encountered a Z with {}{}{} during cycle {}, relative location {}, absolute location {}",
                    cur.0, cur.1, cur.2, iter, i + 1, ctr
                    );
            }
        }
    }
    return 0;
}

fn get_maps(lines: Vec<(MAPID, MAPID, MAPID)>) -> (MAP, MAP, Vec<MAPID>) {
    let mut line_ids: Vec<MAPID> = vec![];
    let mut left_ids: Vec<MAPID> = vec![];
    let mut right_ids: Vec<MAPID> = vec![];
    lines.into_iter().for_each(|(l_id, lef_id, r_id)| {
        line_ids.push(l_id);
        left_ids.push(lef_id);
        right_ids.push(r_id);
    });

    let line_ids_clone = line_ids.clone();
    let line_ids_return = line_ids.clone();

    let left_map: MAP = line_ids.into_iter().zip(left_ids).into_iter().collect();
    let right_map: MAP = line_ids_clone
        .into_iter()
        .zip(right_ids)
        .into_iter()
        .collect();

    return (left_map, right_map, line_ids_return);
}

fn parse_map_sides(l: &str) -> Vec<MapSide> {
    let results = l
        .chars()
        .map(|c| match c {
            'L' => Some(MapSide::Left),
            'R' => Some(MapSide::Right),
            _ => None,
        })
        .map(|c| c.unwrap())
        .collect();

    return results;
}

fn parse_map_line(l: &str) -> IResult<&str, (MAPID, MAPID, MAPID)> {
    let (rem, (id, _, (l, r))) = tuple((
        str_to_mapid,
        tag(" = ("),
        separated_pair(str_to_mapid, tag(", "), str_to_mapid),
    ))(l)?;

    return Ok((rem, (id, l, r)));
}

fn str_to_mapid(s: &str) -> IResult<&str, MAPID> {
    let (rem, res) = alpha1(s)?;
    let mut s_iter = res.chars().into_iter();

    let id = (
        s_iter.next().unwrap(),
        s_iter.next().unwrap(),
        s_iter.next().unwrap(),
    );

    return Ok((rem, id));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::string_to_lines;

    #[test]
    fn part1_test() {
        let test_input: String = String::from(
            "RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)",
        );
        let test_lines = string_to_lines(&test_input);
        let result = part1(test_lines);

        assert_eq!(result, 2);
    }
    #[test]
    fn part2_test() {
        let test_input: String = String::from(
            "LR\n\nWWA = (WWB, XXX)\nWWB = (XXX, WWZ)\nWWZ = (WWB, XXX)\nSSA = (SSB, XXX)\nSSB = (SSC, SSC)\nSSC = (SSZ, SSZ)\nSSZ = (SSB, SSB)\nXXX = (XXX, XXX)",
        );
        let test_lines = string_to_lines(&test_input);
        let result = part2(test_lines);

        assert_eq!(result, 6);
    }
}
