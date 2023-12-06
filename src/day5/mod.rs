use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    multi::many1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

use crate::half_interval::HalfInterval;

#[derive(Debug, Clone, Copy)]
struct SeedRange {
    dest: HalfInterval,
    source: HalfInterval,
    delta: i32,
}

impl SeedRange {
    fn new(dest: u32, source: u32, range: u32) -> SeedRange {
        return SeedRange {
            dest: HalfInterval::from_lb_length(dest, range),
            source: HalfInterval::from_lb_length(source, range),
            delta: (dest as i32 - source as i32),
        };
    }
    fn destination(
        &self,
        int: HalfInterval,
    ) -> (
        Option<HalfInterval>,
        Option<HalfInterval>,
        Option<HalfInterval>,
    ) {
        let center = self.source.intersect(&int);
        let (left, right) = int.diff(&self.source);

        return (left, self.shift_interval_by_delta(center), right);
    }
    fn shift_interval_by_delta(&self, int: Option<HalfInterval>) -> Option<HalfInterval> {
        return match int {
            Some(int) => Some(int.shift(self.delta)),
            None => None,
        };
    }
}

#[derive(Debug, Clone)]
struct SeedMap {
    from: String,
    to: String,
    ranges: Vec<SeedRange>,
}

impl SeedMap {
    fn new(from: String, to: String, ranges: Vec<SeedRange>) -> SeedMap {
        let mut ranges = ranges.to_owned();
        return SeedMap { from, to, ranges };
    }
    fn find_destinations(&self, src: HalfInterval) -> Vec<HalfInterval> {
        let mut intervals_to_iterate = vec![src];
        let mut finished_intervals: Vec<HalfInterval> = vec![];

        dbg!(&self.ranges);

        for range in self.ranges.iter() {
            dbg!(&range);
            let local_intervals_to_iterate = intervals_to_iterate.clone();
            intervals_to_iterate.clear();

            for int in local_intervals_to_iterate.iter() {
                dbg!(&int);
                let (left, center, right) = range.destination(*int);

                dbg!(&left, &center, &right);
                if let Some(left) = left {
                    // We sorted by the source ranges, so anything to the left is done
                    finished_intervals.push(left);
                };
                if let Some(center) = center {
                    // Anything that overlapped is sent to its destination
                    finished_intervals.push(center);
                };
                if let Some(right) = right {
                    // Anything else needs to be processed on coming mappings
                    intervals_to_iterate.push(right);
                };
                dbg!(&intervals_to_iterate);
            }
        }

        // At the end, anything that hasn't been mapped maps to itself
        finished_intervals.append(&mut intervals_to_iterate);

        return finished_intervals;
    }
}

pub fn part1(lines: Vec<String>) -> u32 {
    let lines: Vec<&String> = lines.iter().filter(|l| !l.is_empty()).collect();
    let mut line_iter = lines.iter();

    let l = line_iter.next().unwrap();
    let (_, seeds) = parse_seeds_individual(l).unwrap();

    let mut maps = vec![];

    while let Some(l) = line_iter.next() {
        let (_, mut seed_map) = parse_map_title(l).unwrap();
        let ranges = &mut seed_map.ranges;

        while let Some(l) = line_iter
            .peeking_take_while(|l| l.chars().next().unwrap().is_digit(10))
            .next()
        {
            let (_, r) = parse_range(l).unwrap();
            ranges.push(r);
        }

        ranges.sort_by(|r1, r2| r1.source.lb().cmp(&r2.source.lb()));

        maps.push(seed_map);
    }

    let dests = seeds.iter().map(|s| {
        dbg!(&s);
        let mut current_intervals = vec![*s];
        let mut next_loop_intervals = vec![];
        for m in maps.iter() {
            for int in current_intervals.iter() {
                let mut new_ints = m.find_destinations(*int);
                next_loop_intervals.append(&mut new_ints);
            }
            current_intervals = next_loop_intervals.clone();
            next_loop_intervals.clear();
        }
        current_intervals.iter().map(|int| int.lb()).min().unwrap()
    });

    return dests.min().unwrap();
}

pub fn part2(lines: Vec<String>) -> u32 {
    return 1;
}

fn parse_seeds_individual(l: &str) -> IResult<&str, Vec<HalfInterval>> {
    let (rem, digits) = preceded(tag("seeds:"), many1(space_separated))(l)?;

    let parsed_digits = digits
        .into_iter()
        .map(|e| e.parse().unwrap())
        .map(|v| HalfInterval::new(v, v + 1))
        .collect();

    return Ok((rem, parsed_digits));
}

fn parse_map_title(l: &str) -> IResult<&str, SeedMap> {
    let (rem, (from, to)) = separated_pair(alpha1, tag("-to-"), alpha1)(l)?;

    let ranges = vec![];
    return Ok((rem, SeedMap::new(from.to_owned(), to.to_owned(), ranges)));
}

fn parse_range(l: &str) -> IResult<&str, SeedRange> {
    let (rem, (dest, src, range)) = tuple((digit1, space_separated, space_separated))(l)?;

    return Ok((
        rem,
        SeedRange::new(
            dest.parse().unwrap(),
            src.parse().unwrap(),
            range.parse().unwrap(),
        ),
    ));
}

fn space_separated(l: &str) -> IResult<&str, &str> {
    return preceded(tag(" "), digit1)(l);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::string_to_lines;

    #[test]
    fn part1_test() -> () {
        let test_input: String = String::from(
            "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4",
        );
        let test_lines = string_to_lines(&test_input);
        let result = part1(test_lines);

        assert_eq!(result, 35);
    }

    // #[test]
    // fn part2_test() -> () {
    //     let test_input: String = String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
    //     let test_lines = string_to_lines(&test_input);
    //     let result = part2(test_lines);

    //     assert_eq!(result, 30);
    // }
}
