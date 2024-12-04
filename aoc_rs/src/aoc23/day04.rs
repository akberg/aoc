static DAY: usize = 04;

fn parse_line(line: &str) -> (Vec<u32>, Vec<u32>, usize) {
    let (first, second) = line.split_once("|").unwrap();
    let (name, winning_numbers) = first.split_once(":").unwrap();
    let card_id = name.split(" ").last().unwrap().parse::<usize>().unwrap();
    let winning_numbers = winning_numbers
        .trim()
        .split(" ")
        .filter_map(|x| x.parse::<u32>().ok())
        .collect::<Vec<_>>();
    let card_numbers = second
        .trim()
        .split(" ")
        .filter_map(|x| x.parse::<u32>().ok())
        .collect::<Vec<_>>();
    (winning_numbers, card_numbers, card_id)
}

pub fn input() -> Vec<(Vec<u32>, Vec<u32>, usize)> {
    crate::aoc::input_raw(super::YEAR, DAY)
        .lines()
        .map(parse_line)
        .collect::<_>()
}

pub fn part1(inputs: &Vec<(Vec<u32>, Vec<u32>, usize)>) -> u32 {
    inputs
        .iter()
        .map(|(win, mine, _)| {
            let c = mine.iter().filter(|num| win.contains(num)).count() as u32;
            if c > 0 {
                2u32.pow(c - 1)
            } else {
                0u32
            }
        })
        .sum::<u32>()
}

/// Loop until no copies are made, the winning numbers on a card yield that
/// number of copies of the subseqent cards. More efficient would be to count
/// the numbers won of each card, and process it only once. Current solution
/// runs in ~56 seconds.
pub fn part2(inputs: &Vec<(Vec<u32>, Vec<u32>, usize)>) -> u32 {
    let length = inputs.len();
    let cards = inputs;
    let mut card_count = length;
    let mut cards_in = (0..length).collect::<Vec<_>>();
    // println!("cards: {}", card_count);
    for i in 0.. {
        if i % 10000 == 0 {
            println!("{}", i);
        }
        // println!("cards: {}", card_count);
        let mut cards_out = Vec::new();
        cards_in.into_iter().for_each(|i| {
            // let (win, mine, cid) = cards[i];
            let c = cards[i]
                .1
                .iter()
                .filter(|num| cards[i].0.contains(num))
                .count();
            // println!("Card {} wins {}", cid, c);
            for j in 0..c {
                if cards[i].2 + j < length {
                    // println!("{}: {:?}", j+1, cards[j+cid]);
                    cards_out.push(j + cards[i].2);
                }
            }
        });
        if cards_out.len() == 0 {
            break;
        }
        // println!("CARDS OUT:");
        // cards_out.iter().for_each(|c| println!("{:?}", c));
        card_count += cards_out.len();
        cards_in = cards_out;
    }
    card_count as u32
}

#[test]
fn test_parse_line() {
    let inputs = vec![
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    ];
    assert_eq!(
        parse_line(inputs[0]),
        (
            vec![41, 48, 83, 86, 17],
            vec![83, 86, 6, 31, 17, 9, 48, 53],
            1
        )
    );
    assert_eq!(
        parse_line(inputs[1]),
        (
            vec![13, 32, 20, 16, 61],
            vec![61, 30, 68, 82, 17, 32, 24, 19],
            2
        )
    );
}

#[test]
fn test_day4_part1() {
    let inputs = vec![
        parse_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
        parse_line("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
        parse_line("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
        parse_line("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
        parse_line("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
        parse_line("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
    ];
    assert_eq!(13, part1(&inputs));
}

#[test]
fn test_day4_part2() {
    let inputs = vec![
        parse_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
        parse_line("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
        parse_line("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
        parse_line("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
        parse_line("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
        parse_line("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
    ];
    assert_eq!(30, part2(&inputs));
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
