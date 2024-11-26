use std::cmp::Ordering;

use itertools::Itertools;

static DAY: usize = 07;

pub fn input() -> String {
    crate::aoc::input_raw(super::YEAR, DAY)
    //.lines()
    //.map(|ls| ls.parse::<_>().unwrap())
    //.collect()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    High(u32),
    OnePair(u32),
    TwoPair(u32, u32),
    ThreeOAK(u32),
    FullHouse(u32),
    FourOAK(u32),
    FiveOAK(u32),
}
// impl Rank {
//     pub fn from(count: i32) -> Self
// }

fn card_val_part1(c: char) -> usize {
    if let Some(x) = c.to_digit(10) {
        x as usize
    } else {
        return match c {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => unreachable!(),
        };
    }
}
fn card_val_part2(c: char) -> usize {
    if let Some(x) = c.to_digit(10) {
        x as usize
    } else {
        return match c {
            'T' => 10,
            'J' => 1,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => unreachable!(),
        };
    }
}

/// A simplified poker game (simpler rules, but more difficult because of
/// different rules). Struggled for a while because of a forgotten full house.
pub fn part1(inputs: &str) -> u32 {
    let mut games = inputs
        .lines()
        .map(|line| {
            let (hand, bid) = line.trim().split_once(" ").unwrap();
            let hand = hand.chars().map(card_val_part1).collect::<Vec<_>>();
            let mut parsed = hand
                .iter()
                .fold(vec![0; 13], |mut acc, c| {
                    acc[*c - 2] += 1;
                    acc
                })
                .into_iter()
                .enumerate()
                .map(|(i, c)| (c, i))
                .collect::<Vec<_>>();
            parsed.sort_by(|a, b| b.partial_cmp(a).unwrap());
            let rank = match parsed[0].0 {
                1 => 0,
                2 => {
                    if parsed[1].0 == 2 {
                        2
                    } else {
                        1
                    }
                }
                3 => {
                    if parsed[1].0 == 2 {
                        4
                    } else {
                        3
                    }
                }
                x => x + 1,
            };
            // println!("{:?}", (&hand, bid));
            (rank, hand, bid.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();
    games.sort(); //_by(|a, b| if a.0 > b.0 { Ordering::Greater } else if {})
    games.iter().tuples().for_each(|(prev, game)| {
        println!("{:?} {}", game, if game.1 == prev.1 { "!!!" } else { "" })
    });
    games
        .into_iter()
        .enumerate()
        .map(|(i, (_r, _h, b))| {
            println!("{} * {}", i + 1, b);
            (i as u32 + 1) * b
        })
        .sum::<u32>()
}

/// J is now Jokaer instead of Jack -- it's worth 1, but can masquerade as any
/// card to increase a rank.
pub fn part2(inputs: &str) -> u32 {
    let mut games = inputs
        .lines()
        .map(|line| {
            let (shand, bid) = line.trim().split_once(" ").unwrap();
            let hand = shand.chars().map(card_val_part2).collect::<Vec<_>>();
            let mut parsed = hand
                .iter()
                .fold(vec![0; 14], |mut acc, c| {
                    acc[*c - 1] += 1;
                    acc
                })
                .into_iter()
                .enumerate()
                .map(|(i, c)| (c, i + 1))
                .collect::<Vec<_>>();
            let jokers = parsed[0].0;
            parsed.sort_by(|a, b| b.partial_cmp(a).unwrap());
            let first = if parsed[0].1 == 1 { 1 } else { 0 };
            let second = if parsed[first + 1].1 == 1 {
                first + 2
            } else {
                first + 1
            };
            let rank = match 5.min(parsed[first].0 + if parsed[first].1 > 1 { jokers } else { 0 }) {
                1 => 0,
                2 => {
                    if parsed[second].0 == 2 {
                        2
                    } else {
                        1
                    }
                }
                3 => {
                    if parsed[second].0 == 2 {
                        4
                    } else {
                        3
                    }
                }
                x => x + 1,
            };
            (rank, hand, bid.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();
    games.sort();
    // games.iter().for_each(|game| println!("{:?}", game));
    games
        .into_iter()
        .enumerate()
        .map(|(i, (_r, _h, b))| (i as u32 + 1) * b)
        .sum::<u32>()
}

#[test]
fn test_day7_part1() {
    let inputs = "32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483";
    assert_eq!(6440, part1(inputs));
    let inputs = "74456 5
    82256 6";
    assert_eq!(5 + 2 * 6, part1(inputs));
    let inputs = "33332 5
    2AAAA 6";
    assert_eq!(6 + 2 * 5, part1(inputs));
    let inputs = "77888 5
    77788 6";
    assert_eq!(6 + 2 * 5, part1(inputs));
    let inputs = "2345A 2
    2345J 5
    J345A 3
    32T3K 7
    T55J5 17
    KK677 11
    KTJJT 23
    QQQJA 19
    JJJJJ 29
    JAAAA 37
    AAAAJ 43
    AAAAA 53
    2AAAA 13
    2JJJJ 41
    JJJJ2 31";
    assert_eq!(3542, part1(inputs));
    let inputs = "AAAAA 2
    22222 3
    AAAAK 5
    22223 7
    AAAKK 11
    22233 13
    AAAKQ 17
    22234 19
    AAKKQ 23
    22334 29
    AAKQJ 31
    22345 37
    AKQJT 41
    23456 43";
    assert_eq!(1343, part1(inputs));
}
// 251185752 too high

#[test]
fn test_day7_part2() {
    let inputs = "32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483";
    assert_eq!(5905, part2(inputs));
    let inputs = "2345A 2
    2345J 5
    J345A 3
    32T3K 7
    T55J5 17
    KK677 11
    KTJJT 23
    QQQJA 19
    JJJJJ 29
    JAAAA 37
    AAAAJ 43
    AAAAA 53
    2AAAA 13
    2JJJJ 41
    JJJJ2 31";
    assert_eq!(3667, part2(inputs));
    let inputs = "AAAAA 2
    22222 3
    AAAAK 5
    22223 7
    AAAKK 11
    22233 13
    AAAKQ 17
    22234 19
    AAKKQ 23
    22334 29
    AAKQJ 31
    22345 37
    AKQJT 41
    23456 43";
    assert_eq!(1369, part2(inputs));
}

#[allow(unused)]
pub fn run() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . .");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    let res = part1(&inputs);
    print!("Day {} part 1: ", DAY);
    println!("{}", res);
    println!("Took {:?}", pt_start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    let res = part2(&inputs);
    print!("Day {} part 2: ", DAY);
    println!("{}", res);
    println!("Took {:?}", pt_start.elapsed().unwrap());
    println!("Total time: {:?}", start.elapsed().unwrap());
}
