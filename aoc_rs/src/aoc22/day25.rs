use std::collections::VecDeque;
use itertools::Itertools;

static DAY: usize = 25;

pub fn input() -> String {
    crate::aoc::input_raw(super::YEAR, 25)
}

fn from_snafu(num: &str) -> i64 {
    let len = num.trim().len() as u32;
    num.trim().char_indices()
    .fold(0, |acc, (i,c)| acc + 5_i64.pow(len-i as u32-1) * match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => unreachable!()
    })
}

fn to_snafu(num: i64) -> String {
    let mut src = num;
    if src == 0 {
        return String::from("0");
    }
    let mut out = VecDeque::new();
    while src > 0 {
        match src % 5 {
            0 => {
                out.push_front('0');
            },
            1 => {
                out.push_front('1');
                src -= 1;
            },
            2 => {
                out.push_front('2');
                src -= 2;
            },
            3 => {
                out.push_front('=');
                src += 2;
            },
            4 => {
                out.push_front('-');
                src += 1;
            },
            _ => unreachable!()
        }
        src /= 5;
    }
    out.iter().join("")
}

pub fn part1(inputs: &str) -> String {
    to_snafu(inputs.lines().map(from_snafu).sum())
}

pub fn part2(_inputs: &str) -> String {
    String::from("Merry Christmas 🎉")
}

#[test]
fn test_day25_part1() {
    let to_snafu_ex = [
        (        1,              "1"),
        (        2,              "2"),
        (        3,             "1="),
        (        4,             "1-"),
        (        5,             "10"),
        (        6,             "11"),
        (        7,             "12"),
        (        8,             "2="),
        (        9,             "2-"),
        (       10,             "20"),
        (       15,            "1=0"),
        (       20,            "1-0"),
        (     2022,         "1=11-2"),
        (    12345,        "1-0---0"),
        (314159265,  "1121-1110-1=0"),
        (  1747,   "1=-0-2"   ),
        (  906,   "12111"    ),
        (  198,    "2=0="    ),
        (   11,      "21"    ),
        (  201,    "2=01"    ),
        (   31,     "111"    ),
        ( 1257,   "20012"    ),
        (   32,     "112"    ),
        (  353,   "1=-1="    ),
        (  107,    "1-12"    ),
        (    7,      "12"    ),
        (    3,      "1="    ),
        (   37,     "122"    ),
    ];

    let inputs = "1=-0-2
    12111
    2=0=
    21
    2=01
    111
    20012
    112
    1=-1=
    1-12
    12
    1=
    122";
    for pair in to_snafu_ex {
        assert_eq!(from_snafu(pair.1), pair.0);
        assert_eq!(to_snafu(pair.0), pair.1);

    }
    assert_eq!(part1(inputs), String::from("2=-1=0"))
}

#[test]
fn test_day25_part2() {
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


