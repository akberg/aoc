static DAY: usize = 08;

pub fn input() -> Vec<Vec<usize>> {
    crate::aoc::input_raw(super::YEAR, DAY)
        .lines()
        .map(|ls| ls.chars().map(|c| c as usize - '0' as usize).collect::<Vec<_>>())
        .collect()
}

/// Number of trees that are visible from the edges (no diagonal)
pub fn part1(inputs: &Vec<Vec<usize>>) -> usize {
    inputs.iter().enumerate()
    .map(|(y, row)| {
        row.iter().enumerate()
        .filter(|&(x,&e)| {
            // For each direction, check if all trees are lower in at least one
            // of them.
            (0..x).all(|x0| inputs[y][x0] < e) ||
            (0..y).all(|y0| inputs[y0][x] < e) ||
            (x == row.len()-1 || (x+1..row.len()).all(|x0| inputs[y][x0] < e)) ||
            (y == inputs.len()-1 || (y+1..row.len()).all(|y0| inputs[y0][x] < e))
        }).count()
    }).sum::<usize>()
}

/// Number of trees that are visible from each tree
pub fn part2(inputs: &Vec<Vec<usize>>) -> usize {
    inputs.iter().enumerate()
    .map(|(y, row)| {
        row.iter().enumerate()
        .map(|(x,&e)| {
            // For each direction, count to the first taller or equally tall
            // tree, then multiply the results.
            (x - (0..x).rev().find(|&x0| inputs[y][x0] >= e).unwrap_or(0)) *
            (y - (0..y).rev().find(|&y0| inputs[y0][x] >= e).unwrap_or(0)) *
            ((x+1..row.len()).find(|&x0| inputs[y][x0] >= e).unwrap_or(row.len()-1) - x) *
            ((y+1..inputs.len()).find(|&y0| inputs[y0][x] >= e).unwrap_or(inputs.len()-1) - y)
        }).max().unwrap()
    }).max().unwrap()
}

#[test]
fn test_day8_part1() {
    let inputs = vec![
        vec![3,0,3,7,3],
        vec![2,5,5,1,2],
        vec![6,5,3,3,2],
        vec![3,3,5,4,9],
        vec![3,5,3,9,0],
    ];
    assert_eq!(part1(&inputs), 21);
}

#[test]
fn test_day8_part2() {
    let inputs = vec![
        vec![3,0,3,7,3],
        vec![2,5,5,1,2],
        vec![6,5,3,3,2],
        vec![3,3,5,4,9],
        vec![3,5,3,9,0],
    ];
    assert_eq!(part2(&inputs), 8);
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


