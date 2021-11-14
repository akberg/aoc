use std::str::FromStr;
use std::num::ParseIntError;
#[derive(Debug)]
pub enum Dir { N=270, E=0, S=90, W=180, F }
#[derive(Debug)]
pub enum Action {
    Move(Dir, i64), Rot(i64) // L is Rot(-phi)
}
impl FromStr for Action {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (d, i) = s.split_at(1);
        
        match d {
            "F" => Ok(Self::Move(Dir::F, i.parse::<i64>()?)),
            "N" => Ok(Self::Move(Dir::N, i.parse::<i64>()?)),
            "E" => Ok(Self::Move(Dir::E, i.parse::<i64>()?)),
            "S" => Ok(Self::Move(Dir::S, i.parse::<i64>()?)),
            "W" => Ok(Self::Move(Dir::W, i.parse::<i64>()?)),
            "L" => Ok(Self::Rot(360-i.parse::<i64>()?)),
            "R" => Ok(Self::Rot(i.parse::<i64>()?)),
            &_ => unreachable!()
        }
    }
}

#[allow(unused)]
pub fn input() -> Vec<Action> {
    let mut ret = crate::aoc::input_raw(20, 12)
        .lines()
        .map(|a| a.parse::<Action>().unwrap())
        .collect::<Vec<Action>>();
    ret
}

#[allow(unused)]
pub fn part1(inputs: &[Action]) -> i64 {
    let (n, e, _) = inputs
        .iter()
        .fold((0, 0, 0), |(n, e, d), a| {
            match a {
                Action::Rot(deg) => (n, e, (d + deg) % 360),
                Action::Move(dir, x) => match dir {
                    Dir::N => (n + x, e, d),
                    Dir::E => (n, e + x, d),
                    Dir::S => (n - x, e, d),
                    Dir::W => (n, e - x, d),
                    Dir::F => match d {
                        270 => (n + x, e, d),
                        0 => (n, e + x, d),
                        90 => (n - x, e, d),
                        180 => (n, e - x, d),
                        _ => unreachable!()
                    }
                }
            }}
    );
    n.abs()+e.abs()
}

#[allow(unused)]
pub fn part2(inputs: &[Action]) -> i64 {
    let (n, e, _, _) = inputs
        .iter()
        .fold((0, 0, 1, 10), |(n, e, wn, we), a| {
            match a {
                Action::Rot(deg) => match deg {
                    90 => (n, e, -we, wn),
                    180 => (n, e, -wn, -we),
                    270 => (n, e, we, -wn),
                    _ => unreachable!()
                },
                Action::Move(dir, x) => match dir {
                    Dir::N => (n, e, wn + x, we),
                    Dir::E => (n, e, wn, we + x),
                    Dir::S => (n, e, wn - x, we),
                    Dir::W => (n, e, wn, we - x),
                    Dir::F => (n + wn*x, e +we*x, wn, we)
                }
            }
        }
    );
    n.abs()+e.abs()
}


#[test]
fn test_day12_part1() {
    let inputs = String::from("F10\n\
    N3\n\
    F7\n\
    R90\n\
    F11")
    .lines()
    .map(|a| a.parse::<Action>().unwrap())
    .collect::<Vec<Action>>();
    assert_eq!(25, part1(&inputs));
}

#[test]
fn test_day12_part2() {
    let inputs = String::from("F10\n\
    N3\n\
    F7\n\
    R90\n\
    F11")
    .lines()
    .map(|a| a.parse::<Action>().unwrap())
    .collect::<Vec<Action>>();
    assert_eq!(286, part2(&inputs));
}

#[test]
fn run_day12() {
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