/// Keywords: Compiler optimization
use enum_primitive_derive::Primitive;
// use std::ops::Rem;

/// Keywords: Instruction Set, 3-bit
// Derive Primitive for enum
use num_traits::FromPrimitive; //, ToPrimitive};

use super::YEAR;
static DAY: usize = 17;

// Op: 0 1 2 3 4 5 6 7
//     0 1 2 3 A B C -

#[derive(Debug, Primitive, Copy, Clone)]
enum Instr {
    /// reg_a = floor(reg_a / 2**combo_op)
    Adv = 0,
    /// reg_b = reg_b ^ liter_op
    Bxl = 1,
    /// reg_b = combo_op (mod 8)
    Bst = 2,
    /// if reg_a > 0: pc = liter_op
    Jnz = 3,
    /// reg_b = reg_b ^ reg_c
    Bxc = 4,
    /// print combo_op (mod 8)
    Out = 5,
    /// reg_b = floor(reg_b / 2**combo_op)
    Bdv = 6,
    /// reg_c = floor(reg_c / 2**combo_op)
    Cdv = 7,
}

#[derive(Debug, Clone)]
struct Computer {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    pc: usize,
    prog: Vec<u8>,
    pub output_buffer: Vec<u8>,
}
impl Computer {
    pub fn new(reg_a: usize, reg_b: usize, reg_c: usize, prog: Vec<u8>) -> Self {
        Self {
            reg_a,
            reg_b,
            reg_c,
            pc: 0,
            prog,
            output_buffer: Vec::new(),
        }
    }
    // fn flush(&mut self) {
    //     println!("{}", self.output_buffer);
    //     self.output_buffer = String::new();
    // }
    fn combo_op(&self, op: usize) -> usize {
        match op {
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => panic!("Reserved COMBO OP value 7"),
            _ => op as usize,
        }
    }
    fn step(&mut self) {
        if self.pc == self.prog.len() {
            println!("Halted");
            return;
        }
        let instr = Instr::from_u8(self.prog[self.pc]).unwrap();
        let oprnd = self.prog[self.pc + 1] as usize;

        println!("{:?} {}   PC={}   {:?}", instr, oprnd, self.pc, self);
        // Increase program counter (ignoring this operation if a jump executes)
        self.pc += 2;

        match instr {
            Instr::Adv => self.reg_a /= 2usize.pow(self.combo_op(oprnd) as u32),
            Instr::Bxl => self.reg_b ^= oprnd,
            Instr::Bst => self.reg_b = self.combo_op(oprnd) & 0x7,
            Instr::Jnz => {
                if self.reg_a != 0 {
                    self.pc = oprnd;
                }
            }
            Instr::Bxc => self.reg_b ^= self.reg_c,
            Instr::Out => self.output_buffer.push(self.combo_op(oprnd) as u8 & 0x7),
            Instr::Bdv => self.reg_b = self.reg_a / 2usize.pow(self.combo_op(oprnd) as u32),
            Instr::Cdv => self.reg_c = self.reg_a / 2usize.pow(self.combo_op(oprnd) as u32),
        }
    }

    pub fn run(&mut self) {
        // println!("Run Computer\n{:?}", self);
        while self.pc < self.prog.len() {
            self.step();
            // println!("{:?}", self);
        }
        // println!("Final output");
        // self.flush();
    }
}

