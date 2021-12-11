static DAY: i32 = 11;

use std::collections::VecDeque;

fn parse_line(line: &str) -> Vec<i32> {
    line.chars().map(|c| c as i32 - '0' as i32).collect::<_>()
}

pub fn input() -> Vec<Vec<i32>> {
    use std::io::{prelude::*, BufReader};
    let f = crate::aoc::input_file(DAY).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|line|parse_line(&line.unwrap())).collect::<_>()
}

/** */
pub fn part1(inputs: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;

    for i in 0..inputs.len() {
        for j in 0..inputs[0].len() {
            if [(0,-1),(0,1),(1,0),(-1,0)]
            .iter()
            .filter(|(di,dj)| (i>0 || *di>-1) && (j>0 || *dj>-1))
            .all(|(di,dj)|
                inputs[i][j] < *inputs.get((i as isize + di) as usize)
                    .unwrap_or(&vec![])
                    .get((j as isize+dj) as usize)
                    .unwrap_or(&9)
            ) {
                sum += inputs[i][j] + 1;
            }
        }
    }
    sum
}


/**Fill all basins and return the product of the 3 largest ones */
pub fn part2(inputs: &Vec<Vec<i32>>) -> u32 {
    0
    // #[allow(non_snake_case)]
    // let M = inputs.len();
    // #[allow(non_snake_case)]
    // let N = inputs[0].len();
    // // Track visited points
    // let mut visited = vec![vec![0; inputs[0].len()]; inputs.len()];
    // let low_points = local_min(inputs);
    // let mut queue = low_points.iter().map(|&p| VecDeque::from(vec![p])).collect::<Vec<_>>();
    // // Track size of each basin
    // let mut b_size = vec![0;low_points.len()];

    // // Floodfill
    // for basin in 0..queue.len() {
    //     eprintln!("\n== {} ==\n", basin);
    //     while let Some((i,j)) = queue[basin].pop_front() {
    //         visited[i][j] = basin+1;
    //         // Possible neightbours
    //         let nbs = [(0,-1),(0,1),(1,0),(-1,0)]
    //             .iter()
    //             // Edge filter
    //             .filter(|&(di,dj)| (i>0 || *di>-1) && (i<M-1 || *di<1) && (j>0 || *dj>-1) && (j<N-1 || *dj<1))
    //             .map(|(di,dj)| ((i as isize + di) as usize, (j as isize + dj) as usize))
    //             .filter(|&(ii,jj)| basin+1 != visited[ii][jj])
    //             .collect::<Vec<_>>();
    //         // Just a floodfill with 9's as delimiters...
    //         if inputs[i][j] < 9 {
    //             b_size[basin] += 1;
    //             for nb in nbs {
    //                 visited[nb.0][nb.1] = basin+1;
    //                 queue[basin].push_back(nb);
    //             }
    //         }
    //     }

    //     // Debug
    //     let colors = ["0;31m", "0;32m", "0;33m", "0;34m", "0;35m", "0;36m", "0;37m", "1;30m", "1;31m", "1;32m", "1;33m", "1;34m", "1;35m", "1;36m"];
    //     for (i, line) in (&visited).iter().enumerate() {
    //         for (j, p) in line.iter().enumerate() {
    //             if inputs[i][j] == 9 {
    //                 eprint!("\x1b[0;41m{}\x1b[0m", inputs[i][j]);
    //             } else {
    //                 eprint!("\x1b[{}{}\x1b[0m", if *p==0 { "0m" } else { colors[*p % 14] }, inputs[i][j]);
    //             }
    //         }
    //         println!("");
    //     }
    //     eprintln!("{}: {}", basin, b_size[basin]);
    // }

    // // Return product of three largest basins size
    // b_size.sort_by(|a,b| b.cmp(a));
    // b_size[0] * b_size[1] * b_size[2]
}

/* TESTS */
#[allow(unused)]
static TEST_INPUT: &'static [&str] = &[
    "5483143223",
    "2745854711",
    "5264556173",
    "6141336146",
    "6357385478",
    "4167524645",
    "2176841721",
    "6882881134",
    "4846848554",
    "5283751526",
];

#[test]
fn test_day11_part1() {
    let inputs = TEST_INPUT.iter().map(|line|parse_line(*line)).collect::<Vec<_>>();
    assert_eq!(part1(&inputs), 15);
}

#[test]
fn test_day11_part2() {
    let inputs = TEST_INPUT.iter().map(|line|parse_line(*line)).collect::<Vec<_>>();
    assert_eq!(part2(&inputs), 1134);
}

#[test]
fn test_day11_parse_line() {
    assert_eq!(parse_line(TEST_INPUT[0]), vec![5,4,8,3,1,4,3,2,2,3]);
}

#[allow(unused)]
pub fn run() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . .");
    let inputs = input();
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
