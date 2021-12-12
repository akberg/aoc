static DAY: i32 = 11;

fn parse_line(line: &str) -> Vec<i32> {
    line.chars().map(|c| c as i32 - '0' as i32).collect::<_>()
}

pub fn input() -> Vec<Vec<i32>> {
    use std::io::{prelude::*, BufReader};
    let f = crate::aoc::input_file(DAY).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|line|parse_line(&line.unwrap())).collect::<_>()
}

/**Grid size */
const N: usize = 10;
const M: usize = 10;

/**Step in flashing octopus cycle */
pub fn step(grid: &mut Vec<Vec<i32>>) -> u64 {
    let mut flashes = 0;
    let mut changed = true;
    let mut flashed = [[false; N]; M];

    /* First, the energy level of each octopus increases by 1 */
    (0..M).for_each(|i| (0..N).for_each(|j| grid[i][j] += 1));
    /* Run step until no more octopuses have flashed */
    while changed {
        changed = false;
        for i in 0..M {
            for j in 0..N {
                if grid[i][j] > 9 && !flashed[i][j] {
                    flashes += 1;
                    changed = true;
                    flashed[i][j] = true;
                    [(0,-1),(0,1),(1,0),(-1,0),(1,1),(1,-1),(-1,1),(-1,-1)]
                    .iter()
                    // Edge filter
                    .filter(|(di,dj)| (i>0 || *di>-1) && (i<M-1 || *di<1) && (j>0 || *dj>-1) && (j<N-1 || *dj<1))
                    // Build points
                    .map(|(di,dj)| ((i as isize + di) as usize, (j as isize + dj) as usize))
                    .for_each(|(ii,jj)| {
                        grid[ii][jj] += 1;
                    });
                    
                }
            }
        }
    }
    (0..M).for_each(|i| (0..N).for_each(|j| if grid[i][j] > 9 { grid[i][j] = 0}));
    
    flashes
}

/**Run 100 steps and return sum of flashes */
pub fn part1(inputs: &Vec<Vec<i32>>) -> u64 {
    let mut sum = 0;
    let mut grid = inputs.clone();
    for i in 1..=100 {
        let part_sum = step(&mut grid);
        assert_eq!(true, part_sum <= 100);
        sum += part_sum;
        eprintln!("Step {}: {} flashes", i, part_sum);
    }
    sum
}

/**Find the first step where all octopuses flash simultaneously */
pub fn part2(inputs: &Vec<Vec<i32>>) -> u64 {
    let mut grid = inputs.clone();
    let mut i = 0;
    loop {
        i += 1;
        let flashes = step(&mut grid);
        eprintln!("Step {}: {} flashes", i, flashes);
        if flashes == 100 {
            return i
        }
    }
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
    assert_eq!(part1(&inputs), 1656);
}

#[test]
fn test_day11_part2() {
    let inputs = TEST_INPUT.iter().map(|line|parse_line(*line)).collect::<Vec<_>>();
    assert_eq!(part2(&inputs), 195);
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
