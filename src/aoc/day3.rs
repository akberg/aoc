#[allow(unused)]
pub fn input(inputs: &Vec<String>) -> Vec<Vec<bool>> {
    let mut ret = Vec::new();
    for line in inputs.iter() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c == '.');
        }
        ret.push(row);
    }
    ret
}

fn slope(inputs: &Vec<Vec<bool>>, dx: usize, dy: usize) -> usize {
    (0..).map(|i| (i*dx, i*dy))
        .take_while(|(_, y)| y < &inputs.len())
        .filter(|(x, y)| !inputs[*y][*x % inputs[0].len()])
        .count()
}

#[allow(unused)]
pub fn part1(inputs: &Vec<Vec<bool>>) -> usize {
    slope(inputs, 3, 1)
}

#[allow(unused)]
pub fn part2(inputs: &Vec<Vec<bool>>) -> usize {
    [
        (1, 1), (3, 1), (5, 1), (7, 1), (1, 2)
    ].iter()
        .map(|tup| slope(inputs, tup.0, tup.1))
        .product()
}


#[test]
fn test_input() {
    let test_str = vec![String::from("..#."), String::from("#...")];
    let correct = vec![vec![true, true, false, true], vec![false, true, true, true]];
    assert_eq!(correct, input(&test_str));
}

#[test]
fn test_day3_part1() {
    let inputs = input(&String::from("..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#")
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>());
    assert_eq!(7, part1(&inputs));
}

#[test]
fn test_day3_part2() {
    let inputs = input(&String::from("..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#")
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>());
    assert_eq!(336, part2(&inputs));
}


#[test]
fn run_day3() {
    let ifile = super::input(3);
    println!("Parsing input . . .");
    let inputs = input(&ifile);
    println!("Day 3 part 1:");
    println!("{}", part1(&inputs));
    println!("Day 3 part 2:");
    println!("{}", part2(&inputs));
}