use std::collections::HashMap;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split("\n").collect::<Vec<&str>>();

    let mut ans = vec![true; lines.len()];

    let mut iter = lines.iter();

    while let Some(line) = iter.next() {
        let (game, rest) = line.split_once(": ").unwrap();

        let (_, idx_raw) = game.split_once(" ").unwrap();

        let idx = idx_raw.parse::<usize>().unwrap();

        let mut sets = rest.split("; ");

        while let Some(set) = sets.next() {
            let mut map = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

            let mut set_parts = set.split(", ");

            while let Some(part) = set_parts.next() {
                if let Some((num, color)) = part.split_once(" ") {
                    let val = map.get(color).unwrap();
                    map.insert(color, val - num.parse::<i32>().unwrap());
                }
            }

            let mut map_iter = map.iter();

            while let Some((_key, val)) = map_iter.next() {
                if *val < 0 {
                    ans[idx - 1] = false;
                }
            }
        }
    }

    let total: u32 = ans
        .iter()
        .enumerate()
        .filter(|&(_, val)| *val == true)
        .map(|(idx, _)| return idx as u32 + 1)
        .sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split("\n").collect::<Vec<&str>>();

    let mut ans: Vec<u32> = vec![1; lines.len()];

    let mut iter = lines.iter();

    while let Some(line) = iter.next() {
        let (game, rest) = line.split_once(": ").unwrap();

        let (_, idx_raw) = game.split_once(" ").unwrap();

        let idx = idx_raw.parse::<usize>().unwrap();

        let mut sets = rest.split("; ");

        let mut map: HashMap<&str, u32> = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        while let Some(set) = sets.next() {
            let mut set_parts = set.split(", ");

            while let Some(part) = set_parts.next() {
                if let Some((num, color)) = part.split_once(" ") {
                    let val = map.get(color).unwrap();
                    let parsed = num.parse::<u32>().unwrap();
                    map.insert(color, val.clone().max(parsed));
                }
            }
        }

        let mut map_iter = map.iter();

        while let Some((_, val)) = map_iter.next() {
            ans[idx - 1] *= val;
        }
    }

    Some(ans.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
