
use std::collections::VecDeque;

#[allow(unused)]
pub fn input() -> (VecDeque<u32>, Vec<usize>) {
    (
    crate::aoc::input_raw(20, 23)
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u32)
        .collect(),
    crate::aoc::input_raw(20, 23)
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect()
    )
}

#[allow(unused)]
pub fn part1(mut game: VecDeque<u32>) -> VecDeque<u32> {
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
        if ins == 0 { ins = s as u32; }
        while !game.contains(&ins) {
            ins -= 1;
            if ins == 0 { ins = s as u32; }
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

#[derive(Copy, Clone, PartialEq, Debug)]
struct LLNode {
    next: usize,
    val: usize
}

fn cup_game_print(game: &Vec<LLNode>) {
    let mut i = 0;
    for _ in 0..game.len() {
        print!("{}", game[i].val);
        i = game[i].next;
    }
    println!("");
}

fn cup_game_init(startseq: &[usize], size: usize) -> Vec<LLNode> {
    if size < startseq.len() {panic!("Game is too small")}
    // Init full game
    let mut game = (0..size)
        .map(|i| LLNode { next: (i+1)%size, val: i+1 })
        .collect::<Vec<LLNode>>();
    // Set first in startseq as next to last
    game[size-1].next = startseq[0]-1;
    // Init start sequence
    for i in 0..startseq.len()-1 {
        game[startseq[i]-1].next = (startseq[i+1] - 1) % size;
    }
    if startseq.len() == size {
        game[(*startseq.last().unwrap()-1)%size].next = startseq[0]-1;
    } else {
        game[(*startseq.last().unwrap()-1)%size].next = startseq.len() % size;
    }
    //cup_game_print(&game);
    game
}

fn cup_game(startseq: &[usize], size: usize, turns: usize) -> Vec<LLNode> {
    let mut game = cup_game_init(startseq, size);
    // Start with first in start sequence
    let mut cur = startseq[0]-1;

    for _ in 0..turns {
        // Buffer three next cups
        let buf = [
            game[cur].next, 
            game[game[cur].next].next, 
            game[game[game[cur].next].next].next
            ];
        // Next element
        let nnext = game[buf[2]].next;
        // Stitch hole
        game[cur].next = nnext;

        // Find destination, highest below cur not in buf
        let mut dest = (cur + (size - 1)) % size;
        while buf.contains(&dest) { dest = (dest + (size - 1)) % size; }
        let dest_end = game[dest].next;

        // Insert buffer
        game[dest].next = buf[0];
        game[buf[2]].next = dest_end;

        // Next cup
        cur = game[cur].next;
    }
    //cup_game_print(&game);
    game
}

#[allow(unused)]
pub fn part2(mut startseq: &[usize]) -> u64 {

    let game = cup_game(startseq, 1000000, 10000000);
    game[game[0].next].val as u64 * game[game[game[0].next].next].val as u64
}


#[test]
fn test_day23_cup_game_init() {
    let seq = vec![2, 3, 4, 1];
    let game = vec![
        LLNode { next: 4, val: 1},        
        LLNode { next: 2, val: 2},
        LLNode { next: 3, val: 3},
        LLNode { next: 0, val: 4},
        LLNode { next: 5, val: 5},
        LLNode { next: 1, val: 6}    
    ];
    assert_eq!(game, cup_game_init(&seq, 6));
    let seq = vec![3,8,9,1,2,5,4,6,7];
    let game = vec![
        LLNode { next: 1, val: 1},        
        LLNode { next: 4, val: 2},
        LLNode { next: 7, val: 3},
        LLNode { next: 5, val: 4},
        LLNode { next: 3, val: 5},
        LLNode { next: 6, val: 6}, 
        LLNode { next: 2, val: 7},
        LLNode { next: 8, val: 8},
        LLNode { next: 0, val: 9},   
    ];
    assert_eq!(game, cup_game_init(&seq, 9));
}

#[test]
fn test_day23_cup_game() {
    let inputs = vec![3,8,9,1,2,5,4,6,7];
    assert_eq!(cup_game_init(&[1,6,7,3,8,4,5,2,9], 9), cup_game(&inputs, 9, 100));
}

#[test]
fn test_day23_part1() {
    let inputs = VecDeque::from(vec![3,8,9,1,2,5,4,6,7]);

    assert_eq!(VecDeque::from(vec![6,7,3,8,4,5,2,9]), part1(inputs));
}

#[test]
fn test_day23_part2() {
    let inputs = [3,8,9,1,2,5,4,6,7];//'1..00'8 9 11 12 13 15 16 17 19 20 21 23 24 25 1 3 4 6 7 2 5 10 14 18 22- 

    assert_eq!(149245887792, part2(&inputs));
}

#[test]
fn run_day23() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . . ");
    let (i1, i2) = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 23 part 1: ");
    println!("{:?} - in {:?}", part1(i1.clone()), pt_start.elapsed().unwrap());
    print!("Day 23 part 2: ");
    let pt_start = SystemTime::now();

    println!("{} - in {:?}", part2(&i2), pt_start.elapsed().unwrap());
    println!("Total duration: {:?}", start.elapsed().unwrap())
}