use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.split("\n");

    let pattern = lines.next().unwrap();

    lines.next();

    let lines = lines.collect::<Vec<_>>();

    let mut map = HashMap::new();

    for line in lines {
        let (from, to) = line.split_once(" = ").unwrap();

        let mut iter = to.chars();
        iter.next();
        iter.next_back();

        let (left, right) = iter.as_str().split_once(", ").unwrap();

        map.insert(from, (left, right));
    }

    let mut curr = "AAA";

    let mut ans = 0;

    let mut iter = pattern.chars().cycle();

    while curr != "ZZZ" {
        let next = iter.next().unwrap();

        let (l, r) = map.get(curr).unwrap();

        if next == 'L' {
            curr = l;
        } else if next == 'R' {
            curr = r;
        }

        ans += 1;
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.split("\n");

    let pattern = lines.next().unwrap();

    lines.next();

    let lines = lines.collect::<Vec<_>>();

    let mut map = HashMap::new();

    for line in lines {
        let (from, to) = line.split_once(" = ").unwrap();

        let mut iter = to.chars();
        iter.next();
        iter.next_back();

        let (left, right) = iter.as_str().split_once(", ").unwrap();

        map.insert(from, (left, right));
    }

    let mut nodes: Vec<&str> = Vec::new();

    for (k, _) in map.iter() {
        if k.ends_with('A') {
            nodes.push(*k);
        }
    }

    let mut ans = vec![0; nodes.len()];

    let asd = nodes.iter_mut().enumerate();

    for (idx, curr) in asd {
        let mut iter = pattern.chars().cycle();

        while curr.chars().last().unwrap() != 'Z' {
            let next = iter.next().unwrap();

            let (l, r) = map.get(curr).unwrap();

            if next == 'L' {
                *curr = l;
            } else if next == 'R' {
                *curr = r;
            }

            ans[idx] += 1;
        }
    }

    Some(ans.iter().fold(1, |acc, curr| acc * curr / gcd(acc, *curr)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}

pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}
