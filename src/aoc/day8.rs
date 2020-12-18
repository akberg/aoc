
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum OpCode {NOP, ACC, JMP}

impl FromStr for OpCode {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nop" => Ok(Self::NOP),
            "acc" => Ok(Self::ACC),
            "jmp" => Ok(Self::JMP),
            _e => Err("Unknown opcode")
        }
    }
}

#[allow(unused)]
fn input() -> Vec<(OpCode, i32)> {
    crate::aoc::input_raw(8)
        .lines()
        .map(|line| parse_line(line))
        .collect()
}

fn parse_line(line: &str) -> (OpCode, i32) {
    let mut s = line.split_ascii_whitespace();
    (s.next().unwrap().parse::<OpCode>().unwrap(), s.next().unwrap().parse::<i32>().unwrap())
}

fn machine(prgm: &[(OpCode, i32)], patch: Option<i32>) -> Result<i32, i32> {
    let mut log = HashSet::new();
    let mut pc: i32 = 0;
    let mut glob = 0;
    let p = match patch { Some(_) => true, None => false };
    loop {
        if pc >= prgm.len() as i32 { break; }
        if !log.insert(pc) { break; }
        let (op, arg) = &prgm[pc as usize];
        match op {
            OpCode::NOP => {
                if p { 
                    if patch.unwrap()==pc { pc += arg; } else { pc += 1; }
                } else { pc += 1; }
            },
            OpCode::ACC => {
                glob += arg;
                pc += 1;
            },
            OpCode::JMP => {
                if p {if patch.unwrap_or(pc)!=pc { pc += arg; } else { pc += 1; }}
                else {pc += arg}
            },
        }
    }
    if pc == prgm.len() as i32 {
        Ok(glob)
    } else {
        Err(glob)
    }
}

#[allow(unused)]
fn part1(inputs: &[(OpCode, i32)]) -> i32 {
    match machine(inputs, None) {
        Ok(val) => val,
        Err(val) => val,
    }
}

#[allow(unused)]
fn part2(inputs: &[(OpCode, i32)]) -> i32 {
    for i in (0..inputs.len()-1) {
        let (op, x) = &inputs[i];
        match op {
            &OpCode::NOP|&OpCode::JMP => {
                let patch = Some(i as i32);
                match machine(inputs, patch) {
                    Ok(val) => {return val},
                    Err(_) => continue,
                }
            },
            &OpCode::ACC => continue,
        }
    }
    0
}

#[test]
fn test_day8_parse_line() {
    let inputs = String::from("nop +0\n\
        acc +1\n\
        jmp +4\n\
        acc +3\n\
        jmp -3\n\
        acc -99\n\
        acc +1\n\
        jmp -4\n\
        acc +6")
        .lines()
        .map(|line| parse_line(line))
        .collect::<Vec<(OpCode, i32)>>();
    assert_eq!((OpCode::NOP, 0), inputs[0]);
    assert_eq!((OpCode::JMP, -3), inputs[4]);
}


#[test]
fn test_day8_part1() {
    let inputs = String::from("nop +0\n\
        acc +1\n\
        jmp +4\n\
        acc +3\n\
        jmp -3\n\
        acc -99\n\
        acc +1\n\
        jmp -4\n\
        acc +6")
        .lines()
        .map(|line| parse_line(line))
        .collect::<Vec<_>>();

    assert_eq!(5, part1(&inputs));
}

#[test]
fn test_day8_part2() {
    let inputs = String::from("nop +0\n\
        acc +1\n\
        jmp +4\n\
        acc +3\n\
        jmp -3\n\
        acc -99\n\
        acc +1\n\
        jmp -4\n\
        acc +6")
        .lines()
        .map(|line| parse_line(line))
        .collect::<Vec<_>>();
    
    assert_eq!(8, part2(&inputs));
}

#[test]
fn run_day8() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . . ");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 8 part 1: ");
    println!("{} - in {:?}", part1(&inputs), pt_start.elapsed().unwrap());
    print!("Day 8 part 2: ");
    let pt_start = SystemTime::now();
    println!("{} - in {:?}", part2(&inputs), pt_start.elapsed().unwrap());
    println!("Total duration: {:?}", start.elapsed().unwrap())
}
