use colored::{Colorize, Color};

static DAY: usize = 17;

pub fn input() -> String {
    crate::aoc::input_raw(17)
        //.lines()
        //.map(|ls| ls.parse::<_>().unwrap())
        //.collect()
}

const N_SHAPES: usize = 5;
static SHAPES_H: [usize; N_SHAPES] = [1,3,3,4,2];
static SHAPES_W: [usize; N_SHAPES] = [4,3,3,1,2];

fn get_shape(current: usize, current_coord: (usize, usize)) -> Vec<(usize, usize)> {
    let x = current_coord.0;
    let y = current_coord.1;
    match current {
        0 => vec![(x+0,y+0), (x+1,y+0), (x+2,y+0), (x+3,y+0)],
        1 => vec![(x+1,y+2), (x+0,y+1), (x+1,y+1), (x+2,y+1), (x+1,y+0)],
        2 => vec![(x+2,y+2), (x+2,y+1), (x+0,y+0), (x+1,y+0), (x+2,y+0)],
        3 => vec![(x+0,y+3), (x+0,y+2), (x+0,y+1), (x+0,y+0)],
        4 => vec![(x+0,y+1), (x+1,y+1), (x+0,y+0), (x+1,y+0)],
        _ => unreachable!()
    }
}

fn print_tetris(shaft: &Vec<u8>, current: usize, current_coord: (usize, usize)) {

    let cur = if current < N_SHAPES { get_shape(current, current_coord) } else {vec![]};
    for (y, line) in shaft.into_iter().enumerate().rev().take(30) {
        print!("\t{}", format!("|").bold());
        for x in 0..7 {
            if let Some((_,_)) = cur.iter().find(|(xx,yy)| *xx==x as usize && *yy==y as usize) {
                print!("{}", format!("o").bold().red());
            }
            else if (line >> x) & 0x1 == 1 {
                print!("{}", format!("#").bold().color(Color::TrueColor { r: 182, g: 182, b: 182 }));
            }
            else {
                print!("{}", format!(".").color(Color::TrueColor { r: 128, g: 128, b: 128 }));
            }
        }
        println!("|");
    }
    println!("\t{}", format!("+-------+").bold());
}
fn insert_shape(shaft: &mut Vec<u8>, current: usize, current_coord: (usize, usize)) {
    let cur = get_shape(current, current_coord);
    // println!("{:?}", cur);
    for (x,y) in cur {
        shaft[y] |= 1 << x;
    }
}
/// return true if there is a hit
fn test_hit(shaft: &Vec<u8>, current: usize, current_coord: (usize, usize)) -> bool {
    let cur = get_shape(current, current_coord);
    // println!("{:?}", cur);
    // if current_coord.1 > 0 { println!("{:08b}", shaft[current_coord.1-1]); } else { println!("--------");}
    cur.iter().any(|(xx,yy)| yy==&0 || (shaft[yy-1] >> xx) & 0x1 == 1)
}
const DEBUG: bool = false;
const SLEEP: std::time::Duration = std::time::Duration::from_millis(10);

/// Tetris
/// - Each row one byte (7 cells)
/// - y=0 is bottom of shaft (print vector backwards)
fn tetris(inputs: &str, n_rounds: usize) -> usize {
    let inputs = inputs.trim().chars().collect::<Vec<_>>();

    let mut height = 0;
    let mut landed = vec![0u8;3+2];

    let mut j = 0;
    for i in 0..n_rounds {
        if i % 1000000 == 0 {
            println!("i={}", i);
        }
        let current = i % N_SHAPES;
        let mut x = 2;
        let mut y = height + 3;
        while landed.len() < height+3+SHAPES_H[current] { landed.push(0); }
        if i>0 && i % N_SHAPES == 0 && j % inputs.len() == 0 {
            print_tetris(&landed, current, (x, y));
            println!("^ 0/0");
            break;
        }

        // Appearing rock
        if DEBUG {
            (0..60).for_each(|_|{println!("");});
            print_tetris(&landed, current, (x, y));
            println!("\theight={}", height);
            println!("\ti     ={}", i);
            std::thread::sleep(SLEEP);
            (0..60).for_each(|_|{println!("");});
            println!("\t{}", inputs[j%inputs.len()]);
        }
        match inputs[j%inputs.len()] {
            '<' => if x > 0 && get_shape(current, (x,y)).iter().all(|(xx,yy)| (landed[*yy] >> (xx-1))&0x1==0) { x -= 1 },
            '>' => if x+SHAPES_W[current] < 7 && get_shape(current, (x,y)).iter().all(|(xx,yy)| (landed[*yy] >> (xx+1))&0x1==0) { x += 1 },
            _ => unreachable!()
        }
        // if i==1945 { println!("{:08b}", landed[y]);}
        if DEBUG {
            print_tetris(&landed, current, (x, y));
            println!("\theight={}", height);
            println!("\ti     ={}", i);
            std::thread::sleep(SLEEP);
        }
        j += 1;
        while y > 0 && !test_hit(&landed, current, (x, y)) {
            y -= 1;
            // if i==1945 { println!("{:08b}", landed[y]);}
            if DEBUG {
                (0..60).for_each(|_|{println!("");});
                print_tetris(&landed, current, (x, y));
                println!("\theight={}", height);
                println!("\ti     ={}", i);
                std::thread::sleep(SLEEP);
                (0..60).for_each(|_|{println!("");});
                println!("\t{}", inputs[j%inputs.len()]);
            }
            match inputs[j%inputs.len()] {
                '<' => if x > 0 && get_shape(current, (x,y)).iter().all(|(xx,yy)| (landed[*yy] >> (xx-1))&0x1==0) { x -= 1 },
                '>' => if x+SHAPES_W[current] < 7 && get_shape(current, (x,y)).iter().all(|(xx,yy)| (landed[*yy] >> (xx+1))&0x1==0) { x += 1 },
                _ => unreachable!()
            }
            // if i==1945 { println!("{:08b}", landed[y]);}
            if DEBUG {
                print_tetris(&landed, current, (x, y));
                println!("\theight={}", height);
                println!("\ti     ={}", i);
                std::thread::sleep(SLEEP);
            }
            j += 1;
        }
        if DEBUG {
            (0..60).for_each(|_|{println!("");});
            print_tetris(&landed, current, (x, y));
            println!("\theight={}", height);
            println!("\ti     ={}", i);
            std::thread::sleep(SLEEP);
        }
        insert_shape(&mut landed, current, (x, y));
        // Update height
        height = landed.iter().enumerate().rev().find(|(_,line)| line!=&&0).unwrap().0 + 1;
        // if i==1945 { println!("height={}", height); }
    }
    print_tetris(&landed, N_SHAPES, (0,0));

    height
}

pub fn part1(inputs: &str) -> usize {
    tetris(inputs, 100000)
}
// 3143 too low
// example: 3068 reached at 1915 rounds

pub fn part2(inputs: &str) -> usize {
    // TODO: Naively simulating tetris is not efficient enough (would require 6 000 000 sec)
    // TODO: search for a repeating pattern
    tetris(inputs, 1_000_000_000_000)
}

#[test]
fn test_day17_part1() {
    let inputs = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    assert_eq!(part1(inputs), 3068);
}

#[test]
fn test_day17_part2() {
    let inputs = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    assert_eq!(part2(inputs), 1514285714288);
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


