static DAY: i32 = 10;

pub fn input() -> Vec<String> {
    crate::aoc::input(DAY)
}

/**Parse a line and report corrupted lines */
fn parse_line1(line: &str) -> i32 {
    let mut stack = Vec::new();

    for c in line.chars() {
        if "{[(<".contains(c) {
            stack.push(c);
        } else {
            if let Some(cc) = stack.pop() {
                match (cc, c) {
                    ('[',']') | ('{','}') | ('(',')') | ('<','>') => continue,
                    (_,')') => return 3,
                    (_,']') => return 57,
                    (_,'}') => return 1197,
                    (_,'>') => return 25137,
                    _ => return 0
                }
            } else {
                // Too many closing brackets
                return 0
            }
        }
    }
    0
}

/**Parse a line and autocomplete incomplete lines */
fn parse_line2(line: &str) -> u64 {
    let mut stack = Vec::new();

    for c in line.chars() {
        if "{[(<".contains(c) {
            stack.push(c);
        } else {
            if let Some(cc) = stack.pop() {
                match (cc, c) {
                    ('[',']') | ('{','}') | ('(',')') | ('<','>') => continue,
                    _ => return 0   // Discard corrupted lines
                }
            } else {
                // Too many closing brackets
                return 0
            }
        }
    }
    stack.iter().rev().fold(0, |acc, &c| acc * 5 + match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => unreachable!()
    })
}

pub fn part1(inputs: &Vec<String>) -> i32 {
    inputs.iter().map(|line| parse_line1(line)).sum::<i32>()
}

pub fn part2(inputs: &Vec<String>) -> u64 {
    let mut scores = inputs.iter()
        .map(|line| parse_line2(line))
        .filter(|&n| n != 0)
        .collect::<Vec<_>>();
    scores.sort();
    scores[scores.len() / 2]
}
// 349440 too low

/* TESTS */
#[allow(unused)]
static TEST_INPUT: &str = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

#[test]
fn test_day10_part1() {
    let inputs = TEST_INPUT.lines().map(|line|line.to_string()).collect::<Vec<_>>();
    assert_eq!(part1(&inputs), 26397);
}

#[test]
fn test_day10_part2() {
    let inputs = TEST_INPUT.lines().map(|line|line.to_string()).collect::<Vec<_>>();
    assert_eq!(part2(&inputs), 288957);
}

#[test]
fn test_day10_parse_line1() {
    let line = "{([(<{}[<>[]}>{[]{[(<()>";
    assert_eq!(parse_line1(line), 1197);
    let line = "[[<[([]))<([[{}[[()]]]";
    assert_eq!(parse_line1(line), 3);
}

#[test]
fn test_day10_parse_line2() {
    let line = "[({(<(())[]>[[{[]{<()<>>";
    assert_eq!(parse_line2(line), 288957);
    eprintln!("");
    let line = "[(()[<>])]({[<{<<[]>>(";
    assert_eq!(parse_line2(line), 5566);
    eprintln!("");
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
