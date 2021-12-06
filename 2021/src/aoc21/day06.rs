use std::collections::VecDeque;

static DAY: i32 = 6;


pub fn input() -> Vec<u64> {
    use std::io::{prelude::*, BufReader};
    let f = crate::aoc::input_file(DAY).unwrap();
    let f = BufReader::new(f);
    f.lines().next().unwrap().unwrap()
        .split(",")
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<_>()

}

/**Evolve colony according to reproduction rules */
pub fn evolve(inputs: &Vec<u64>, days: usize) -> u64 {
    let mut ocean = VecDeque::from(vec![0;9]);

    for i in inputs {
        ocean[*i as usize] += 1;
    }

    (0..days).fold(ocean, |mut ocean, _i| 
        {
            let new_fish = ocean.pop_front().unwrap();
            ocean.push_back(new_fish);
            ocean[6] += new_fish;
            ocean
        }).iter().sum::<_>()
}

/**Run evolve for 80 days */
pub fn part1(inputs: &Vec<u64>) -> u64 {
    evolve(inputs, 80)
}

/**Run evolve for 256 days */
pub fn part2(inputs: &Vec<u64>) -> u64 {
    evolve(inputs, 256)
}


/* TESTS */
#[allow(unused)]
static TEST_NUMBERS: &'static [u64] = &[3,4,3,1,2];

#[test]
fn test_day06_part1() {
    let inputs = Vec::from(TEST_NUMBERS);
    assert_eq!(part1(&inputs), 5934);
}

#[test]
fn test_day06_part2() {
    let inputs = Vec::from(TEST_NUMBERS);
    assert_eq!(part2(&inputs), 26984457539);
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
