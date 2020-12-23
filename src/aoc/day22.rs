extern crate either;
use either::Either;
use std::collections::{VecDeque, HashSet};

type Deck = VecDeque<usize>;

#[allow(unused)]
pub fn input() -> (Deck, Deck) {
    parse_input(&crate::aoc::input_raw(22))
}

fn parse_input(inputs: &str) -> (Deck, Deck) {
    let players = inputs.split("\n\n").collect::<Vec<_>>();
    
    (
        players[0].lines()
            .skip(1)
            .map(|x| x.parse::<usize>().unwrap())
            .collect(),
        players[1].lines()
            .skip(1)
            .map(|x| x.parse::<usize>().unwrap())
            .collect()
    )
}

fn war_game(mut p1: Deck, 
            mut p2: Deck) -> Deck {
    while !p1.is_empty() && !p2.is_empty() {
        let x1 = p1.pop_front().unwrap();
        let x2 = p2.pop_front().unwrap();
        if x1 > x2 {
            p1.push_back(x1);
            p1.push_back(x2);
        } else {
            p2.push_back(x2);
            p2.push_back(x1);
        }
    }
    if p1.is_empty() { p2 } else { p1 }
}

enum Player { One, Two }
fn recursive_war_game(mut p1: Deck, 
                    mut p2: Deck, 
                    depth: usize) -> Either<Deck, Player> {
    use Either::{Left, Right};
    use Player::{One, Two};

    let h1 = p1.iter().fold(0, |acc, x| if acc > *x { acc } else { *x });
    let h2 = p2.iter().fold(0, |acc, x| if acc > *x { acc } else { *x });
    if h1 > h2  && depth > 0 { return Right(One) }

    let mut log = HashSet::new();
    while !p1.is_empty() && !p2.is_empty() {
        // if depth < 1 {
        //     println!("({:02}) p1: {:?}\n     p2: {:?}", depth, p1, p2);
        // }
        // Draw cards
        let x1 = p1.pop_front().unwrap();
        let x2 = p2.pop_front().unwrap();

        // Both players have at least as many cards left in
        // their deck as the value of the card, winner of the
        // round is the winner of a recursive game
        if p1.len() >= x1 && p2.len() >= x2 {
            //println!("Recursive game");
            match recursive_war_game(
                p1.clone().into_iter().take(x1).collect(), 
                p2.clone().into_iter().take(x2).collect(), 
                depth+1
            ).right().unwrap() 
            {
                One => {
                    p1.push_back(x1);
                    p1.push_back(x2);
                },
                Two => {
                    p2.push_back(x2);
                    p2.push_back(x1);
                }
            }
        }
        // One ore both players does not have enough cards left for
        // a recursive game, continue with normal round
        else {
            if x1 > x2 {
                p1.push_back(x1);
                p1.push_back(x2);
            } else {
                p2.push_back(x2);
                p2.push_back(x1);
            }
        }
        
        // Avoid infinite recursion, player 1 wins if
        // configuration has been seen previously
        if log.contains(&(p1.clone(), p2.clone())) {
            // TODO: Can happen on depth 0?
            //println!("loop!");
            return Right(One)
        }
        log.insert((p1.clone(), p2.clone()));
    }
    // Winner of game, propagate or return result
    if depth == 0 {
        if p1.is_empty() { Left(p2) } else { Left(p1) }
    } else {
        if p1.is_empty() { Right(Two) } else { Right(One) }
    }
    
}

#[allow(unused)]
pub fn part1(inputs: &(Deck, Deck)) -> usize {
    let winner = war_game(inputs.0.clone(), inputs.1.clone());
    winner
    .iter()
    .enumerate()
    .map(|(i, v)| v*(winner.len()-i) as usize)
    .sum::<usize>() as usize
}

#[allow(unused)]
pub fn part2(inputs: &(Deck, Deck)) -> usize {
    let winner = recursive_war_game(inputs.0.clone(), inputs.1.clone(), 0).left().unwrap();
    winner
    .iter()
    .enumerate()
    .map(|(i, v)| v*(winner.len()-i))
    .sum::<usize>()
}


#[test]
fn test_day22_part1() {
    let inputs = parse_input("Player 1:\n\
    9\n\
    2\n\
    6\n\
    3\n\
    1\n\
    \n\
    Player 2:\n\
    5\n\
    8\n\
    4\n\
    7\n\
    10");

    assert_eq!(306, part1(&inputs));
}

#[test]
fn test_day22_part2() {
    let inputs = parse_input("Player 1:\n\
    9\n\
    2\n\
    6\n\
    3\n\
    1\n\
    \n\
    Player 2:\n\
    5\n\
    8\n\
    4\n\
    7\n\
    10");

    assert_eq!(291, part2(&inputs));
}


#[test]
fn test_hashset() {
    let mut set = HashSet::new();
    let mut p1 = VecDeque::from(vec![1, 2, 3, 4]);
    let mut p2 = VecDeque::from(vec![5, 6, 7, 8]);
    set.insert((p1.clone(), p2.clone()));
    let x = p1.pop_back().unwrap();
    set.insert((p1.clone(), p2.clone()));
    p1.push_back(x);
    assert_eq!(true, set.contains(&(p1.clone(), p2.clone())));
}
#[test]
fn run_day22() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . . ");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 22 part 1: ");
    println!("{} - in {:?}", part1(&inputs), pt_start.elapsed().unwrap());
    print!("Day 22 part 2: ");
    let pt_start = SystemTime::now();
    println!("{} - in {:?}", part2(&inputs), pt_start.elapsed().unwrap());
    println!("Total duration: {:?}", start.elapsed().unwrap())
}