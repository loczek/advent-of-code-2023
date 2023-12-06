use std::ops::Range;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<i64> {
    let sections = input.split("\n\n").collect::<Vec<_>>();

    let (_, raw_seeds) = sections[0].split_once(": ").unwrap();

    let seeds = raw_seeds
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let raw_maps = sections.iter().skip(1).enumerate();

    let mut maps: Vec<Vec<(std::ops::Range<i64>, i64)>> = vec![vec![]; 7];

    for (idx, map) in raw_maps {
        let map_lines = map.split("\n").collect::<Vec<_>>();

        let map_num_iter = map_lines.iter().skip(1);

        for num_str in map_num_iter {
            let nums = num_str
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            let dst_range_start = nums[0];
            let src_range_start = nums[1];
            let range_len = nums[2];

            maps.get_mut(idx).unwrap().push((
                Range {
                    start: src_range_start,
                    end: src_range_start + range_len,
                },
                dst_range_start - src_range_start,
            ));
        }

        let map = maps.get_mut(idx).unwrap();

        map.sort_by(|a, b| a.0.start.cmp(&b.0.start));
    }

    let mut ans = i64::MAX;

    for seed in seeds {
        let iter = maps.iter();

        let mut val = seed;

        for v in iter {
            let mut vec_iter = v.iter();
            while let Some((range, diff)) = vec_iter.next() {
                if range.contains(&val) {
                    val = val + diff;
                    break;
                }
            }
        }

        ans = ans.min(val);
    }

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }
}
