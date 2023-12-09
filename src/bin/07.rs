use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(7);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandKind {
    FiveOfKind = 6,
    FourOfKind = 5,
    FullHouse = 4,
    ThreeOfKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl HandKind {
    fn from(str: &str) -> HandKind {
        let mut map = HashMap::new();

        str.chars().for_each(|char| {
            map.entry(char).and_modify(|x| *x += 1).or_insert(1);
        });

        let x = map.values().collect::<Vec<&i32>>();

        if map.len() == 1 {
            return HandKind::FiveOfKind;
        }
        if x.len() == 2 {
            if x.contains(&&1) && x.contains(&&4) {
                return HandKind::FourOfKind;
            }
            if x.contains(&&2) && x.contains(&&3) {
                return HandKind::FullHouse;
            }
        }
        if x.len() == 3 {
            if x.contains(&&3) {
                return HandKind::ThreeOfKind;
            }
            if x.contains(&&2) && x.contains(&&1) {
                return HandKind::TwoPair;
            }
        }
        if x.len() == 4 {
            return HandKind::OnePair;
        }
        return HandKind::HighCard;
    }

    fn from_joker(str: &str) -> HandKind {
        let mut map = HashMap::new();

        str.chars().for_each(|char| {
            map.entry(char).and_modify(|x| *x += 1).or_insert(1);
        });

        if let Some(num_jokers) = map.get(&'J') {
            if let Some(vals) = map
                .iter()
                .filter(|&(char, _)| char != &'J')
                .max_by(|&(_, a), &(_, b)| a.cmp(b))
            {
                let val = map.get(vals.0).unwrap();

                map.insert(*vals.0, val + num_jokers);

                map.remove(&'J');
            }
        }

        let x = map.values().collect::<Vec<&i32>>();

        if map.len() == 1 {
            return HandKind::FiveOfKind;
        }
        if x.len() == 2 {
            if x.contains(&&1) && x.contains(&&4) {
                return HandKind::FourOfKind;
            }
            if x.contains(&&2) && x.contains(&&3) {
                return HandKind::FullHouse;
            }
        }
        if x.len() == 3 {
            if x.contains(&&3) {
                return HandKind::ThreeOfKind;
            }
            if x.contains(&&2) && x.contains(&&1) {
                return HandKind::TwoPair;
            }
        }
        if x.len() == 4 {
            return HandKind::OnePair;
        }
        return HandKind::HighCard;
    }
}

#[derive(Debug)]
struct Hand {
    cards: String,
    kind: HandKind,
    kind_joker: HandKind,
    bid: u32,
}

impl Hand {
    fn new(str: &str) -> Hand {
        let (card, bid) = str.split_once(" ").unwrap();

        return Hand {
            cards: card.to_string(),
            kind: HandKind::from(card),
            kind_joker: HandKind::from_joker(card),
            bid: bid.parse::<u32>().unwrap(),
        };
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.split("\n").map(Hand::new).collect::<Vec<_>>();

    let types = HashMap::from([
        ('2', 0),
        ('3', 1),
        ('4', 2),
        ('5', 3),
        ('6', 4),
        ('7', 5),
        ('8', 6),
        ('9', 7),
        ('T', 8),
        ('J', 9),
        ('Q', 10),
        ('K', 11),
        ('A', 12),
    ]);

    lines.sort_by(|a, b| {
        if a.kind.cmp(&b.kind) == Ordering::Equal {
            let iter = a.cards.chars().zip(b.cards.chars());

            for (c, d) in iter {
                let left = types.get(&c).unwrap();
                let right = types.get(&d).unwrap();
                if left > right {
                    return core::cmp::Ordering::Greater;
                }
                if left < right {
                    return core::cmp::Ordering::Less;
                }
            }
            return core::cmp::Ordering::Equal;
        } else {
            return a.kind.cmp(&b.kind);
        }
    });

    let mut total: u32 = 0;

    let iter = lines.iter().enumerate();

    for (idx, hand) in iter {
        total += hand.bid * (idx as u32 + 1);
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.split("\n").map(Hand::new).collect::<Vec<_>>();

    let types = HashMap::from([
        ('J', 0),
        ('2', 1),
        ('3', 2),
        ('4', 3),
        ('5', 4),
        ('6', 5),
        ('7', 6),
        ('8', 7),
        ('9', 8),
        ('T', 9),
        ('Q', 10),
        ('K', 11),
        ('A', 12),
    ]);

    lines.sort_by(|a, b| {
        if a.kind_joker.cmp(&b.kind_joker) == Ordering::Equal {
            let iter = a.cards.chars().zip(b.cards.chars());

            for (c, d) in iter {
                let left = types.get(&c).unwrap();
                let right = types.get(&d).unwrap();
                if left > right {
                    return core::cmp::Ordering::Greater;
                }
                if left < right {
                    return core::cmp::Ordering::Less;
                }
            }
            return core::cmp::Ordering::Equal;
        } else {
            return a.kind_joker.cmp(&b.kind_joker);
        }
    });

    let mut total: u32 = 0;

    let iter = lines.iter().enumerate();

    for (idx, hand) in iter {
        total += hand.bid * (idx as u32 + 1);
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
