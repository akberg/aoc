static DAY: usize = 05;

pub fn input() -> String {
    crate::aoc::input_raw(5)
}

pub fn part1(inputs: &str) -> String {
    let (init, instr) = inputs.split_once("\n\n").unwrap();

    let mut init = init.lines().collect::<Vec<_>>();
    // Remove ID row, and count no of needed stacks
    let mut stacks = vec![Vec::new(); init.remove(init.len()-1).split_ascii_whitespace().count()];
    init.iter().rev().for_each(|line| {
        line.chars().enumerate().filter(|(_,c)| c.is_alphabetic()).for_each(|(i, item)| {
            stacks[(i-1)/4].push(item);
        })
    });

    instr.lines().for_each(|line| {
        let (n, i0, i1) = sscanf::scanf!(line, "move {} from {} to {}",usize,usize,usize).unwrap();
        for _ in 0..n {
            let item = stacks[i0-1].pop().unwrap();
            stacks[i1-1].push(item);
        }
    });
    String::from(
        stacks.iter().map(|s| s.last().unwrap().to_string()).collect::<Vec<_>>().join("")
    )
}

pub fn part2(inputs: &str) -> String {
    let (init, instr) = inputs.split_once("\n\n").unwrap();

    let mut init = init.lines().collect::<Vec<_>>();
    // Remove ID row, and count no of needed stacks
    let mut stacks = vec![Vec::new(); init.remove(init.len()-1).split_ascii_whitespace().count()];
    init.iter().rev().for_each(|line| {
        line.chars().enumerate().filter(|(_,c)| c.is_alphabetic()).for_each(|(i, item)| {
            stacks[(i-1)/4].push(item);
        })
    });

    instr.lines().for_each(|line| {
        let (n, i0, i1) = sscanf::scanf!(line, "move {} from {} to {}",usize,usize,usize).unwrap();
        let items = (0..n).map(|_| stacks[i0-1].pop().unwrap()).collect::<Vec<_>>();
        items.iter().rev().for_each(|&item|{stacks[i1-1].push(item)});
    });
    String::from(
        stacks.iter().map(|s| s.last().unwrap().to_string()).collect::<Vec<_>>().join("")
    )
}

#[test]
fn test_day5_part1() {
    let inputs = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    assert_eq!(part1(&inputs), String::from("CMZ"));
}

#[test]
fn test_day5_part2() {
    let inputs = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    assert_eq!(part2(&inputs), String::from("MCD"));
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


