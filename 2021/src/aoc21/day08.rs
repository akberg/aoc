static DAY: i32 = 8;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct SegPattern {
    pattern: u8,
    len: u8,
}

fn parse_pattern(pattern: &str) -> SegPattern {
    SegPattern {
        pattern: pattern.chars()
                        .fold(0, |acc, c| acc + (1 << (c as u8 - 'a' as u8))),
        len: pattern.len() as u8,
    }
}

fn parse_line(line: &str) -> (Vec<SegPattern>, Vec<SegPattern>) {
    let mut line = line.split("|");
    (
        line.next().unwrap().trim().split_whitespace()
            .map(parse_pattern).collect::<Vec<_>>(),    // <- 10 unique signal patterns
        line.next().unwrap().trim().split_whitespace()
            .map(parse_pattern).collect::<Vec<_>>(),    // <- 4 signal patterns
    )
}

pub fn input() -> Vec<(Vec<SegPattern>, Vec<SegPattern>)> {
    use std::io::{prelude::*, BufReader};
    let f = crate::aoc::input_file(DAY).unwrap();
    let f = BufReader::new(f);
    f.lines()
        .map(|line| parse_line(&line.unwrap()))
        .collect::<_>()
}

/**Count occurences of the numbers with unique amount of active segments,
 * 2 (1), 3 (7), 4 (4), 7 (8)
 */
pub fn part1(inputs: &Vec<(Vec<SegPattern>, Vec<SegPattern>)>) -> usize {
    inputs.iter()
        .map(|(_notes, signals)| 
            signals.iter()
                .filter(|pattern| vec![2, 3, 4, 7].contains(&pattern.len))
                .count()
        )
        .sum::<usize>()
}

/**Use known patterns to deduce encoding of remaining patterns
 *  dddd    a: used by 1, 4, 7, 8 and 2, 3, 9, 0        present 8 times
 * e    a   b: used by 1, 4, 7, 8, and 3, 5, 6, 9, 0    present 9 times
 * e    a   c: used by 8, and 2, 3, 5, 6, 9, 0          the one used in only 8 that is present 7 times
 *  ffff    d: used by 7, 8, and 2, 3, 5, 6, 9, 0       the one used in only 7 (and 8), present 8 times
 * g    b   e: used by 4, 8, and 5, 6, 9, 0             the one used in only 4 that is present 6 times
 * g    b   f: used by 4, 8, and 2, 3, 5, 6, 9          the one used in only 4 that is present 7 times
 *  cccc    g: used by 8, and 2, 6, 0                   the one present 4 times
 * 
 * Segments: abcdeg, ab, acdfg, abcd, abef, bcdef, bcdefg, abd, abcdefg, abcdef
 */
fn make_map(patterns: &Vec<SegPattern>) -> [u8;10] {
    // Extract easy patterns
    let mut mapping = [0; 10];
    mapping[1] = (*patterns.iter().find(|p| p.len == 2 ).unwrap()).pattern;
    mapping[4] = (*patterns.iter().find(|p| p.len == 4 ).unwrap()).pattern;
    mapping[7] = (*patterns.iter().find(|p| p.len == 3 ).unwrap()).pattern;
    mapping[8] = (*patterns.iter().find(|p| p.len == 7 ).unwrap()).pattern;
    // Count occurences of each bit
    let mut occurences = [0; 7];
    for p in patterns {
        for i in 0..7 {
            occurences[i] += (p.pattern >> i) & 1;
        }
    }
    // Implement rules
    let a = 1 << (0..7).find(|&i| occurences[i] == 8 && (mapping[1] >> i & 1) == 1).unwrap();
    let b = 1 << (0..7).find(|&i| occurences[i] == 9).unwrap();
    let c = 1 << (0..7).find(|&i| occurences[i] == 7 && (mapping[4] >> i & 1) == 0).unwrap();
    let d = 1 << (0..7).find(|&i| occurences[i] == 8 && (mapping[1] >> i & 1) == 0).unwrap();
    let e = 1 << (0..7).find(|&i| occurences[i] == 6 && (mapping[4] >> i & 1) == 1).unwrap();
    let f = 1 << (0..7).find(|&i| occurences[i] == 7 && (mapping[4] >> i & 1) == 1).unwrap();
    let g = 1 << (0..7).find(|&i| occurences[i] == 4).unwrap();

    // New segment mapping
    mapping[0] = a + b + c + d + e + g;
    mapping[2] = a + c + d + f + g;
    mapping[3] = a + b + c + d + f;
    mapping[5] = b + c + d + e + f;
    mapping[6] = b + c + d + e + f + g;
    mapping[9] = a + b + c + d + e + f;
    // Return
    mapping
}

/**Return index is segmap of the pattern, which is the digit it represents */
fn decode_digit(pattern: SegPattern, segmap: &[u8; 10]) -> u8 {
    (0..10).find(|&i| segmap[i] == pattern.pattern).unwrap() as u8
}

pub fn part2(inputs: &Vec<(Vec<SegPattern>, Vec<SegPattern>)>) -> usize {
    inputs.iter()
        .map(|(notes, signals)| {
            let segmap = make_map(&notes);
            signals.iter()
                .enumerate()
                .fold(0, |acc, (i, n)| 
                    acc + decode_digit(*n, &segmap) as usize * usize::pow(10, 3-i as u32)
                )
        })
        .sum::<usize>()
}


/* TESTS */
#[allow(unused)]
static TEST_LINE: &'static str = "\
acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | \
cdfeb fcadb cdfeb cdbaf\
";
#[allow(unused)]
static TEST_INPUT: &'static [&str] = &[
    "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
    "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
    "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
    "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
    "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
    "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
    "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
    "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
    "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
    "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
];


#[test]
fn test_day08_part1() {
    let inputs = TEST_INPUT.iter()
        .map(|line| parse_line(*line)).collect::<Vec<_>>();
    assert_eq!(part1(&inputs), 26);
}

#[test]
fn test_day08_part2() {
    let inputs = TEST_INPUT.iter()
        .map(|line| parse_line(*line)).collect::<Vec<_>>();
    assert_eq!(part2(&inputs), 61229);
}

#[test]
fn test_day08_parse_line() {
    let line = parse_line(TEST_LINE);
    let patterns = vec![
        SegPattern { pattern: 0b1111111, len: 7 },  // 8
        SegPattern { pattern: 0b0111110, len: 5 }, 
        SegPattern { pattern: 0b1101101, len: 5 }, 
        SegPattern { pattern: 0b0101111, len: 5 }, 
        SegPattern { pattern: 0b0001011, len: 3 },  // 7
        SegPattern { pattern: 0b0111111, len: 6 },
        SegPattern { pattern: 0b1111110, len: 6 },
        SegPattern { pattern: 0b0110011, len: 4 },  // 4
        SegPattern { pattern: 0b1011111, len: 6 },
        SegPattern { pattern: 0b0000011, len: 2 },  // 1
    ];
    for i in 0..line.0.len() {
        assert_eq!(line.0[i], patterns[i]);
        eprintln!("{:?} == {:?} OK", line.0[i], patterns[i]);
    }
}

#[test]
fn test_day08_decode() {
    let line = parse_line(TEST_LINE);
    let segmap = make_map(&line.0);
    eprintln!("Segment mapping: {:?}", segmap);
    let number = line.1.iter()
    .enumerate()
    .fold(0, |acc, (i, n)| 
        acc + decode_digit(*n, &segmap) as usize * usize::pow(10, 3-i as u32)
    );
    assert_eq!(number, 5353);
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
