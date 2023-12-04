use std::collections::hash_set::HashSet;

use crate::common::intersect_sets;

use itertools::Itertools;

pub fn part1(lines: Vec<String>) -> usize {
    let scores = get_points_cards(lines);
    let result = scores
        .iter()
        .map(|l| {
            if l > &0 {
                (2_usize).pow(*l as u32 - 1 as u32)
            } else {
                0
            }
        })
        .sum();

    return result;
}

pub fn part2(lines: Vec<String>) -> usize {
    let scores = get_points_cards(lines);
    let mut card_counts = vec![0; scores.len()];

    for (i, s) in scores.iter().enumerate() {
        // Apparently we count cards even if they did lose, previously I had
        //  a piece of logic that checked whether the score was positive to decide
        //  whether we increment extras
        card_counts[i] += 1;
        for n in 1..s + 1 {
            card_counts[i + n] += card_counts[i];
        }
    }

    return card_counts.iter().sum();
}

fn get_points_cards(lines: Vec<String>) -> Vec<usize> {
    let result: Vec<usize> = lines
        .iter()
        .map(|l| -> (HashSet<u32>, HashSet<u32>) {
            let split1: &str = l.split(':').collect::<Vec<&str>>()[1];
            let split2: Vec<&str> = split1.split('|').collect::<Vec<&str>>();
            let parsed_data: Vec<Vec<u32>> = split2.iter().map(parse_space_delimited).collect();
            (
                HashSet::from_iter(parsed_data[0].clone()),
                HashSet::from_iter(parsed_data[1].clone()),
            )
        })
        .map(|(a, b)| intersect_sets(a, b).into_iter().try_len().unwrap())
        .collect();

    return result;
}

fn parse_space_delimited(v: &&str) -> Vec<u32> {
    let digits: Vec<&str> = v.split(' ').filter(|s| s.len() > 0).collect();
    let nums: Vec<u32> = digits.iter().map(|s| s.parse().unwrap()).collect();

    return nums;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::string_to_lines;

    #[test]
    fn part1_test() -> () {
        let test_input: String = String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        let test_lines = string_to_lines(&test_input);
        let result = part1(test_lines);

        assert_eq!(result, 13);
    }

    #[test]
    fn part2_test() -> () {
        let test_input: String = String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        let test_lines = string_to_lines(&test_input);
        let result = part2(test_lines);

        assert_eq!(result, 30);
    }
}
