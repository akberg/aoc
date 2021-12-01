
use std::convert::From;
use std::cmp;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum State {Taken, Empty, NoSeat}
impl From<char> for State {
    fn from(c: char) -> State {
        match c {
            'L' => Self::Empty,
            '#' => Self::Taken,
            '.'|_ => Self::NoSeat,
        }
    }
}

#[allow(unused)]
pub fn input() -> Vec<Vec<State>> {
    let mut ret = crate::aoc::input_raw(20, 11)
        .lines()
        .map(|line| line
            .chars()
            .map(|c| {match c { 'L' => State::Empty, '#' => State::Taken, _=> State::NoSeat}})
            .collect::<Vec<State>>()
        )
        .collect::<Vec<Vec<State>>>();
    ret
}

fn count_adj(seats: &Vec<Vec<State>>) -> Vec<Vec<i64>> {
    let mut count = vec![vec![0; seats[0].len()]; seats.len()];
    for i in 0..seats.len() as i64 {
        for j in 0..seats[i as usize].len() as i64 {
            if seats[i as usize][j as usize] == State::Taken {
                for x in i-1..i+2 {
                    for y in j-1..j+2 {
                        if x >= 0 && y >= 0 
                        && x < seats.len() as i64
                        && y < seats[i as usize].len() as i64 
                        && (x != i || y != j) {
                            count[x as usize][y as usize] += 1;
                        }
                    }
                }
            }
        }
    }
    count
}

#[allow(unused)]
pub fn part1(inputs: &Vec<Vec<State>>) -> i64 {
    let mut layout = inputs.clone();
    loop {
        let mut revised = false;
        let count = count_adj(&layout);
        for i in 0..layout.len() {
            for j in 0..layout[0].len() {
                if layout[i][j] == State::Empty && count[i][j] == 0 {
                    layout[i][j] = State::Taken;
                    revised = true;
                }
                else if layout[i][j] == State::Taken && count[i][j] >= 4 {
                    layout[i][j] = State::Empty;
                    revised = true;
                }
            }
        }
        if !revised { break; }
    }
    layout.iter().map(|line| line.iter().filter(|s| **s==State::Taken).count() as i64).sum()
}

fn count_adj_2(seats: &Vec<Vec<State>>) -> Vec<Vec<i64>> {
    let mut count = vec![vec![0; seats[0].len()]; seats.len()];
    for i in 0..seats.len() as i64 {
        for j in 0..seats[i as usize].len() as i64 {
            if seats[i as usize][j as usize] == State::Taken {
                for x in -1..2 {
                    for y in -1..2 {
                        for z in 1..cmp::max(seats.len(), seats[0].len()) as i64 {
                            if i+x*z >= 0 
                            && j+y*z >= 0 
                            && i+x*z < seats.len() as i64 
                            && j+y*z < seats[i as usize].len() as i64 
                            && (x != 0 || y != 0) {
                                if seats[(i+x*z) as usize][(j+y*z) as usize] != State::NoSeat {
                                    count[(i+x*z) as usize][(j+y*z) as usize] += 1;
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    count
}


#[allow(unused)]
pub fn part2(inputs: &Vec<Vec<State>>) -> i64 {
    let mut layout = inputs.clone();
    loop {
        let mut revised = false;
        let count = count_adj_2(&layout);
        for i in 0..layout.len() {
            for j in 0..layout[0].len() {
                if layout[i][j] == State::Empty && count[i][j] == 0 {
                    layout[i][j] = State::Taken;
                    revised = true;
                }
                else if layout[i][j] == State::Taken && count[i][j] >= 5 {
                    layout[i][j] = State::Empty;
                    revised = true;
                }
            }
        }
        if !revised { break; }
    }
    layout.iter().map(|line| line.iter().filter(|s| **s==State::Taken).count() as i64).sum()
}

#[test]
fn test_count() {
    let inputs = String::from(
        "#.#L.L#.##\n\
         #LLL#LL.L#\n\
         L.#.L..#..\n\
         #L##.##.L#\n\
         #.#L.LL.LL\n\
         #.#L#L#.##\n\
         ..L.L.....\n\
         #L#L##L#L#\n\
         #.LLLLLL.L\n\
         #.#L#L#.##")
        .lines()
        .map(|line| line
            .chars()
            .map(|c| {match c { 'L' => State::Empty, '#' => State::Taken, _=> State::NoSeat}})
            .collect::<Vec<State>>()
        )
        .collect::<Vec<Vec<State>>>();
    //println!("{:?}", inputs);
    assert_eq!(State::Taken, inputs[0][0]);
    assert_eq!(State::Empty, inputs[3][1]);

    let count = count_adj(&inputs);
    //println!("{:?}", count);
    assert_eq!(1, count[0][0]);
    assert_eq!(3, count[9][1]);
}

#[test]
fn test_day11_part1() {
    let inputs = String::from("L.LL.LL.LL\n\
    LLLLLLL.LL\n\
    L.L.L..L..\n\
    LLLL.LL.LL\n\
    L.LL.LL.LL\n\
    L.LLLLL.LL\n\
    ..L.L.....\n\
    LLLLLLLLLL\n\
    L.LLLLLL.L\n\
    L.LLLLL.LL")
    .lines()
    .map(|line| line
        .chars()
        .map(|c| {println!("{}", c); match c { 'L' => State::Empty, '#' => State::Taken, _=> State::NoSeat}})
        .collect::<Vec<State>>()
    )
    .collect::<Vec<Vec<State>>>();
    assert_eq!(37, part1(&inputs));
}

#[test]
fn test_day11_part2() {
    let inputs = String::from("L.LL.LL.LL\n\
    LLLLLLL.LL\n\
    L.L.L..L..\n\
    LLLL.LL.LL\n\
    L.LL.LL.LL\n\
    L.LLLLL.LL\n\
    ..L.L.....\n\
    LLLLLLLLLL\n\
    L.LLLLLL.L\n\
    L.LLLLL.LL")
    .lines()
    .map(|line| line
        .chars()
        .map(|c| {println!("{}", c); match c { 'L' => State::Empty, '#' => State::Taken, _=> State::NoSeat}})
        .collect::<Vec<State>>()
    )
    .collect::<Vec<Vec<State>>>();
    assert_eq!(26, part2(&inputs));
}

#[test]
fn run_day11() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . . ");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 11 part 1: ");
    println!("{} - in {:?}", part1(&inputs), pt_start.elapsed().unwrap());
    print!("Day 11 part 2: ");
    let pt_start = SystemTime::now();
    println!("{} - in {:?}", part2(&inputs), pt_start.elapsed().unwrap());
    println!("Total duration: {:?}", start.elapsed().unwrap())
}