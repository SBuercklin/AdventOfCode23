use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Universe {
    rows: u64,
    cols: u64,
    galaxies: Vec<Galaxy>,
}

impl Universe {
    fn from_array_arrays(mut v: Vec<Vec<Galaxy>>, rows: u64, cols: u64) -> Universe {
        let mut galaxies: Vec<Galaxy> = vec![];
        for gs in v.iter_mut() {
            galaxies.append(gs);
        }
        return Universe {
            rows,
            cols,
            galaxies,
        };
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Galaxy {
    row: u64,
    col: u64,
}

impl Galaxy {
    fn mdist(&self, other: &Self) -> u64 {
        let retval = self.row.abs_diff(other.row) + self.col.abs_diff(other.col);

        return retval;
    }
}

pub fn part1(lines: Vec<String>) -> u64 {
    return logic(lines, 1);
}
pub fn part2(lines: Vec<String>) -> u64 {
    return logic(lines, 1000000 - 1);
}

pub fn logic(lines: Vec<String>, factor: u64) -> u64 {
    let rows = lines.len() as u64;
    let cols = lines[0].len() as u64;

    let galaxies: Vec<Vec<Galaxy>> = lines
        .iter()
        .enumerate()
        .map(|(row, l)| parse_line(l, row.try_into().unwrap()))
        .collect();

    let universe = Universe::from_array_arrays(galaxies, rows, cols);
    let missing_rows = find_missing_rows(&universe);
    let missing_cols = find_missing_cols(&universe);

    let universe = expand_rows(universe, missing_rows, factor);
    let universe = expand_cols(universe, missing_cols, factor);

    let galaxies = universe.galaxies;
    let mut acc = 0;
    for i in 0..galaxies.len() {
        let g1 = galaxies[i];
        for j in (i + 1)..galaxies.len() {
            let g2 = galaxies[j];
            acc += g1.mdist(&g2);
        }
    }

    return acc;
}

fn expand_rows(mut universe: Universe, miss_rows: HashSet<u64>, factor: u64) -> Universe {
    // Sort in descending order of row
    let mut galaxies = universe.galaxies;
    galaxies.sort_by(|a, b| a.row.cmp(&b.row));

    let mut mrows: Vec<u64> = miss_rows.into_iter().collect();
    mrows.sort();

    mrows.iter().rev().for_each(|mrow| {
        galaxies.iter_mut().rev().for_each(|g| {
            if mrow < &g.row {
                g.row += factor;
            }
        })
    });

    universe.galaxies = galaxies;
    return universe;
}
fn expand_cols(mut universe: Universe, miss_cols: HashSet<u64>, factor: u64) -> Universe {
    // Sort in descending order of row
    let mut galaxies = universe.galaxies;
    galaxies.sort_by(|a, b| a.col.cmp(&b.col));

    let mut mcols: Vec<u64> = miss_cols.into_iter().collect();
    mcols.sort();

    mcols.iter().rev().for_each(|mcol| {
        galaxies.iter_mut().rev().for_each(|g| {
            if mcol < &g.col {
                g.col += factor;
            }
        })
    });

    universe.galaxies = galaxies;
    return universe;
}

fn find_missing_rows(u: &Universe) -> HashSet<u64> {
    let mut h = HashSet::from_iter(0..u.rows);
    for g in &u.galaxies {
        h.remove(&g.row);
    }

    return h;
}
fn find_missing_cols(u: &Universe) -> HashSet<u64> {
    let mut h = HashSet::from_iter(0..u.cols);
    for g in &u.galaxies {
        h.remove(&g.col);
    }

    return h;
}

fn parse_line(l: &str, row: u64) -> Vec<Galaxy> {
    let substrs: Vec<&str> = l.split('#').collect();
    let mut acc = 0;
    let mut galaxies: Vec<Galaxy> = vec![];

    if let Some((_, els)) = substrs.split_last() {
        els.iter().for_each(|s| {
            let l = s.len() as u64;
            acc += l;
            galaxies.push(Galaxy { row, col: acc });
            acc += 1;
        })
    }

    return galaxies;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::string_to_lines;

    #[test]
    fn part1_test() {
        let string_input = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....".to_string();
        let line_input = string_to_lines(&string_input);

        let result = part1(line_input);

        assert_eq!(result, 374);
    }
    #[test]
    fn part2_test() {
        let string_input = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....".to_string();
        let line_input = string_to_lines(&string_input);

        let result = logic(line_input.clone(), 9);
        assert_eq!(result, 1030);

        let result = logic(line_input, 99);
        assert_eq!(result, 8410);
    }
}
