
#[allow(unused)]
pub fn input() -> Vec<i64> {
    crate::aoc::input_raw(20, 9)
        .lines()
        .map(|i| i.parse::<i64>().unwrap())
        .collect()
}

fn find_weakness2(stream: &[i64], preamble: usize) -> i64 {
    stream[
        (preamble..stream.len())
            .find(|&i| !(i-preamble..i)
                .any(|x| (x+1..i)
                    .any(|y| stream[x]+stream[y]==stream[i]))).unwrap()
        ]
}
fn find_weakness(stream: &[i64], preamble: usize) -> i64 {
    for i in preamble..stream.len() as usize {
        let n = stream[i];
        let mut found = false;
        'outer: for x in i-preamble..i {
            for y in x+1..i {
                if stream[x]!=stream[y] && stream[x]+stream[y]==n {
                    found = true;
                    break 'outer
                }
            }
        }
        if !found { return n; }
    }
    panic!("No result")
}

fn employ_weakness(stream: &[i64], w: i64) -> i64 {
    for i in 0..stream.len() {
        let mut sum = 0;
        for j in i..stream.len() {
            sum += stream[j];
            if sum == w {
                return *stream[i..j].iter().min().unwrap() + *stream[i..j].iter().max().unwrap()
            }
            else if sum > w {
                break;
            }
        }
    }
    panic!("No result")
}

#[allow(unused)]
pub fn part1(inputs: &[i64]) -> i64 {
    find_weakness2(inputs, 25)
} // correct: 31161678

#[allow(unused)]
pub fn part2(inputs: &[i64]) -> i64 {
    employ_weakness(inputs, find_weakness(inputs, 25))
}


#[test]
fn test_day9_part1() {
    let inputs = vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576];

    assert_eq!(127, find_weakness(&inputs, 5));
    assert_eq!(127, find_weakness2(&inputs, 5));
}

#[test]
fn test_day9_part2() {
    let inputs = vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576];
    assert_eq!(62, part2(&inputs));
}

#[test]
fn run_day9() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . . ");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 9 part 1: ");
    println!("{} - in {:?}", find_weakness(&inputs, 25), pt_start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 9 part 1 (rev1): ");
    println!("{} - in {:?}", find_weakness2(&inputs, 25), pt_start.elapsed().unwrap());
    print!("Day 9 part 2: ");
    let pt_start = SystemTime::now();
    println!("{} - in {:?}", part2(&inputs), pt_start.elapsed().unwrap());
    println!("Total duration: {:?}", start.elapsed().unwrap())
}
