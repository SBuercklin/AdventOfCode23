#![allow(unused)]

use std::error;
use std::fs::read_to_string;

use clap::Parser;

use advent_of_code_23::{
    common::{Cli, NoInputError, NotImplementedError},
    *,
};

fn main() -> Result<(), Box<dyn error::Error>> {
    let cli = Cli::parse();

    // TODO: Probably better way to handle the "no input" than the cascaded Nones here
    let input = if let Some(p) = cli.file {
        match read_to_string(p) {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    } else if let Some(cli_input) = cli.input {
        Some(cli_input)
    } else {
        None
    };

    let day = cli.day;
    let part = cli.part;

    let result: Box<dyn std::fmt::Display> = match input {
        Some(instring) => entry(instring, day, part)?,
        None => return Err(NoInputError.into()),
    };

    println!("Answer for day {}, part {}", day, part);
    println!("{}", result);

    return Ok(());
}

fn entry(
    instr: String,
    day: u32,
    part: u32,
) -> Result<Box<dyn std::fmt::Display>, Box<dyn error::Error>> {
    let lines = common::string_to_lines(&instr);

    // TODO: Is there a better way of structuring this?
    let result: Box<dyn std::fmt::Display> = match day {
        1 => Box::new(match part {
            1 => day1::part1(lines),
            2 => day1::part2(lines),
            _ => return Err(NotImplementedError.into()),
        }),
        2 => Box::new(match part {
            1 => day2::part1(lines),
            2 => day2::part2(lines),
            _ => return Err(NotImplementedError.into()),
        }),
        3 => Box::new(match part {
            1 => day3::part1(lines),
            2 => day3::part2(lines),
            _ => return Err(NotImplementedError.into()),
        }),
        4 => Box::new(match part {
            1 => day4::part1(lines),
            2 => day4::part2(lines),
            _ => return Err(NotImplementedError.into()),
        }),
        5 => Box::new(match part {
            1 => day5::part1(lines),
            2 => day5::part2(lines),
            _ => return Err(NotImplementedError.into()),
        }),
        6 => Box::new(match part {
            1 => day6::part1(lines),
            2 => day6::part2(lines),
            _ => return Err(NotImplementedError.into()),
        }),
        7 => Box::new(match part {
            1 => day7::part1(lines),
            2 => day7::part2(lines),
            _ => return Err(NotImplementedError.into()),
        }),
        8 => Box::new(match part {
            1 => day8::part1(lines),
            2 => day8::part2(lines),
            _ => return Err(NotImplementedError.into()),
        }),
        9 => Box::new(match part {
            1 => day9::part1(lines),
            2 => day9::part2(lines),
            _ => return Err(NotImplementedError.into()),
        }),
        10 => Box::new(match part {
            1 => day10::part1(lines),
            2 => day10::part2(lines),
            _ => return Err(NotImplementedError.into()),
        }),
        11 => Box::new(match part {
            1 => day11::part1(lines),
            2 => day11::part2(lines),
            _ => return Err(NotImplementedError.into()),
        }),
        12 => Box::new(match part {
            1 => day12::part1(lines),
            2 => day12::part2(lines),
            _ => return Err(NotImplementedError.into()),
        }),
        13 => Box::new(match part {
            1 => day13::part1(lines),
            2 => day13::part2(lines),
            _ => return Err(NotImplementedError.into()),
        }),
        14 => Box::new(match part {
            1 => day14::part1(lines),
            2 => day14::part2(lines),
            _ => return Err(NotImplementedError.into()),
        }),
        15 => Box::new(match part {
            1 => day15::part1(lines),
            2 => day15::part2(lines),
            _ => return Err(NotImplementedError.into()),
        }),
        16 => Box::new(match part {
            1 => day16::part1(lines),
            2 => day16::part2(lines),
            _ => return Err(NotImplementedError.into()),
        }),
        17 => Box::new(match part {
            1 => day17::part1(lines),
            2 => day17::part2(lines),
            _ => return Err(NotImplementedError.into()),
        }),
        _ => return Err(NotImplementedError.into()),
    };

    return Ok(result);
}
