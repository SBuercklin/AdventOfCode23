#![allow(unused)]

pub fn part1(lines: Vec<String>) -> i64 {
    let input = lines.iter().map(|l| parse_line(l));
    let results = input.map(|invec| {
        let dvecs = get_diff_vecs(&invec);
        return propagate_diffs(dvecs, &invec);
    });

    return results.sum();
}

pub fn part2(lines: Vec<String>) -> i64 {
    let input = lines.iter().map(|l| parse_line(l));
    let results = input.map(|invec| {
        let mut rvec = invec.to_owned();
        rvec.reverse();
        let dvecs = get_diff_vecs(&rvec);
        return propagate_diffs(dvecs, &rvec);
    });

    return results.sum();
}

fn parse_line(l: &str) -> Vec<i64> {
    return l.split(' ').map(|s| s.parse::<i64>().unwrap()).collect();
}

fn propagate_diffs(diffs: Vec<Vec<i64>>, v: &Vec<i64>) -> i64 {
    let mut last: i64 = 0;
    for vec in diffs.iter().rev() {
        last = vec.iter().last().unwrap() + last;
    }
    return v.last().unwrap() + last;
}

fn get_diff_vecs(v: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut cvec = v.to_owned();
    let mut retvecs: Vec<Vec<i64>> = vec![];
    while !vec_const(&cvec) {
        let diff_vec = vec_diff(&cvec);
        retvecs.push(diff_vec.clone());

        cvec = diff_vec;
    }

    return retvecs;
}

fn vec_diff<T: std::ops::Sub<Output = T> + Copy>(v: &Vec<T>) -> Vec<T> {
    let mut prev: T = *v.first().unwrap();
    let mut retvec: Vec<T> = vec![];
    for el in (&v[1..]).iter() {
        let delta = *el - prev;
        prev = *el;
        retvec.push(delta);
    }

    return retvec;
}

fn append_from_diff(v: &mut Vec<i64>, diff: i64) -> &Vec<i64> {
    let last = v.last().unwrap();
    v.push(*last + diff);

    return v;
}

fn vec_zero(v: &Vec<i64>) -> bool {
    vec_eq_const(v, 0)
}

fn vec_const(v: &Vec<i64>) -> bool {
    let first = *v.first().unwrap();
    return vec_eq_const(v, first);
}

fn vec_eq_const(v: &Vec<i64>, c: i64) -> bool {
    for el in (&v[1..]).iter() {
        if !(el.eq(&c)) {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::string_to_lines;

    #[test]
    fn part1_test() {
        let test_input = String::from("0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45");
        let test_lines = string_to_lines(&test_input);

        let result = part1(test_lines);

        assert_eq!(result, 114);
    }
    #[test]
    fn part2_test() {
        let test_input = String::from("0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45");
        let test_lines = string_to_lines(&test_input);

        let result = part2(test_lines);

        assert_eq!(result, 2);
    }
}
