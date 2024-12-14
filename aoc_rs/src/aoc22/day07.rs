static DAY: usize = 07;

use std::collections::HashMap;

pub fn input() -> String {
    crate::aoc::input_raw(super::YEAR, DAY)
}

fn dir_sizes(inputs: &str, fs: &mut HashMap<String, usize>) {
    fs.insert(String::from(""), 0);
    let mut pwd = vec![String::from("")];
    for line in inputs.lines() {
        let mut line = line.split_ascii_whitespace();
        match line.next().unwrap() {
            "$" => {
                // Command
                match line.next().unwrap() {
                    "ls" => (),
                    "cd" => {
                        match line.next().unwrap() {
                            "/" => {
                                // To root
                                pwd.clear();
                            },
                            ".." => {
                                // Up one level
                                let _ = pwd.pop();
                            },
                            d => {
                                // In one level
                                pwd.push(String::from(d));
                                fs.insert(pwd.join("/"), 0);
                            }
                        }
                    },
                    _ => unreachable!()
                }

            },
            "dir" => {},
            s => {
                // File size (don't care about files)
                let _fname = line.next().unwrap();
                let s = s.parse::<usize>().unwrap();
                let mut path = String::from("");
                let f = fs.get_mut(&path).unwrap();
                *f += s;
                for i in 0..pwd.len() {
                    path = pwd[0..i+1].join("/");
                    let f = fs.get_mut(&path).unwrap();
                    *f += s;
                }
            }
        }
    }
}

pub fn part1(inputs: &str) -> u32 {
    let mut fs = HashMap::new();
    dir_sizes(inputs, &mut fs);
    let mut fs = fs.values().filter(|&&v| v < 100000).collect::<Vec<_>>();
    fs.sort();
    fs.iter().map(|&&v|v).sum::<usize>() as u32
}

pub fn part2(inputs: &str) -> usize {
    let mut fs = HashMap::new();
    dir_sizes(inputs, &mut fs);
    let mut fs = fs.values().collect::<Vec<_>>();
    fs.sort();
    let goal = 30000000 - (70000000 - fs[fs.len()-1]);
    **fs.iter().find(|&&&e|e>=goal).unwrap()
}

#[test]
fn test_day7_part1() {
    let inputs = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
assert_eq!(part1(inputs), 95437);
}

#[test]
fn test_day7_part2() {
    let inputs = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
assert_eq!(part2(inputs), 24933642);
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


