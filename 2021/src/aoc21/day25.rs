static DAY: i32 = 25;

pub fn input(test: i64) -> Vec<Vec<char>> {
    use std::io::{prelude::*, BufReader};
    let f = match test { 
        0 => crate::aoc::_test_input_file(DAY, 0),
        _ => crate::aoc::input_file(DAY) 
    };
    let f = BufReader::new(f.unwrap());
    f.lines().map(parse_line).collect::<_>()
}

fn parse_line(line: Result<String, std::io::Error>) -> Vec<char> {
    line.unwrap().trim().chars().collect::<_>()
}

pub fn part1(inputs: &Vec<Vec<char>>) -> usize {
    let mut map = inputs.clone();
    let (m, n) = (map.len(), map[0].len());
    for i in 0.. {
        let mut changed = false;
        let mut buf1 = vec![vec!['.';map[0].len()];map.len()];
        let mut buf2 = vec![vec!['.';map[0].len()];map.len()];
        for y in 0..m {
            for x in 0..n {
                match map[y][x] {
                    '>' => if map[y][(x+1)%n] == '.' {
                        buf1[y][(x+1)%n] = '>';
                        changed = true
                    } else {
                        buf1[y][x] = '>'
                    },
                    'v' => buf1[y][x] = 'v',
                    _ => continue
                }
            }
        }
        for y in 0..m {
            for x in 0..n {
                match buf1[y][x] {
                    'v' => if buf1[(y+1)%m][x] == '.' {
                        buf2[(y+1)%m][x] = 'v';
                        changed = true
                    } else {
                        buf2[y][x] = 'v'
                    },
                    '>' => buf2[y][x] = '>',
                    _ => continue
                }
            }
        }
        if !changed {
            return i + 1
        }
        map = buf2;
    }
    0
}

/* TESTS */

#[test]
fn test_day25_part1() {
    let inputs = input(0);
    assert_eq!(part1(&inputs), 58);
}

#[allow(unused)]
pub fn run() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . .");
    let inputs = input(-1);
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day {} part 1: ", DAY);
    println!("{}", part1(&inputs));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    println!("Total time: {:?}", start.elapsed().unwrap());
}
