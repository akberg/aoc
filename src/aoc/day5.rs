
#[allow(unused)]
pub fn row_col(line: &str) -> usize {
    line.chars()
        .fold((0, 1), |(acc, i), c| 
        (
            acc + (match c{'F'|'L'=>0, 'B'|'R'=>1,_=>0}<<(line.len()-i)), 
            i+1
        )
    ).0
}

#[allow(unused)]
pub fn part1(inputs: &Vec<usize>, sorted: bool) -> usize {
    if sorted {
        *inputs.last().unwrap()
    } else {
        *inputs.iter()
            .max()
            .unwrap()
    }
}

#[allow(unused)]
pub fn part2(inputs: &Vec<usize>) -> usize {
    (0..inputs.len()-2)
        .skip_while(|&i| inputs[i] + 2 != inputs[i+1])
        .map(|i| inputs[i] + 1)
        .collect::<Vec<usize>>()[0]
}

#[test]
fn test_rowcol() {
    assert_eq!(70, row_col("BFFFBBF"));
    assert_eq!(7, row_col("RRR"));
    assert_eq!(14, row_col("FFFBBBF"));
    assert_eq!(7, row_col("RRR"));
    assert_eq!(102, row_col("BBFFBBF"));
    assert_eq!(4, row_col("RLL"))
}

#[test]
fn test_day5_part1() {
    let inputs = String::from("BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL")
        .lines()
        //.map(|s| s.to_string())
        .map(|line| row_col(&line[0..7]) * 8 + row_col(&line[7..]))
        .collect::<Vec<usize>>();
    assert_eq!(820, part1(&inputs, false));
}

#[test]
fn run_day5() {
    println!("Parsing input . . . ");
    let mut inputs: Vec<usize> = super::input(5)
        .iter()
        .map(|line| row_col(&line[0..7]) * 8 + row_col(&line[7..]))
        .collect::<Vec<usize>>();
    inputs.sort();
    println!("Day 5 part 1: ");
    println!("{}", part1(&inputs, true));
    println!("Day 5 part 2: ");
    println!("{}", part2(&inputs));
}