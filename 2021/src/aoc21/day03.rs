use rayon::prelude::*;

pub fn input() -> Vec<String> {
    crate::aoc::input(3)
}

/**
 * Read gamma and epsilon from a list of binary numbers,
 * gamma being the most common bit value of each radix and
 * epsilon being gamma's bitwise compliment.
 * */
pub fn part1(inputs: &Vec<String>) -> u32 {
    let mid = inputs.len() / 2;   // threshold for majority
    let width = inputs[0].len();  // bit width of diagnostic numbers

    // One radix at the time
    let gamma = (0..width).into_par_iter()
        .map(
            // Count 1's
            |i| inputs.iter().filter(
                |line| line.chars().skip(i).next().unwrap() == '1'
            ).count()
        )
        .enumerate()
        .map(
            // Binary positions
            |(i, r)| if r > mid { u32::pow(2, (width-1-i) as u32) } else { 0 }
        )
        .sum::<u32>();

    // epsilon is compliment of gamma
    gamma * (gamma ^ (u32::pow(2, width as u32)-1))
}

/**
 * Determine "life support rating", the product of "oxygen generator rating" and
 * "CO2 scrubber rating". These two calculations are put in parallel threads.
 * */
pub fn part2(inputs: &Vec<String>) -> u32 {
    let width = inputs[0].len();  // bit width of diagnostic numbers

    let mut o2g = inputs.clone();   // O2 generator
    let mut co2s = inputs.clone();  // CO2 scrubber

    /* O2 generator */
    let o2 = std::thread::spawn(move || {
        for i in 0..width {
            if o2g.len() > 1 {
                let mcb = if o2g
                    .iter()
                    .filter(
                        |line| line.chars().skip(i).next().unwrap() == '1'
                    )
                    .count() as f64 >= o2g.len() as f64 / 2.0 { '1' } else { '0' };
                o2g = o2g.iter()
                    .filter(|line| line.chars().skip(i).next().unwrap() == mcb)
                    .map(|s| s.to_owned())
                    .collect();
            }
        }
        u32::from_str_radix(&o2g[0], 2).unwrap()
    });
    /* CO2 scrubber */
    let co2 = std::thread::spawn(move || {
        for i in 0..width {
            if co2s.len() > 1 {
                let mcb = if co2s
                    .iter()
                    .filter(
                        |line| line.chars().skip(i).next().unwrap() == '1'
                    )
                    .count() as f64 >= co2s.len() as f64 / 2.0 { '1' } else { '0' };
                co2s = co2s.iter()
                .filter(|line| line.chars().skip(i).next().unwrap() != mcb)
                .map(|s| s.to_owned())
                .collect();
            }
        }
        u32::from_str_radix(&co2s[0], 2).unwrap()
    });
    o2.join().unwrap() * co2.join().unwrap()
}

#[test]
fn test_day03_part1() {
    let inputs = vec![
        String::from("00100"), String::from("11110"), String::from("10110"), String::from("10111"),
        String::from("10101"), String::from("01111"), String::from("00111"), String::from("11100"),
        String::from("10000"), String::from("11001"), String::from("00010"), String::from("01010")
        ];
    assert_eq!(part1(&inputs), 198);
}

#[test]
fn test_day03_part2() {
    let inputs = vec![
        String::from("00100"), String::from("11110"), String::from("10110"), String::from("10111"),
        String::from("10101"), String::from("01111"), String::from("00111"), String::from("11100"),
        String::from("10000"), String::from("11001"), String::from("00010"), String::from("01010")
        ];
    assert_eq!(part2(&inputs), 230);
}

#[allow(unused)]
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
