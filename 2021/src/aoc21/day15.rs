static DAY: i32 = 15;

use priority_queue::DoublePriorityQueue;

fn parse_line(line: &str) -> Vec<i32> {
    line.chars().map(|c| c as i32 - '0' as i32).collect::<_>()
}

pub fn input() -> Vec<Vec<i32>> {
    use std::io::{prelude::*, BufReader};
    let f = crate::aoc::input_file(DAY).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|line|parse_line(&line.unwrap())).collect::<_>()
}

pub fn dijkstra_shortest_path(weights: &Vec<Vec<i32>>) -> (i32, Vec<Vec<Option<(usize, usize)>>>) {
    let height = weights.len();
    let width  = weights[0].len();
    
    let mut dist = vec![vec![std::i32::MAX;width];height];
    let mut prev = vec![vec![None;width];height];

    dist[0][0] = 0; //weights[0][0];

    let mut queue = DoublePriorityQueue::new();

    (0..height).for_each(|i| (0..width).for_each(|j| { queue.push((j, i), dist[i][j]); }));

    let height = height;
    let width = width;

    let mut count = 0;
    while let Some(((y, x), _w)) = queue.pop_min() {
        count += 1;
        /* Break when goal is reached */
        if (y, x) == (height-1, width-1) { break }
        for (dy, dx) in [(-1,0),(0,-1),(1,0),(0,1)] {
            let nb = (y as i32 + dy,x as i32 + dx);
            /* Check OoB */
            if nb.0 >= 0 && nb.1 >= 0 && height as i32 > nb.0 && width as i32 > nb.1 {
                let nb = (nb.0 as usize, nb.1 as usize);
                let alt = dist[y][x] + weights[nb.0][nb.1];
                
                if alt < dist[nb.0][nb.1] {
                    dist[nb.0][nb.1] = alt;
                    prev[nb.0][nb.1] = Some((y,x));
                    queue.push(nb, alt);
                }
            }
        }
    }
    (dist[height-1][width-1], prev)
}

fn inflate(mat: &Vec<Vec<i32>>, factor: usize) -> Vec<Vec<i32>> {
    let mut weights = vec![vec![0;mat[0].len()*factor];mat.len()*factor];

    for i in 0..mat.len() {
        for j in 0..mat[0].len() {
            for k in 0..5 {
                weights[i][j + mat[0].len()*k] = (mat[i][j] + k as i32 - 1) % 9 + 1;
            }
        }
    }
    for i in 0..mat.len() {
        for j in 0..weights.len() {
            for k in 1..5 {
                weights[i + mat.len()*k][j] = (weights[i][j] + k as i32 - 1) % 9 + 1;
            }
        }
    }
    weights
}


/// Dijkstra's shortest path to find least risky path
pub fn part1(inputs: &Vec<Vec<i32>>) -> i32 {
    dijkstra_shortest_path(inputs).0
}

/**Find the first step where all octopuses flash simultaneously */
pub fn part2(inputs: &Vec<Vec<i32>>) -> i32 {
    let weights = inflate(inputs, 5);
    dijkstra_shortest_path(&weights).0
}

/* TESTS */
#[allow(unused)]
static TEST_INPUT: &'static [&str] = &[
    "1163751742",
    "1381373672",
    "2136511328",
    "3694931569",
    "7463417111",
    "1319128137",
    "1359912421",
    "3125421639",
    "1293138521",
    "2311944581",
];

#[test]
fn test_day15_part1() {
    let inputs = TEST_INPUT.iter().map(|line|parse_line(*line)).collect::<Vec<_>>();
    assert_eq!(part1(&inputs), 40);
}

#[test]
fn test_day15_part2() {
    let inputs = TEST_INPUT.iter().map(|line|parse_line(*line)).collect::<Vec<_>>();
    assert_eq!(part2(&inputs), 315);
}

#[test]
fn test_day15_parse_line() {
    assert_eq!(parse_line(TEST_INPUT[0]), vec![1,1,6,3,7,5,1,7,4,2]);
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
