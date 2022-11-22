static DAY: i32 = 24;

pub fn input(test: i64) -> Vec<Instr> {
    use std::io::{prelude::*, BufReader};
    let f = match test { 
        0 => crate::aoc::_test_input_file(DAY, 0),
        1 => crate::aoc::_test_input_file(DAY, 1),
        _ => crate::aoc::input_file(DAY) 
    };
    let f = BufReader::new(f.unwrap());
    f.lines().map(parse_line).collect::<_>()
}

fn parse_line(line: Result<String, std::io::Error>) -> Instr {
    use Instr::*;
    let mut line = line.unwrap();
    let mut line = line.split_ascii_whitespace();
    match line.next().unwrap() {
        "inp" => Inp(line.next().unwrap().parse::<Op>().unwrap()),
        "add" => Add(line.next().unwrap().parse::<Op>().unwrap(), line.next().unwrap().parse::<Op>().unwrap()),
        "mul" => Mul(line.next().unwrap().parse::<Op>().unwrap(), line.next().unwrap().parse::<Op>().unwrap()),
        "div" => Div(line.next().unwrap().parse::<Op>().unwrap(), line.next().unwrap().parse::<Op>().unwrap()),
        "mod" => Mod(line.next().unwrap().parse::<Op>().unwrap(), line.next().unwrap().parse::<Op>().unwrap()),
        "eql" => Eql(line.next().unwrap().parse::<Op>().unwrap(), line.next().unwrap().parse::<Op>().unwrap()),
        &_ => unreachable!(),
    }
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Op { X, Y, Z, W, Im(i64) }
impl std::str::FromStr for Op {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Op::X),
            "y" => Ok(Op::Y),
            "z" => Ok(Op::Z),
            "w" => Ok(Op::W),
            s => Ok(Op::Im(s.parse::<i64>().unwrap())),
        }
    }
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Instr { Inp(Op), Add(Op, Op), Mul(Op, Op), Div(Op, Op), Mod(Op, Op), Eql(Op, Op) }
#[derive(Default, Debug)]
struct Alu {
    x_reg: i64,
    y_reg: i64,
    z_reg: i64,
    w_reg: i64,
    inp: Vec<i64>,
}

impl Alu {
    fn get(&self, op: Op) -> i64 {
        match op {
            Op::X => self.x_reg,
            Op::Y => self.y_reg,
            Op::Z => self.z_reg,
            Op::W => self.w_reg,
            Op::Im(x) => x,
        }
    }
    fn set(&mut self, op: Op, val: i64) {
        match op {
            Op::X => self.x_reg = val,
            Op::Y => self.y_reg = val,
            Op::Z => self.z_reg = val,
            Op::W => self.w_reg = val,
            Op::Im(_) => unreachable!(),
        }
    }
    pub fn execute(&mut self, instr: Instr) {
        // eprintln!("{:?}", self);
        match instr {
            Instr::Inp(reg) => {
                let x = self.inp.remove(0);
                self.set(reg, x);
            },
            Instr::Add(op1, op2) => {
                self.set(op1, self.get(op1) + self.get(op2));
            },
            Instr::Mul(op1, op2) => {
                self.set(op1, self.get(op1) * self.get(op2));
            },
            Instr::Div(op1, op2) => {
                self.set(op1, self.get(op1) / self.get(op2));
            },
            Instr::Mod(op1, op2) => {
                self.set(op1, self.get(op1) % self.get(op2));
            },
            Instr::Eql(op1, op2) => {
                self.set(op1, if self.get(op1) == self.get(op2) { 1 } else { 0 });
            },
        }
    }
    pub fn run(&mut self, prog: &Vec<Instr>) {
        prog.iter().for_each(|&instr| self.execute(instr));
    }
}

pub fn part1(inputs: &Vec<Instr>) -> u64 {
    let mut modno = vec![9, 9, 9, 9, 9, 9, 7, 2, 6, 5, 7, 9, 3, 9];
    loop {
        let mut alu = Alu { inp: modno.clone(), ..Default::default() };
        alu.run(inputs);
        if alu.z_reg == 0 {
            // Mod no accepted
            return modno.iter().enumerate().fold(0, |acc, (i, &n)| acc + n as u64 * u64::pow(10, 13 - i as u32))
        }
        for i in (0..modno.len()).rev() {
            modno[i] -= 1;
            if i == 6 {
                eprintln!("{:?}", modno);
            }
            if modno[i] == 0 {
                modno[i] = 9;
            } else {
                break;
            }
        }
    }
}


pub fn part2(inputs: &Vec<Instr>) -> u64 {
    0
}

/* TESTS */

#[test]
fn test_day24_part1() {
    let inputs = input(0);
    assert_eq!(part1(&inputs), 12521);
}

#[test]
fn test_day24_part2() {
    let inputs = input(2);
    assert_eq!(part2(&inputs), 2_758_514_936_282_235_u64);
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
    let pt_start = SystemTime::now();
    print!("Day {} part 2: ", DAY);
    println!("{}", part2(&inputs));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    println!("Total time: {:?}", start.elapsed().unwrap());
}
