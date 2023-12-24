use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input
        .split("\n")
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let rows = lines.len();
    let cols = lines[0].len();

    let mut row_offset = vec![0; rows];
    let mut col_offset = vec![0; cols];

    for r in 0..rows {
        let mut all_dots = true;
        for c in 0..cols {
            if lines[r][c] != '.' {
                all_dots = false;
            }
        }
        if all_dots {
            row_offset[r] = row_offset[r - 1] + 1;
        } else if r > 0 {
            row_offset[r] = row_offset[r - 1]
        }
    }

    for c in 0..cols {
        let mut all_dots = true;

        for r in 0..rows {
            if lines[r][c] != '.' {
                all_dots = false;
            }
        }

        if all_dots {
            col_offset[c] = col_offset[c - 1] + 1;
        } else if c > 0 {
            col_offset[c] = col_offset[c - 1]
        }
    }

    let mut map: HashMap<(i64, i64), u64> = HashMap::new();

    let mut count = 1;

    for r in 0..rows {
        for c in 0..cols {
            if lines[r][c] == '#' {
                map.insert((r as i64 + row_offset[r], c as i64 + col_offset[c]), count);
                count += 1;
            }
        }
    }

    let mut ans: HashMap<(u64, u64), u64> = HashMap::new();

    for ((r1, c1), n1) in map.iter() {
        for ((r2, c2), n2) in map.iter() {
            if n1 == n2 || ans.contains_key(&(*n1.min(n2), *n1.max(n2))) {
                continue;
            }

            ans.insert(
                (*n1.min(n2), *n1.max(n2)),
                r1.abs_diff(*r2) + c1.abs_diff(*c2),
            );
        }
    }

    Some(ans.values().map(|x| *x).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input
        .split("\n")
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let rows = lines.len();
    let cols = lines[0].len();

    let mut row_offset = vec![0; rows];
    let mut col_offset = vec![0; cols];

    for r in 0..rows {
        let mut all_dots = true;
        for c in 0..cols {
            if lines[r][c] != '.' {
                all_dots = false;
            }
        }
        if all_dots {
            row_offset[r] = row_offset[r - 1] + 1000000 - 1;
        } else if r > 0 {
            row_offset[r] = row_offset[r - 1]
        }
    }

    for c in 0..cols {
        let mut all_dots = true;

        for r in 0..rows {
            if lines[r][c] != '.' {
                all_dots = false;
            }
        }

        if all_dots {
            col_offset[c] = col_offset[c - 1] + 1000000 - 1;
        } else if c > 0 {
            col_offset[c] = col_offset[c - 1]
        }
    }

    let mut map: HashMap<(i64, i64), u64> = HashMap::new();

    let mut count = 1;

    for r in 0..rows {
        for c in 0..cols {
            if lines[r][c] == '#' {
                map.insert((r as i64 + row_offset[r], c as i64 + col_offset[c]), count);
                count += 1;
            }
        }
    }

    let mut ans: HashMap<(u64, u64), u64> = HashMap::new();

    for ((r1, c1), n1) in map.iter() {
        for ((r2, c2), n2) in map.iter() {
            if n1 == n2 || ans.contains_key(&(*n1.min(n2), *n1.max(n2))) {
                continue;
            }

            ans.insert(
                (*n1.min(n2), *n1.max(n2)),
                r1.abs_diff(*r2) + c1.abs_diff(*c2),
            );
        }
    }

    Some(ans.values().map(|x| *x).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
