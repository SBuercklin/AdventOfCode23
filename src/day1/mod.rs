use regex::Regex;

pub fn part1(lines: Vec<String>) -> u32 {
    let result = lines
        .iter()
        .map(|l| {
            let digits: Vec<u32> = l
                .chars()
                .filter(|c| c.is_numeric())
                .into_iter()
                .map(|c| c.to_digit(10).unwrap())
                .collect();
            let d1 = digits.first().unwrap();
            let d2 = digits.last().unwrap();
            10 * d1 + d2
        })
        .sum::<u32>();

    return result;
}

pub fn part2(lines: Vec<String>) -> u32 {
    let r_forward = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|[0-9]).*").unwrap();
    let r_reverse = Regex::new(r"(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|[0-9]).*").unwrap();

    let result = lines
        .iter()
        .map(|l| {
            let c1 = r_forward.captures(&l).unwrap();
            let temp = reverse_string(l);
            let c2 = r_reverse.captures(&temp).unwrap();

            let d1 = forward_to_digit(&c1[1]);
            let d2 = reverse_to_digit(&c2[1]);

            return 10 * d1 + d2;
        })
        .sum();

    return result;
}

fn forward_to_digit(s: &str) -> u32 {
    if s.len() == 1 {
        let chars: Vec<char> = s.chars().collect();
        return chars[0].to_digit(10).unwrap();
    } else {
        return match s {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            "zero" => 0,
            _ => panic!("Don't know what this number is: {}", s),
        };
    };
}

fn reverse_to_digit(s: &str) -> u32 {
    let local_s = reverse_string(s);
    return forward_to_digit(&local_s);
}

fn reverse_string(s: &str) -> String {
    return s.to_owned().chars().rev().collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::string_to_lines;

    #[test]
    fn part_1() -> () {
        let test_input: String = String::from("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet");
        let test_lines = string_to_lines(&test_input);
        let result = part1(test_lines);

        assert_eq!(result, 142);
    }

    #[test]
    fn part_2() -> () {
        let test_input: String = String::from("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen");
        let test_lines = string_to_lines(&test_input);
        let result = part2(test_lines);

        assert_eq!(result, 281);
    }
}
