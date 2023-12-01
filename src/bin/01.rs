use std::{collections::HashMap, hash, ops::Add};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let result = &advent_of_code::template::read_file("inputs", DAY);

    let mut lines = result.split("\n");

    let mut total: u32 = 0;

    while let Some(line) = lines.next() {
        let mut ans = String::new();

        let mut left = line.chars();

        while let Some(letter) = left.next() {
            if letter.is_digit(10) {
                ans.push_str(letter.to_string().as_str());
                break;
            }
        }

        let mut right = line.chars().rev();

        while let Some(letter) = right.next() {
            if letter.is_digit(10) {
                ans.push_str(letter.to_string().as_str());
                break;
            }
        }

        total += ans.parse::<u32>().unwrap();
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = &advent_of_code::template::read_file_part("inputs", DAY, 2);

    let map = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut lines = result.split("\n");

    let mut total: u32 = 0;

    while let Some(line) = lines.next() {
        let mut ans = String::new();

        let mut left = line.chars().enumerate();

        'left: while let Some((idx, letter)) = left.next() {
            if letter.is_digit(10) {
                ans.push_str(letter.to_string().as_str());
                break;
            }

            let x = &line[idx..];

            let mut map_iter = map.iter();

            while let Some((key, val)) = map_iter.next() {
                if x.starts_with(key) {
                    ans.push_str(val);
                    break 'left;
                }
            }
        }

        let mut right = line.chars().rev().enumerate();

        'right: while let Some((idx, letter)) = right.next() {
            if letter.is_digit(10) {
                ans.push_str(letter.to_string().as_str());
                break;
            }

            let x = &line[line.len() - idx - 1..line.len()];

            let mut map_iter = map.iter();

            while let Some((key, val)) = map_iter.next() {
                if x.starts_with(key) {
                    ans.push_str(val);
                    break 'right;
                }
            }
        }

        total += ans.parse::<u32>().unwrap();
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

// let z = letter.parse::<i32>().unwrap();
// if letter.is_digit(10) {
// let x = letter.to_string().as_str();
// first = i32::from();
// }
