
pub fn input() -> Vec<Vec<u32>> {
    crate::aoc::input_raw(1)
        .split("\n\n")
        .map(|ls|{
            ls.lines()
            .filter(|&s| s.trim() != "")
            .map(|x| x.parse::<u32>().unwrap())
            .collect()}
        )
        .collect()
}


/// Largest sum, O(n)
pub fn part1(inputs: &Vec<Vec<u32>>) -> u32 {
    inputs.iter().map(|ls| ls.iter().sum::<u32>()).max().unwrap_or(0)
}

/// Sum, sort, sum 3 largest
pub fn part2(inputs: &Vec<Vec<u32>>) -> u32 {
    let mut sums = inputs.iter().map(|ls| ls.iter().sum::<u32>()).collect::<Vec<_>>();
    sums.sort_by(|a,b| b.cmp(a));
    sums.iter().take(3).sum::<_>()
}

#[test]
fn test_day01_part1() {
    let inputs = vec![
        vec![1000, 2000, 3000],
        vec![4000],
        vec![5000, 6000],
        vec![7000, 8000, 9000],
        vec![10000]
    ];
    assert_eq!(part1(&inputs), 24000);
}

#[test]
fn test_day01_part2() {
    let inputs = vec![
        vec![1000, 2000, 3000],
        vec![4000],
        vec![5000, 6000],
        vec![7000, 8000, 9000],
        vec![10000]
    ];
    assert_eq!(part2(&inputs), 45000);
}

pub fn run() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . .");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 1 part 1: ");
    println!("{}", part1(&inputs));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 1 part 2: ");
    println!("{}", part2(&inputs));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    println!("Total time: {:?}", start.elapsed().unwrap());
}
