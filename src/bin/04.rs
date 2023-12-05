use std::collections::{HashMap, HashSet};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split("\n").collect::<Vec<&str>>();

    let mut total = 0;

    let mut iter = lines.iter();

    while let Some(line) = iter.next() {
        let (_, rest) = line.split_once(":").unwrap();

        let (left, right) = rest.split_once("|").unwrap();

        let left_nums = left
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let right_nums = right
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let mut map = HashSet::new();

        for left_num in left_nums {
            map.insert(left_num);
        }

        let mut num = 0;

        for right_num in right_nums {
            if let Some(_) = map.get(&right_num) {
                if num == 0 {
                    num += 1
                } else {
                    num *= 2;
                }
            }
        }

        total += num;
    }

    Some(total)
}

pub fn dfs(lines: &Vec<&str>, idx: usize, map: &mut HashMap<usize, i32>) -> i32 {
    let line = lines[idx];

    if let Some(x) = map.get(&idx) {
        return *x;
    }

    let (_, rest) = line.split_once(":").unwrap();

    let (left, right) = rest.split_once("|").unwrap();

    let left_nums = left
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let right_nums = right
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut set = HashSet::new();

    for left_num in left_nums {
        set.insert(left_num);
    }

    let mut num = 0;

    for right_num in right_nums {
        if let Some(_) = set.get(&right_num) {
            num += 1;
        }
    }

    if num == 0 {
        return 1;
    }

    let mut ans = 1;

    for val in (idx + 1)..(idx + 1 + num as usize) {
        ans += dfs(lines, val, map);
    }

    map.insert(idx, ans);

    ans
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split("\n").collect::<Vec<&str>>();

    let mut iter = lines.iter().enumerate();

    let mut map: HashMap<usize, i32> = HashMap::new();

    let mut ans = 0;

    while let Some((idx, _)) = iter.next() {
        ans += dfs(&lines, idx, &mut map);
    }

    Some(ans as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
