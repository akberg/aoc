
use std::collections::VecDeque;

#[allow(unused)]
pub fn input() -> VecDeque<u64> {
    crate::aoc::input_raw(23)
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .collect()
}

#[allow(unused)]
pub fn part1(mut game: VecDeque<u64>) -> VecDeque<u64> {
    let s = game.len();
    let mut index = 0;
    for i in 0..100 {
        //println!("\ngame: {:?}", game);
        //let mut index = i % s;
        let mut cur = game[index];
        //println!("current: {} at {}, pick from {}, {}, {}", cur, index, (index+1)%s, ((index+1)%s)%(s-1), (((index+1)%s)%(s-1))%(s-2));
        let mut buf = VecDeque::new();
        buf.push_back(game.remove((index+1)%s).unwrap());
        buf.push_back(game.remove(((index+1)%s)%(s-1)).unwrap());
        buf.push_back(game.remove((((index+1)%s)%(s-1))%(s-2)).unwrap());
        //println!("buffer: {:?}", buf);

        let mut ins = cur - 1;
        if ins == 0 { ins = s as u64; }
        while !game.contains(&ins) {
            ins -= 1;
            if ins == 0 { ins = s as u64; }
        }
        index = (game.iter().position(|x|x==&ins).unwrap() + 1)%(s-2);
        game.insert(index, buf.pop_back().unwrap());
        game.insert(index, buf.pop_back().unwrap());
        game.insert(index, buf.pop_back().unwrap());
        index = (game.iter().position(|x|x==&cur).unwrap() + 1)%s;
    }
    while game[0] != 1 {
        let x = game.pop_front().unwrap();
        game.push_back(x);
    }
    game.pop_front();
    game
}

#[allow(unused)]
pub fn part2(mut game: VecDeque<u64>) -> u64 {
    use std::time::SystemTime;
    let pt_start = SystemTime::now();
    for i in game.len()..10000000 {
        game.push_back(i as u64 + 1);
    }
    let s = game.len();
    let mut index = 0;
    let mut lap = SystemTime::now();
    for i in 0..10000000 {
        if i % 1000 == 0 {
            println!("\nround: {:#8}\tround time: {:?} - total time: {:?}", i, lap.elapsed().unwrap(),  pt_start.elapsed().unwrap());
            lap = SystemTime::now();
        }
        
        
        //
        let mut cur = game[index];
        //println!("current: {} at {}, pick from {}, {}, {}", cur, index, (index+1)%s, ((index+1)%s)%(s-1), (((index+1)%s)%(s-1))%(s-2));
        let mut buf = VecDeque::new();
        buf.push_back(game.remove((index+1)%s).unwrap());
        buf.push_back(game.remove(((index+1)%s)%(s-1)).unwrap());
        buf.push_back(game.remove((((index+1)%s)%(s-1))%(s-2)).unwrap());
        //println!("buffer: {:?}", buf);

        let mut ins = cur - 1;
        if ins == 0 { ins = s as u64; }
        while !game.contains(&ins) {
            ins -= 1;
            if ins == 0 { ins = s as u64; }
        }
        index = (game.iter().position(|x|x==&ins).unwrap() + 1)%(s-2);
        game.insert(index, buf.pop_back().unwrap());
        game.insert(index, buf.pop_back().unwrap());
        game.insert(index, buf.pop_back().unwrap());
        index = (game.iter().position(|x|x==&cur).unwrap() + 1)%s;
    }
    index = game.iter().position(|x|x==&1).unwrap() + 1;
    game[index%s] * game[(index+1)%s]
}


#[test]
fn test_day23_part1() {
    let inputs = VecDeque::from(vec![3,8,9,1,2,5,4,6,7]);

    assert_eq!(VecDeque::from(vec![6,7,3,8,4,5,2,9]), part1(inputs));
}

#[test]
fn test_day23_part2() {
    let inputs = VecDeque::from(vec![3,8,9,1,2,5,4,6,7]);

    assert_eq!(149245887792, part2(inputs));
}

#[test]
fn run_day23() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . . ");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 23 part 1: ");
    println!("{:?} - in {:?}", part1(inputs.clone()), pt_start.elapsed().unwrap());
    print!("Day 23 part 2: ");
    let pt_start = SystemTime::now();
    println!("{} - in {:?}", part2(inputs), pt_start.elapsed().unwrap());
    println!("Total duration: {:?}", start.elapsed().unwrap())
}