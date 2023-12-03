use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use crate::common::*;

use nom::{
    branch::alt,
    bytes::complete::{take, take_till1},
    character::complete::digit1,
    multi::many1,
    IResult,
};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Copy)]
struct Number {
    /// A number with its associated location. Explicitly stores the line, start and ending columns
    ///     as well as the literal value of the number
    value: u32,
    line: usize,
    start_col: usize,
    end_col: usize,
}

impl Number {
    fn new(value: u32, line: usize, start_col: usize, end_col: usize) -> Number {
        return Number {
            value,
            line,
            start_col,
            end_col,
        };
    }
    fn overlap(&self, id: &Identifier) -> bool {
        return (max((id.col - 1) as i32, self.start_col as i32)
            <= min((id.col + 1).try_into().unwrap(), self.end_col as i32))
            && ((id.line as i32 - self.line as i32).abs() <= 1);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Identifier {
    /// A non-numerical character/identifier, stored with its precise line/column location
    value: char,
    line: usize,
    col: usize,
}

impl Identifier {
    fn new(value: char, line: usize, col: usize) -> Identifier {
        return Identifier { value, line, col };
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Copy)]
struct NumberIDPair {
    /// A pair of a Number with the associated Identifier that it's attached to
    number: Number,
    id: Identifier,
}

impl NumberIDPair {
    fn new(number: Number, id: Identifier) -> NumberIDPair {
        return NumberIDPair { number, id };
    }
}

pub fn part1(lines: Vec<String>) -> u32 {
    let results = lines_to_number_id_pairs(lines);
    return results.iter().map(|n| n.number.value).sum();
}

pub fn part2(lines: Vec<String>) -> u32 {
    let results = lines_to_number_id_pairs(lines);

    let mut id_map: HashMap<Identifier, Vec<Number>> = HashMap::new();

    // Construct the hash map mapping the * IDs to the numbers that they touch
    for r in results {
        let id = r.id;
        let num = r.number;
        match r.id.value {
            '*' => {
                if id_map.contains_key(&id) {
                    id_map.get_mut(&id).unwrap().push(num);
                } else {
                    id_map.insert(id, vec![num]);
                }
            }
            _ => (),
        }
    }

    // Find all IDs from the map (implicity * IDs) which touch two values, and compute the result
    return id_map
        .iter()
        .map(|(_, v)| {
            if v.len() == 2 {
                v[0].value * v[1].value
            } else {
                0
            }
        })
        .sum();
}

fn lines_to_number_id_pairs(lines: Vec<String>) -> Vec<NumberIDPair> {
    let (numbers, mut ids): (Vec<_>, Vec<_>) = lines
        .iter()
        .enumerate()
        .map(|(idx, l)| {
            dbg!(&l);
            let result = parse_line(idx, l);
            return match result {
                Ok((_, results)) => results,
                Err(e) => panic!("Encountered error parsing line {}: {:?}", l, e),
            };
        })
        .unzip();

    let ids = ids
        .iter_mut()
        .reduce(|a, b| {
            a.append(b);
            a
        })
        .unwrap()
        .to_vec();

    let results = populate_number_id_pairs(numbers, ids);

    return results;
}

fn populate_number_id_pairs(
    numbers: Vec<HashSet<Number>>,
    ids: Vec<Identifier>,
) -> Vec<NumberIDPair> {
    // Given a list of sets of Numbers, one for each line, as well as the
    //     list of Indentifiers, compute the list of all Number-Identifier pairs
    let mut combined_lines: HashSet<Number> = combine_sets(numbers.into());
    let mut results: Vec<NumberIDPair> = vec![];

    for id in ids.iter() {
        let mut matches: Vec<NumberIDPair> = vec![];
        for cur_n in combined_lines.iter() {
            if cur_n.overlap(id) {
                dbg!(cur_n, id);
                matches.push(NumberIDPair::new(*cur_n, *id));
            }
        }
        for m in matches.iter() {
            combined_lines.remove(&m.number);
        }
        results.append(&mut matches);
    }

    return results;
}

fn parse_line(l_idx: usize, l: &str) -> IResult<&str, (HashSet<Number>, Vec<Identifier>)> {
    // TODO: rewrite this with nom_locate

    // many1 applies the internal alt parser until the line terminates
    // alt applies the parsers inside of it in order until once matches:
    //      take_till1(...) matches until we encounter a non-empty/period slot (the predicate)
    //      digit1 matches as many digits as it can, with a minimum of 1
    //      take(1usize) takes a single byte. This is a standing for anychar, since we want a &str, not a char
    // This parser parses the line as &strs into one of 3 string classes: "...", "1234", "*"
    let (rem, parsed): (&str, Vec<&str>) =
        many1(alt((take_till1(|c| c != '.'), digit1, take(1usize))))(l)?;

    let mut numbers: HashSet<Number> = HashSet::new();
    let mut ids: Vec<Identifier> = vec![];

    let mut col: usize = 0;

    for substr in parsed.iter() {
        let slen = substr.len();

        /*
         Default case, we parsed a number
         Otherwise, if we parsed a single character, and distinguish the character from a period
        */
        if let Ok(n) = substr.parse::<u32>() {
            let new_number = Number::new(n, l_idx, col, col + substr.len() - 1);
            numbers.insert(new_number);
        } else if slen == 1 {
            match substr.chars().next().unwrap() {
                // If we parsed a single character, it's either a period or an identifier
                '.' => (),
                c => {
                    let new_id = Identifier::new(c, l_idx, col);
                    ids.push(new_id);
                }
            }
        }
        // Increment the column counter by length of the current substring, since we're tracking that manually
        // nom_locate may be able to fix this easier
        col += slen
    }

    return Ok((rem, (numbers, ids)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::string_to_lines;

    #[test]
    fn part1_test() -> () {
        let test_input: String = String::from("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..");
        let test_lines = string_to_lines(&test_input);
        let result = part1(test_lines);

        assert_eq!(result, 4361);
    }

    #[test]
    fn part2_test() -> () {
        let test_input: String = String::from("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..");
        let test_lines = string_to_lines(&test_input);
        let result = part2(test_lines);

        assert_eq!(result, 467835);
    }
}
