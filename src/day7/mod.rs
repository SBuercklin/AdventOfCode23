use std::collections::hash_map::HashMap;
use std::iter::zip;

type FIVECARDS = [Card; 5];

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Clone)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Ord, PartialEq, PartialOrd, Eq, Hash, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl HandType {
    fn from_set(s: HashMap<Card, u32>) -> HandType {
        let mut s_iter = s.into_iter();

        return match s_iter.len() {
            1 => HandType::FiveKind,
            2 => loop {
                let (_, ct) = s_iter.next().unwrap();
                if ct == 1 || ct == 4 {
                    break HandType::FourKind;
                } else {
                    break HandType::FullHouse;
                };
            },
            3 => {
                let mut max = 2;
                for (_, ct) in s_iter {
                    if ct > max {
                        max = ct;
                    }
                }
                match max {
                    2 => HandType::TwoPair,
                    3 => HandType::ThreeKind,
                    _ => panic!("Incorrect max card occurrence of {}", max),
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            x => panic!("Bad number of cards in hand: {}", x),
        };
    }
}

impl Card {
    fn new(c: char) -> Card {
        match c {
            'W' => Card::Joker,
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Unrecognized card: {}", c),
        }
    }
}

#[derive(Eq, Debug)]
struct Hand {
    cards: FIVECARDS,
    bid: u32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let hand_ord = self.hand_type().cmp(&other.hand_type());
        match hand_ord {
            std::cmp::Ordering::Equal => return card_cmp(&self.cards, &other.cards),
            other => other,
        }
    }
}

fn card_cmp(a: &FIVECARDS, b: &FIVECARDS) -> std::cmp::Ordering {
    for (a, b) in zip(a, b) {
        if a != b {
            return a.cmp(&b);
        }
    }
    return std::cmp::Ordering::Equal;
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        let cmp = self.cmp(other);
        match cmp {
            std::cmp::Ordering::Equal => true,
            _ => false,
        }
    }
}

impl Hand {
    fn new(cards: FIVECARDS, bid: u32) -> Hand {
        return Hand { cards, bid };
    }
    fn hand_type(&self) -> HandType {
        let mut hand_set = unique(&self.cards);
        replace_jokers(&mut hand_set);
        return HandType::from_set(hand_set);
    }
}

fn unique(cards: &FIVECARDS) -> HashMap<Card, u32> {
    let mut hm: HashMap<Card, u32> = HashMap::new();
    for c in cards.iter() {
        let new_val = match hm.get(c) {
            Some(v) => v + 1,
            None => 1,
        };
        hm.insert(c.clone(), new_val);
    }

    return hm;
}

fn maximal_element(cardset: &HashMap<Card, u32>) -> Card {
    let mut max_count = 0;
    let mut max_card = None;
    for (card, count) in cardset {
        if *count > max_count {
            max_count = *count;
            max_card = Some(card.clone());
        }
    }

    return max_card.unwrap();
}

fn replace_jokers(hand_set: &mut HashMap<Card, u32>) {
    let binding = hand_set.clone();
    let j_count = binding.get(&Card::Joker);
    match j_count {
        None => hand_set,
        Some(5) => hand_set,
        Some(c) => {
            hand_set.remove(&Card::Joker);
            let max_card = maximal_element(hand_set);
            let max_val = hand_set.get(&max_card).unwrap();
            hand_set.insert(max_card, max_val + c);
            hand_set
        }
    };
}

pub fn part1(lines: Vec<String>) -> u32 {
    let mut hands: Vec<Hand> = lines.iter().map(|l| parse_line(l)).collect();
    hands.sort_by(|a, b| a.cmp(&b));

    return hands
        .iter()
        .enumerate()
        .map(|(idx, h)| (idx + 1) as u32 * h.bid)
        .sum();
}

pub fn part2(lines: Vec<String>) -> u32 {
    let mut hands: Vec<Hand> = lines.iter().map(|l| parse_line_joker(l)).collect();
    hands.sort_by(|a, b| a.cmp(&b));

    return hands
        .iter()
        .enumerate()
        .map(|(idx, h)| (idx + 1) as u32 * h.bid)
        .sum();
}

fn parse_line(l: &str) -> Hand {
    let split_l: Vec<&str> = l.split(' ').collect();

    let hand: FIVECARDS = split_l[0]
        .chars()
        .into_iter()
        .map(|c| Card::new(c))
        .collect::<Vec<Card>>()
        .try_into()
        .unwrap();

    let bid = split_l[1].parse::<u32>().unwrap();

    return Hand::new(hand.into(), bid);
}

fn parse_line_joker(l: &str) -> Hand {
    return parse_line(&l.to_owned().replace("J", "W"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::string_to_lines;

    #[test]
    fn part1_test() {
        let test_input: String =
            String::from("32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483");
        let test_lines = string_to_lines(&test_input);
        let result = part1(test_lines);

        assert_eq!(result, 6440);
    }

    #[test]
    fn part2_test() {
        let test_input: String =
            String::from("32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483");
        let test_lines = string_to_lines(&test_input);
        let result = part2(test_lines);

        assert_eq!(result, 5905);
    }
}
