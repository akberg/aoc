use colored::{Colorize, Color};

static DAY: usize = 22;

pub fn input() -> String {
    crate::aoc::input_raw(super::YEAR, DAY)
        //.lines()
        //.map(|ls| ls.parse::<_>().unwrap())
        //.collect()
}

fn print_map(pmap: &Vec<Vec<char>>) {
    std::thread::sleep_ms(250);
    (0..10).for_each(|_|println!(""));
    for y in 0..pmap.len() {
        for x in 0..pmap[0].len() {
            print!("{}", pmap[y][x]);
        }
        println!("");
    }
}
fn print_map_slice(pmap: &Vec<Vec<char>>, posx: usize, posy: usize) {
    std::thread::sleep_ms(100);
    (0..10).for_each(|_|println!(""));
    for y in posy.checked_sub(20).unwrap_or(0)..pmap.len().min(posy+20) {
        for x in 0..pmap[0].len() {
            if x==posx && y==posy {
                print!("{}", format!("{}", pmap[y][x]).bold().red());
            }
            else if '#'==pmap[y][x] {
                print!("{}", format!("{}", pmap[y][x]).bold());
            }
            else if '.'==pmap[y][x] {
                print!("{}", format!("{}", pmap[y][x]).color(Color::TrueColor { r: 140, g: 140, b: 140 }));
            }
            else {
                print!("{}", format!("{}", pmap[y][x]).color(Color::TrueColor { r: 210, g: 170, b: 170 }));
            }
        }
        println!("");
    }
}

