use std::iter::zip;

use nom::ParseTo;

pub fn part1(lines: Vec<String>) -> u32 {
    let times = parse_line(&lines[0]);
    let dists = parse_line(&lines[1]);

    return zip(times, dists)
        .map(|(t, l)| {
            let sqrt = (t * t - 4 as f64 * l).sqrt() / 2 as f64;
            let vertex = t / 2 as f64;

            let right = (vertex + sqrt).floor() as u32;
            let left = (vertex - sqrt).ceil() as u32;

            let retval = if sqrt % 1.0 == 0.0 {
                right - left - 1
            } else {
                right - left + 1
            };

            return retval;
        })
        .product();
}

pub fn part2(lines: Vec<String>) -> u32 {
    let t = parse_line_single(&lines[0]);
    let l = parse_line_single(&lines[1]);

    let sqrt = (t * t - 4 as f64 * l).sqrt() / 2 as f64;
    let vertex = t / 2 as f64;

    let right = (vertex + sqrt).floor() as u32;
    let left = (vertex - sqrt).ceil() as u32;

    let retval = if sqrt % 1.0 == 0.0 {
        right - left - 1
    } else {
        right - left + 1
    };

    return retval;
}

fn parse_line(l: &str) -> Vec<f64> {
    let entries: Vec<Option<f64>> = l
        .split(' ')
        .map(|s| s.parse_to())
        .filter(|o| *o != None)
        .collect();
    return entries.into_iter().map(|e| e.unwrap()).collect();
}

// Never ended up using this since I just went with floats in the end
// fn isqrt(x: u32) -> (u32, u32, u32) {
//     let xsqrt = (x as f64).sqrt();
//     if xsqrt.floor() == xsqrt {
//         return (xsqrt as u32, xsqrt as u32 + 1, xsqrt as u32);
//     } else {
//         return (
//             xsqrt.floor() as u32,
//             xsqrt.round() as u32,
//             xsqrt.ceil() as u32,
//         );
//     }
// }

fn parse_line_single(l: &str) -> f64 {
    let entries: Vec<&str> = l.split(' ').filter(|s| !s.is_empty()).collect();
    let entries = entries[1..].to_vec();

    let entry = entries.join("");

    return entry.parse().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::string_to_lines;

    #[test]
    fn part1_test() {
        let test_input: String = String::from("Time:      7  15   30\nDistance:  9  40  200");
        let test_lines = string_to_lines(&test_input);
        let result = part1(test_lines);

        assert_eq!(result, 288);
    }

    #[test]
    fn part2_test() {
        let test_input: String = String::from("Time:      7  15   30\nDistance:  9  40  200");
        let test_lines = string_to_lines(&test_input);
        let result = part1(test_lines);

        assert_eq!(result, 288);
    }
}
