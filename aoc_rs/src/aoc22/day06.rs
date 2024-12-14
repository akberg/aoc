static DAY: usize = 06;


pub fn input() -> String {
    crate::aoc::input_raw(super::YEAR, DAY)
}

/// Find the first window of size `len` in which all characters are distinct,
/// and return the following index
fn find_marker(inputs: &Vec<char>, len: usize) -> u32 {
    (len-1..inputs.len())
    .filter(|&i| {
        (i-(len-2)..=i).all(|j| !inputs[i-(len-1)..j].contains(&inputs[j]))
    })
    .next().unwrap() as u32 + 1
}

pub fn part1(inputs: &str) -> u32 {
    let inputs = inputs.chars().collect::<Vec<_>>();
    find_marker(&inputs, 4)
}

pub fn part2(inputs: &str) -> u32 {
    let inputs = inputs.chars().collect::<Vec<_>>();
    find_marker(&inputs, 14)
}
// 2135 not correct

#[test]
fn test_day6_part1() {
    let inputs = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    assert_eq!(part1(inputs), 5);
    let inputs = "nppdvjthqldpwncqszvftbrmjlhg";
    assert_eq!(part1(inputs), 6);
    let inputs = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    assert_eq!(part1(inputs), 10);
    let inputs = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    assert_eq!(part1(inputs), 11);

}

#[test]
fn test_day6_part2() {
    let inputs = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    assert_eq!(part2(inputs), 19);
    let inputs = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    assert_eq!(part2(inputs), 23);
    let inputs = "nppdvjthqldpwncqszvftbrmjlhg";
    assert_eq!(part2(inputs), 23);
    let inputs = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    assert_eq!(part2(inputs), 29);
    let inputs = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    assert_eq!(part2(inputs), 26);
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