// TODO: wrapping around might not mean finding the first cell from 0, so search backwards from current position
pub fn part1(inputs: &str) -> usize {
    let (inputs, mut instructions) = inputs.split_once("\n\n").unwrap();
    let height = inputs.lines().count();
    let width = inputs.lines().fold(0, |acc, l| acc.max(l.len()));
    let mut map = vec![vec![' ';width];height];
    inputs.lines()
    .enumerate()
    .for_each(|(y, line)| {
        line.char_indices()
            .for_each(|(x, c)| {
                match c {
                    '.' => map[y][x] = '.',
                    '#' => map[y][x] = '#',
                    _ => (),
                };
            })
    });
    let mut pmap = map.clone();
    let mut posy = 0;
    let mut posx = map[0].iter().enumerate().find(|(_,c)| **c=='.').unwrap().0;
    let mut dir = 0;
    print_map_slice(&pmap, posx, posy);
    println!("dims {}x{}", width, height);

    while !instructions.trim().is_empty() {

        // println!("{}", instructions);
        // println!("dir: {}", dir);
        let (steps, instr) = sscanf::scanf!(instructions, "{}{}", usize, str)
            .unwrap_or_else(|_|(instructions.parse().unwrap(), ""));
        let directions = ['>', 'v', '<', '^'];
        print_map_slice(&pmap, posx, posy);
        println!("{} {}", steps, directions[dir]);

        match dir {
            0 => { // Right
                for _ in 0..steps {
                    if map[posy][posx] != '.' { panic!(); }
                    pmap[posy][posx] = '>';
                    // print_map_slice(&pmap, posx, posy);
                    // println!("x: {}, y: {} >", posx, posy);
                    if posx+1 >= map[posy].len() || map[posy][posx+1] == ' ' {
                        // Wrap around to the right, but stop if wall is hit
                        let wall = map[posy].iter()
                            .enumerate()
                            .find(|(_i,c)| **c=='#')
                            .unwrap_or((usize::MAX,&'#')).0;
                        let x = map[posy].iter()
                            .enumerate()
                            .find(|(_i,c)| **c=='.')
                            .unwrap().0;
                        // println!("wrap x right, open cell at {}", x);
                        if wall < x {
                            // println!("but wall in the way");
                            break;
                        } else {
                            posx = x;
                        }
                    } else if map[posy][posx+1] != '#' {
                        // Continue to the right if no obstacles
                        posx += 1;
                    } else {
                        break;
                    }
                }
            },
            2 => {
                for _ in 0..steps {
                    if map[posy][posx] != '.' { panic!(); }
                    pmap[posy][posx] = '<';
                    // print_map_slice(&pmap, posx, posy);
                    // println!("x: {}, y: {} <", posx, posy);
                    if posx == 0 || map[posy][posx-1] == ' ' {
                        // Wrap around to the left, but stop if wall is hit
                        let wall = map[posy].iter()
                            .enumerate()
                            .rev()
                            .find(|(_i,c)| **c=='#')
                            .unwrap_or((usize::MAX,&'#')).0;
                        let x = map[posy].iter()
                            .enumerate()
                            .rev()
                            .find(|(_i,c)| **c=='.')
                            .unwrap().0;
                            // println!("wrap x left, open cell at {}", x);
                        if wall > x {
                            // println!("but wall in the way");
                            break;
                        } else {
                            posx = x;
                        }
                    } else if map[posy][posx-1] != '#' {
                        // Continue to the right if no obstacles
                        posx -= 1;
                    } else {
                        break;
                    }
                }
            },
            1 => {
                for _ in 0..steps {
                    if map[posy][posx] != '.' { panic!(); }
                    pmap[posy][posx] = 'v';
                    // print_map_slice(&pmap, posx, posy);
                    // println!("x: {}, y: {} v", posx, posy);
                    if posy+1 >= map.len() || map[posy+1][posx] == ' ' {
                        // Wrap around to the right, but stop if wall is hit
                        let wall = map.iter()
                            .enumerate()
                            .map(|(i,v)| (i, v[posx]))
                            .find(|(_i,c)| *c=='#')
                            .unwrap_or((usize::MAX,'#')).0;
                        let y = map.iter()
                            .enumerate()
                            .map(|(i,v)| (i, v[posx]))
                            .find(|(_i,c)| *c=='.')
                            .unwrap().0;
                        // println!("wrap y down, open cell at {}", y);
                        if wall < y {
                            // println!("but wall in the way");
                            break;
                        } else {
                            posy = y;
                        }
                    } else if map[posy+1][posx] != '#' {
                        // Continue to the right if no obstacles
                        posy += 1;
                    } else {
                        break;
                    }
                }
            },
            3 => {
                for _ in 0..steps {
                    if map[posy][posx] != '.' { println!("x: {}, y: {} ^", posx, posy); panic!(); }
                    pmap[posy][posx] = '^';
                    // print_map_slice(&pmap, posx, posy);
                    // println!("x: {}, y: {} ^", posx, posy);
                    if posy == 0 || map[posy-1][posx] == ' ' {
                        // Wrap around to the left, but stop if wall is hit
                        let wall = map.iter()
                            .enumerate()
                            .rev()
                            .map(|(i,v)| (i, v[posx]))
                            .find(|(_i,c)| *c=='#')
                            .unwrap_or((usize::MAX,'#')).0;
                        let y = map.iter()
                            .enumerate()
                            .rev()
                            .map(|(i,v)| (i, v[posx]))
                            .find(|(_i,c)| *c=='.')
                            .unwrap().0;
                        // println!("wrap y up, open cell at {}", y);
                        if wall > y {
                            // println!("but wall in the way");
                            break;
                        } else {
                            posy = y;
                        }
                    } else if map[posy-1][posx] != '#' {
                        // Continue to the right if no obstacles
                        posy -= 1;
                    } else {
                        break;
                    }
                }
            },
            _ => break,
        }

        let (rot, instr) = sscanf::scanf!(instr, "{}{}", char, str).unwrap_or(('F', ""));
        if rot == 'F' {
            break;
        }
        instructions = instr;
        if rot == 'R' {
            dir = (dir+1) % 4;
        } else {
            dir = (dir+4-1) % 4;
        }
    }
    print_map(&pmap);
    println!("Ended in col {}, row {}, facing {}", posx+1, posy+1, dir);
    1000 * (posy+1) + 4 * (posx+1) + dir
}
// 5442 too low

pub fn part2(inputs: &str) -> u32 {
    0
}

#[allow(unused, non_upper_case_globals)]
static test_inputs: &'static str =
"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

#[test]
fn test_day22_part1() {
    let inputs = ".....#....\n\n4L0L4";
    assert_eq!(part1(inputs), 1006);
    let inputs = ".....#....\n\n4L0L5";
    assert_eq!(part1(inputs), 1042);
    assert_eq!(part1(test_inputs), 6032);
}

#[test]
fn test_day22_part2() {
    // TODO
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


