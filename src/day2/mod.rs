use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, space0, space1},
    sequence::tuple,
    IResult,
};

pub fn part1(lines: Vec<String>) -> u32 {
    let games: Vec<Game> = lines.iter().map(|l| parse_game(l)).collect();

    let max_round = Round {
        red: 12,
        green: 13,
        blue: 14,
    };

    let count_games = games.iter().filter(|g| !g.exceeds_round(&max_round));

    return count_games.map(|g| g.id).sum();
}

pub fn part2(lines: Vec<String>) -> u32 {
    let games = lines.iter().map(|l| parse_game(l));

    let powers = games.map(|g| g.min_round()).map(|r| r.power());

    return powers.sum();
}

#[derive(Clone)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn power(&self) -> u32 {
        return self.red * self.green * self.blue;
    }
}

#[derive(Clone)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn exceeds_round(&self, r: &Round) -> bool {
        return self
            .rounds
            .iter()
            .map(|gameround| {
                gameround.red > r.red || gameround.green > r.green || gameround.blue > r.blue
            })
            .reduce(|a, b| a || b)
            .unwrap();
    }
    fn min_round(&self) -> Round {
        let mut min = Round {
            red: 0,
            green: 0,
            blue: 0,
        };
        for r in self.rounds.iter() {
            if r.red > min.red {
                min.red = r.red
            };
            if r.green > min.green {
                min.green = r.green
            };
            if r.blue > min.blue {
                min.blue = r.blue
            };
        }

        return min;
    }
}

fn parse_game(l: &str) -> Game {
    let game_result = parse_game_prefix(l);

    match game_result {
        Ok((rest, id)) => {
            let rounds = parse_rounds(rest);
            return Game { id, rounds };
        }
        Err(_) => panic!("Unable to parse game id"),
    };
}

fn parse_game_prefix(l: &str) -> IResult<&str, u32> {
    let (rest, (_, id, _)) = tuple((tag("Game "), digit1, tag(": ")))(l)?;

    return Ok((rest, id.parse().unwrap()));
}

fn parse_rounds(l: &str) -> Vec<Round> {
    return l.split(';').map(parse_round).collect();
}

fn parse_round(l: &str) -> Round {
    let color_counts = l.split(", ").map(parse_color_count);

    let mut round = Round {
        red: 0,
        green: 0,
        blue: 0,
    };
    for c in color_counts.into_iter() {
        match c {
            Ok((_, ("red", ct))) => round.red = ct,
            Ok((_, ("green", ct))) => round.green = ct,
            Ok((_, ("blue", ct))) => round.blue = ct,
            _ => panic!("Parsed an unknown color"),
        }
    }

    return round;
}

fn parse_color_count(l: &str) -> IResult<&str, (&str, u32)> {
    let (rest, (_, ct, _, color)) = tuple((space0, digit1, space1, alpha1))(l)?;
    return Ok((rest, (color, ct.parse().unwrap())));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::string_to_lines;

    #[test]
    fn part_1() -> () {
        let test_input: String = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        let test_lines = string_to_lines(&test_input);
        let result = part1(test_lines);

        assert_eq!(result, 8);
    }

    #[test]
    fn part_2() -> () {
        let test_input: String = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        let test_lines = string_to_lines(&test_input);
        let result = part2(test_lines);

        assert_eq!(result, 2286);
    }
}
