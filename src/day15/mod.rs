use nom::FindToken;
use std::collections::HashMap;

/*
 Entrypoints
*/

pub fn part1(lines: Vec<String>) -> usize {
    let in_str = &lines[0];

    return in_str.split(',').map(my_hash).sum();
}

pub fn part2(lines: Vec<String>) -> usize {
    let in_str = &lines[0];
    let cmds = in_str.split(',').map(Command::new);
    let lens_boxes = fill_lens_boxes(cmds);

    return lens_boxes
        .iter()
        .enumerate()
        .map(|(idx, lb)| lb.score(idx))
        .sum::<usize>();
}

/*
 Types
*/

type LENS<'a> = (&'a str, usize);

#[derive(Debug, Clone)]
enum Command<'a> {
    Insert(&'a str, usize),
    Remove(&'a str),
}

impl Command<'_> {
    fn new(input: &str) -> Command {
        let insertion = input.find_token('=');

        return if insertion {
            let mut splits = input.split('=').into_iter();
            let box_id = splits.next().unwrap();
            let focal_length = splits.next().unwrap().parse().unwrap();
            Command::Insert(box_id, focal_length)
        } else {
            let mut splits = input.split('-');
            let box_id = splits.next().unwrap();
            Command::Remove(box_id)
        };
    }
    fn get_target_idx(&self) -> usize {
        let target_name = match self {
            Command::Insert(tgt, _) => tgt,
            Command::Remove(tgt) => tgt,
        };

        return my_hash(target_name);
    }
}

impl<'a> std::fmt::Display for Command<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Insert(tgt, fl) => write!(f, "Insert: {} Focal Length: {}", tgt, fl),
            Command::Remove(tgt) => write!(f, "Remove: {}", tgt),
        }
    }
}

#[derive(Debug)]
struct LensBox {
    map: HashMap<String, usize>,
    lens_lengths: Vec<usize>,
}

impl LensBox {
    fn new() -> LensBox {
        return LensBox {
            map: HashMap::new(),
            lens_lengths: vec![],
        };
    }
    fn insert<'a>(&'a mut self, l: LENS<'a>) {
        match self.map.get(l.0) {
            Some(idx) => self.lens_lengths[*idx] = l.1,
            None => {
                self.lens_lengths.push(l.1);
                self.map
                    .insert(l.0.to_string(), self.lens_lengths.len() - 1);
            }
        };
    }
    fn remove(&mut self, tgt: &str) {
        let mut update_flag = false;
        let mut update_idx = 0;
        if let Some(idx) = self.map.get(tgt) {
            update_idx = *idx;
            update_flag = true;
        };
        if update_flag {
            self.lens_lengths.remove(update_idx);
            self.map.remove(tgt);
            let keys: Vec<String> = self.map.keys().map(|s| s.clone()).collect();
            for k in keys.into_iter() {
                let v = self.map.get(&k);
                match v {
                    Some(v) => {
                        if *v > update_idx {
                            self.map.insert(k.to_string(), v - 1);
                        }
                    }
                    None => (),
                };
            }
        }
    }
    fn score(&self, box_num: usize) -> usize {
        let score = (box_num + 1)
            * self
                .lens_lengths
                .iter()
                .enumerate()
                .map(|(idx, fl)| (idx + 1) * fl)
                .sum::<usize>();

        return score;
    }
}

/*
 Business logic
*/

pub fn my_hash(token: &str) -> usize {
    return token
        .chars()
        .fold(0, |acc, c| (acc + (c as usize)) * 17 % 256);
}

fn fill_lens_boxes<'a>(cmds: impl Iterator<Item = Command<'a>>) -> Vec<LensBox> {
    let mut boxes: Vec<_> = (0..256).map(|_| LensBox::new()).collect();

    cmds.for_each(|cmd| {
        let tgt = cmd.get_target_idx();
        let target_box = &mut boxes[tgt];
        match cmd {
            Command::Insert(tgt, fl) => {
                target_box.insert((tgt, fl));
                ()
            }
            Command::Remove(tgt) => {
                target_box.remove(tgt);
                ()
            }
        }
    });

    return boxes.into();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::string_to_lines;
    use std::iter::zip;

    #[test]
    fn myhash_test() {
        let string_input = "HASH".to_string();

        assert_eq!(my_hash(&string_input), 52);
    }

    #[test]
    fn myhash_groups() {
        let string_input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string();
        let split_input = string_input.split(',');
        let hash_vals = [30, 253, 97, 47, 14, 180, 9, 197, 48, 214, 231];

        for (string, hash_val) in zip(split_input, hash_vals) {
            assert_eq!(my_hash(string), hash_val);
        }
    }

    #[test]
    fn part1_test() {
        let string_input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string();
        let line_input = string_to_lines(&string_input);

        let result = part1(line_input);

        assert_eq!(result, 1320);
    }

    #[test]
    fn part2_test() {
        let string_input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string();
        let line_input = string_to_lines(&string_input);

        let result = part2(line_input);

        assert_eq!(result, 145);
    }
}
