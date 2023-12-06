use itertools::Itertools;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split("\n").collect::<Vec<_>>();

    let grid = lines
        .iter()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let mut ans = 1;

    for col in 1..grid[0].len() {
        let total_time = grid[0][col].parse::<i32>().unwrap();
        let total_distance = grid[1][col].parse::<i32>().unwrap();

        let mut ways = 0;

        for hold_time in 1..total_time {
            let travel_time = total_time - hold_time;
            let distance = hold_time * travel_time;
            if distance > total_distance {
                ways += 1;
            }
        }

        ans *= ways;
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.split("\n").collect::<Vec<_>>();

    let grid = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .join("")
                .parse::<i64>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let mut ans = 1;

    let total_time = grid[0];
    let total_distance = grid[1];

    let mut ways = 0;

    for hold_time in 1..total_time {
        let travel_time = total_time - hold_time;
        let distance = hold_time * travel_time;
        if distance > total_distance {
            ways += 1;
        }
    }

    ans *= ways;

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u32> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
