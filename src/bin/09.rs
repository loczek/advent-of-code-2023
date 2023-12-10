advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    let histories = input.split("\n");

    let mut total = 0;

    for history in histories {
        let mut history_total = 0;

        let mut nums = history
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        history_total += nums.last().unwrap();

        while !nums.iter().all(|x: &i32| *x == 0) {
            let windows = nums.windows(2);

            let new_nums = windows.map(|arr| arr[1] - arr[0]).collect::<Vec<_>>();

            history_total += new_nums.last().unwrap();

            nums = new_nums.clone();
        }

        total += history_total;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<i32> {
    let histories = input.split("\n");

    let mut total = 0;

    for history in histories {
        let mut history_total = 0;

        let mut nums_history: Vec<i32> = vec![];

        let mut nums = history
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        nums_history.push(*nums.first().unwrap());

        while !nums.iter().all(|x: &i32| *x == 0) {
            let windows = nums.windows(2);

            let new_nums = windows.map(|arr| arr[1] - arr[0]).collect::<Vec<_>>();

            nums_history.push(*new_nums.first().unwrap());

            nums = new_nums.clone();
        }

        let iter = nums_history.iter().rev();

        for num in iter {
            history_total = num - history_total;
        }

        total += history_total;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