fn input() -> Computer {
    let inputs = crate::aoc::input_raw(YEAR, DAY);
    let mut inputs = inputs.lines();
    let a = inputs
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .trim()
        .parse::<usize>()
        .unwrap();
    let b = inputs
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .trim()
        .parse::<usize>()
        .unwrap();
    let c = inputs
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .trim()
        .parse::<usize>()
        .unwrap();
    let prog = inputs
        .skip(1)
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .trim()
        .split(",")
        .map(|c| c.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    Computer::new(a, b, c, prog)
}

/// (Solved >2h) Run computer as specified, and print output as a comma-
/// separated list.
///
/// Reading specifications is difficult...
fn part1(inputs: &Computer) -> String {
    let mut comp = inputs.clone();
    comp.run();
    comp.output_buffer
        .into_iter()
        .map(|i| format!("{}", i))
        .collect::<Vec<_>>()
        .join(",")
}

fn dfs_prog(comp: Computer, depth: usize) -> Option<usize> {
    let mut base = 0;
    for i in 0..8 {
        let mut test = comp.clone();
        let a = comp.reg_a + (i << (3*(depth - 1)));
        println!("depth = {} - a += {} = {}", depth, i, a);
        test.reg_a = a;
        test.run();

        if test.output_buffer.len() >= depth && test.output_buffer[0..depth] == test.prog[0..depth]
        {
            base = a;
            if depth == test.prog.len() {
                return Some(test.reg_a);
            }
        }
    }
    for i in 0.. {
        let mut test = comp.clone();
        let a = comp.reg_a + ((base + i*8) << (3*depth - 1));
        test.reg_a = a;
        test.run();
        if test.output_buffer.len() >= depth && test.output_buffer[0..depth] == test.prog[0..depth]
        {
            let mut n = comp.clone();
            n.reg_a = a;
            if let Some(x) = dfs_prog(n, depth + 1) {
                return Some(x);
            }
        }
    }
    None
}

// XOR
// 101 ^ 000 = 101
//       001 = 100
//       010 = 111
//       011 = 110
//       100 = 001
//       101 = 000
//       110 = 011
//       111 = 010

/// Find the starting value of A whichc makes the program output a copy of
/// itself.
fn part2(inputs: &Computer) -> usize {
    // Loop:
    //
    // BST 4    B <- A              B=A[2:0]
    // BXL 5    B <- B ^ 0b101      B=(A[2:0]) ^ 0b101
    // CDV 5    C <- A >> B         C=A >> ((A[2:0]) ^ 0b101)
    // BXL 6    B <- B ^ 0b100      B=(A[2:0]) ^ 1
    // BXC 1    B <- B ^ C          B=((A[2:0]) ^ 1) ^ (A >> ((A[2:0]) ^ 0b101))
    // OUT 5    print B             p (A ^ 1) ^ (A >> (A ^ 0b101))
    // ADV 3    A <- A >> 3
    // JNZ 0
    //
    // The loop processes 3 bits of A at the time, so it should suffice to
    // compute 3-bitwise the correct value for A.
    //
    let mut comp = inputs.clone();
    comp.reg_a = 0;
    dfs_prog(comp, 1).unwrap()
}

#[test]
fn test_2024_day17_part1_0() {
    let input = Computer::new(729, 0, 0, vec![0, 1, 5, 4, 3, 0]);
    assert_eq!(part1(&input), String::from("4,6,3,5,6,3,5,2,1,0"));
}
#[test]
fn test_2024_day17_part1_1() {
    let mut input = Computer::new(0, 0, 9, vec![2, 6]);
    input.run();
    assert_eq!(input.reg_b, 1);

    let mut input = Computer::new(60589763, 5, 0, vec![2, 4]);
    input.run();
    assert_eq!(input.reg_b, 3);
}
#[test]
fn test_2024_day17_part1_2() {
    let mut input = Computer::new(10, 0, 0, vec![5, 0, 5, 1, 5, 4]);
    input.run();
    assert_eq!(
        input
            .output_buffer
            .into_iter()
            .map(|i| format!("{}", i))
            .collect::<Vec<_>>()
            .join(","),
        String::from("0,1,2")
    );
}
#[test]
fn test_2024_day17_part1_3() {
    let mut input = Computer::new(2024, 0, 0, vec![0, 1, 5, 4, 3, 0]);
    input.run();
    assert_eq!(input.reg_a, 0);
    assert_eq!(
        input
            .output_buffer
            .into_iter()
            .map(|i| format!("{}", i))
            .collect::<Vec<_>>()
            .join(","),
        String::from("4,2,5,6,7,7,7,7,3,1,0")
    );
}
#[test]
fn test_2024_day17_part1_4() {
    let mut input = Computer::new(0, 29, 0, vec![1, 7]);
    input.run();
    assert_eq!(input.reg_b, 26);
}
#[test]
fn test_2024_day17_part1_5() {
    let mut input = Computer::new(0, 2024, 43690, vec![4, 0]);
    input.run();
    assert_eq!(input.reg_b, 44354);
}

#[test]
fn test_2024_day17_part2() {
    let input = Computer::new(10, 0, 0, vec![0, 3, 5, 4, 3, 0]);
    assert_eq!(part2(&input), 117440);
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
    print!("{} Day {} part 1: ", YEAR, DAY);
    println!("{}", res);
    println!("Took {:?}", pt_start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    let res = part2(&inputs);
    print!("{} Day {} part 2: ", YEAR, DAY);
    println!("{}", res);
    println!("Took {:?}", pt_start.elapsed().unwrap());
    println!("Total time: {:?}", start.elapsed().unwrap());
}
