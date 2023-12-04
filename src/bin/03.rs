use std::collections::HashSet;

advent_of_code::solution!(3);

fn dfs_one(set: &mut HashSet<(i32, i32)>, rows: i32, cols: i32, row: i32, col: i32) {
    if row < 0 || col < 0 || row >= rows || col >= cols {
        return;
    }

    set.insert((row, col));
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input
        .split("\n")
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let rows = lines.len() as i32;
    let cols = lines[0].len() as i32;

    let mut set = HashSet::new();

    let mut total = 0;

    for row in 0..rows as i32 {
        for col in 0..cols as i32 {
            if !lines[row as usize][col as usize].is_digit(10)
                && lines[row as usize][col as usize] != '.'
            {
                dfs_one(&mut set, rows, cols, row - 1, col - 1);
                dfs_one(&mut set, rows, cols, row - 1, col);
                dfs_one(&mut set, rows, cols, row - 1, col + 1);
                dfs_one(&mut set, rows, cols, row, col - 1);
                dfs_one(&mut set, rows, cols, row, col + 1);
                dfs_one(&mut set, rows, cols, row + 1, col - 1);
                dfs_one(&mut set, rows, cols, row + 1, col);
                dfs_one(&mut set, rows, cols, row + 1, col + 1);
            }
        }
    }

    for row in 0..rows as i32 {
        let mut ans = 0;
        let mut add = false;
        for col in 0..cols as i32 {
            if lines[row as usize][col as usize].is_digit(10) {
                ans *= 10;
                ans += lines[row as usize][col as usize].to_digit(10).unwrap();
                if set.contains(&(row, col)) {
                    add = true;
                }
            } else {
                if add {
                    total += ans;
                    add = false;
                }
                ans = 0;
            }
        }
        if add {
            total += ans;
        }
    }

    Some(total)
}

fn dfs_two(
    grid: &Vec<Vec<char>>,
    set: &mut HashSet<(i32, i32)>,
    rows: i32,
    cols: i32,
    row: i32,
    col: i32,
    values: &mut Vec<u32>,
) {
    if row < 0
        || col < 0
        || row >= rows
        || col >= cols
        || set.contains(&(row, col))
        || !grid[row as usize][col as usize].is_digit(10)
    {
        return;
    }

    set.insert((row, col));

    let mut num = grid[row as usize][col as usize].to_digit(10).unwrap();

    let line = &grid[row as usize];

    let mut left_iter = line[..col as usize].iter().rev().enumerate();

    while let Some((idx, x)) = left_iter.next() {
        if !x.is_digit(10) {
            break;
        };

        set.insert((row, col - idx as i32));

        num += u32::pow(10, (idx + 1).try_into().unwrap()) * x.to_digit(10).unwrap();
    }

    let mut right_iter = line[col as usize + 1..].iter().enumerate();

    while let Some((idx, x)) = right_iter.next() {
        if !x.is_digit(10) {
            break;
        };

        set.insert((row, col + idx as i32 + 1));
        num *= 10;
        num += x.to_digit(10).unwrap();
    }

    values.push(num);
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input
        .split("\n")
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let rows = lines.len() as i32;
    let cols = lines[0].len() as i32;

    let mut total = 0;

    for row in 0..rows as i32 {
        for col in 0..cols as i32 {
            if !lines[row as usize][col as usize].is_digit(10)
                && lines[row as usize][col as usize] != '.'
                && lines[row as usize][col as usize] == '*'
            {
                let mut set: HashSet<(i32, i32)> = HashSet::new();

                let mut values = Vec::new();

                dfs_two(&lines, &mut set, rows, cols, row - 1, col - 1, &mut values);
                dfs_two(&lines, &mut set, rows, cols, row - 1, col, &mut values);
                dfs_two(&lines, &mut set, rows, cols, row - 1, col + 1, &mut values);
                dfs_two(&lines, &mut set, rows, cols, row, col - 1, &mut values);
                dfs_two(&lines, &mut set, rows, cols, row, col + 1, &mut values);
                dfs_two(&lines, &mut set, rows, cols, row + 1, col - 1, &mut values);
                dfs_two(&lines, &mut set, rows, cols, row + 1, col, &mut values);
                dfs_two(&lines, &mut set, rows, cols, row + 1, col + 1, &mut values);

                if values.len() == 2 {
                    total += values[0] * values[1];
                }
            }
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
