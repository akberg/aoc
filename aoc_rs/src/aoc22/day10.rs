static DAY: usize = 10;

pub fn input() -> String {
    crate::aoc::input_raw(super::YEAR, DAY)
}

pub fn part1(inputs: &str) -> i64 {
    let mut cycle = 1;
    let mut regx = 1;
    let mut res = 0;
    inputs.lines().for_each(|line| {
        match line {
            "noop" => {
                if (cycle-20)%40 == 0 {
                    // Sum "signal strength"
                    res += regx * cycle;
                }
                cycle += 1;
            },
            _ => {
                // addx takes 2 cycles
                let x = line.split_at(4).1.trim().parse::<i64>().unwrap();
                if (cycle-20)%40 == 0 {
                    // Sum "signal strength"
                    res += regx * cycle;
                }
                if (cycle-19)%40 == 0 {
                    // Sum "signal strength"
                    res += regx * (cycle+1);
                }
                cycle+=2;
                regx+=x;
            }
        };
    });
    res
}

pub fn part2(inputs: &str) -> Vec<Vec<char>> {
    let mut crt = vec![vec!['.'; 40]; 6];
    fn crtprint(cycle: i64, regx: i64, crt: &mut Vec<Vec<char>>) {
        let posx = (cycle-1) as usize % 40;
        let posy = (cycle-1) as usize / 40;
        let sprite = (regx-1).max(0) as usize ..= (regx + 1) as usize;

        if sprite.contains(&posx) {
            crt[posy][posx] = '#';
        }
    }
    let mut cycle = 1i64;
    let mut regx = 1;
    inputs.lines().for_each(|line| {
        match line {
            "noop" => {
                crtprint(cycle, regx, &mut crt);
                cycle += 1;
            },
            _ => {
                let x = line.split_at(4).1.trim().parse::<i64>().unwrap();
                crtprint(cycle, regx, &mut crt);
                cycle += 1;
                crtprint(cycle, regx, &mut crt);
                cycle+=1;
                regx += x;
            }
        };
    });
    crt
}


#[test]
fn test_day10_part1() {
    let inputs = crate::aoc::_test_input_raw(10, 0);
    assert_eq!(part1(&inputs), 13140);
}

#[test]
fn test_day10_part2() {
    let inputs = crate::aoc::_test_input_raw(10, 0);
    let res = part2(&inputs);
    for y in 0..6 {
        for x in 0..40 {
            print!("{}", res[y][x]);
        }
        println!("");
    }
    // assert!(false);
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
    println!("");
    for y in 0..6 {
        for x in 0..40 {
            print!("{}", res[y][x]);
        }
        println!("");
    }
    println!("Took {:?}", pt_start.elapsed().unwrap());
    println!("Total time: {:?}", start.elapsed().unwrap());
}


