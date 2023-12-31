use std::collections::HashSet;
use std::error;
use std::fmt;
use std::fs::read_to_string;
use std::hash::Hash;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// manual input
    pub input: Option<String>,

    /// path to data
    #[arg(short, long, value_name = "FILE")]
    pub file: Option<PathBuf>,

    /// day number to run
    #[arg(short, long, value_parser=clap::value_parser!(u32).range(1..26))]
    pub day: u32,

    /// problem component (1 or 2)
    #[arg(short, long, value_parser=clap::value_parser!(u32).range(1..3))]
    pub part: u32,
}

pub fn fname_to_string(f: &str) -> String {
    return read_to_string(f).unwrap();
}

pub fn fname_to_lines(f: &str) -> Vec<String> {
    return string_to_lines(&fname_to_string(&f));
}

pub fn string_to_lines(s: &str) -> Vec<String> {
    return s.lines().map(String::from).collect();
}

/*
    Set operations
*/

pub fn combine_sets<T: Eq + std::hash::Hash + Copy>(v: Vec<HashSet<T>>) -> HashSet<T> {
    let mut return_set: HashSet<T> = HashSet::new();
    for set in v {
        for el in set {
            return_set.insert(el);
        }
    }

    return return_set;
}

pub fn intersect_sets<T: Eq + Hash + Copy>(a: HashSet<T>, b: HashSet<T>) -> HashSet<T> {
    let mut c: HashSet<T> = HashSet::new();

    for v in a.iter() {
        if b.contains(v) {
            c.insert(*v);
        }
    }

    return c;
}

/*
   Error types
*/

#[derive(Debug, Clone)]
pub struct NoInputError;

impl fmt::Display for NoInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "No valid input provided");
    }
}

impl error::Error for NoInputError {}

#[derive(Debug, Clone)]
pub struct NotImplementedError;

impl fmt::Display for NotImplementedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Invalid problem input day or part");
    }
}

impl error::Error for NotImplementedError {}
