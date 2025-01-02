use std::collections::HashMap;

use super::YEAR;
static DAY: usize = 21;


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum KeyPadKey {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
}
impl KeyPadKey {
    fn new(key: char) -> Self {
        match key {
            '0' => KeyPadKey::Key0,
            '1' => KeyPadKey::Key1,
            '2' => KeyPadKey::Key2,
            '3' => KeyPadKey::Key3,
            '4' => KeyPadKey::Key4,
            '5' => KeyPadKey::Key5,
            '6' => KeyPadKey::Key6,
            '7' => KeyPadKey::Key7,
            '8' => KeyPadKey::Key8,
            '9' => KeyPadKey::Key9,
            'A' => KeyPadKey::KeyA,
            _ => panic!("Not a valid keypad key."),
        }
    }
    fn adj(&self, dir: ArrowPadKey) -> Option<Self> {
        use KeyPadKey::*;
        match &self {
            KeyPadKey::Key0 => match dir {
                ArrowPadKey::Left => None,
                ArrowPadKey::Right => Some(KeyA),
                ArrowPadKey::Up => Some(Key2),
                ArrowPadKey::Down => None,
                ArrowPadKey::Ack => Some(Key0),
            },
            KeyPadKey::Key1 => match dir {
                ArrowPadKey::Left => todo!(),
                ArrowPadKey::Right => todo!(),
                ArrowPadKey::Up => todo!(),
                ArrowPadKey::Down => todo!(),
                ArrowPadKey::Ack => todo!(),
            },
            KeyPadKey::Key2 => match dir {
                ArrowPadKey::Left => todo!(),
                ArrowPadKey::Right => todo!(),
                ArrowPadKey::Up => todo!(),
                ArrowPadKey::Down => todo!(),
                ArrowPadKey::Ack => todo!(),
            },
            KeyPadKey::Key3 => match dir {
                ArrowPadKey::Left => todo!(),
                ArrowPadKey::Right => todo!(),
                ArrowPadKey::Up => todo!(),
                ArrowPadKey::Down => todo!(),
                ArrowPadKey::Ack => todo!(),
            },
            KeyPadKey::Key4 => match dir {
                ArrowPadKey::Left => todo!(),
                ArrowPadKey::Right => todo!(),
                ArrowPadKey::Up => todo!(),
                ArrowPadKey::Down => todo!(),
                ArrowPadKey::Ack => todo!(),
            },
            KeyPadKey::Key5 => match dir {
                ArrowPadKey::Left => todo!(),
                ArrowPadKey::Right => todo!(),
                ArrowPadKey::Up => todo!(),
                ArrowPadKey::Down => todo!(),
                ArrowPadKey::Ack => todo!(),
            },
            KeyPadKey::Key6 => match dir {
                ArrowPadKey::Left => todo!(),
                ArrowPadKey::Right => todo!(),
                ArrowPadKey::Up => todo!(),
                ArrowPadKey::Down => todo!(),
                ArrowPadKey::Ack => todo!(),
            },
            KeyPadKey::Key7 => match dir {
                ArrowPadKey::Left => todo!(),
                ArrowPadKey::Right => todo!(),
                ArrowPadKey::Up => todo!(),
                ArrowPadKey::Down => todo!(),
                ArrowPadKey::Ack => todo!(),
            },
            KeyPadKey::Key8 => match dir {
                ArrowPadKey::Left => todo!(),
                ArrowPadKey::Right => todo!(),
                ArrowPadKey::Up => todo!(),
                ArrowPadKey::Down => todo!(),
                ArrowPadKey::Ack => todo!(),
            },
            KeyPadKey::Key9 => match dir {
                ArrowPadKey::Left => todo!(),
                ArrowPadKey::Right => todo!(),
                ArrowPadKey::Up => todo!(),
                ArrowPadKey::Down => todo!(),
                ArrowPadKey::Ack => todo!(),
            },
            KeyPadKey::KeyA => match dir {
                ArrowPadKey::Left => todo!(),
                ArrowPadKey::Right => todo!(),
                ArrowPadKey::Up => todo!(),
                ArrowPadKey::Down => todo!(),
                ArrowPadKey::Ack => todo!(),
            },
        }
    }
}
impl std::fmt::Display for KeyPadKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match &self {
            KeyPadKey::Key0 => '0',
            KeyPadKey::Key1 => '1',
            KeyPadKey::Key2 => '2',
            KeyPadKey::Key3 => '3',
            KeyPadKey::Key4 => '4',
            KeyPadKey::Key5 => '5',
            KeyPadKey::Key6 => '6',
            KeyPadKey::Key7 => '7',
            KeyPadKey::Key8 => '8',
            KeyPadKey::Key9 => '9',
            KeyPadKey::KeyA => 'A',
        })
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ArrowPadKey {
    Left,
    Right,
    Up,
    Down,
    Ack,
}
impl ArrowPadKey {
    pub fn reverse(&self) -> Self {
        use ArrowPadKey::*;
        match &self {
            Left => Right,
            Right => Left,
            Up => Down,
            Down => Up,
            Ack => Ack,
        }
    }
}
impl std::fmt::Display for ArrowPadKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                ArrowPadKey::Left => '<',
                ArrowPadKey::Right => '>',
                ArrowPadKey::Up => '^',
                ArrowPadKey::Down => 'v',
                ArrowPadKey::Ack => 'A',
            }
        )
    }
}

/// ```md
/// +---+---+---+
/// | 7 | 8 | 9 |
/// +---+---+---+
/// | 4 | 5 | 6 |
/// +---+---+---+
/// | 1 | 2 | 3 |
/// +---+---+---+
///     | 0 | A |
///     +---+---+
/// ```
fn keypad(from: KeyPadKey, to: KeyPadKey) -> Vec<ArrowPadKey> {
    use ArrowPadKey::*;
    use KeyPadKey::*;
    if from == to {
        return Vec::new();
    }
    match from {
        Key0 => match to {
            Key1 => vec![Up, Left],
            Key2 => vec![Up],
            Key3 => vec![Up, Right],
            Key4 => vec![Up, Up, Left],
            Key5 => vec![Up, Up],
            Key6 => vec![Up, Up, Right],
            Key7 => vec![Up, Up, Up, Left],
            Key8 => vec![Up, Up, Up],
            Key9 => vec![Up, Up, Up, Right],
            KeyA => vec![Right],
            _ => keypad(to, from).into_iter().rev().map(|a| a.reverse()).collect(),
        },
        Key1 => match to {
            Key2 => vec![Right],
            Key3 => vec![Right, Right],
            Key4 => vec![Up],
            Key5 => vec![Right, Up],
            Key6 => vec![Right, Right, Up],
            Key7 => vec![Up, Up],
            Key8 => vec![Right, Up, Up],
            Key9 => vec![Right, Right, Up, Up],
            KeyA => vec![Right, Right, Down],
            _ => keypad(to, from).into_iter().rev().map(|a| a.reverse()).collect(),
        },
        Key2 => match to {
            Key3 => vec![Right],
            Key4 => vec![Left, Up],
            Key5 => vec![Up],
            Key6 => vec![Up, Right],
            Key7 => vec![Up, Up, Left],
            Key8 => vec![Up, Up],
            Key9 => vec![Up, Up, Right],
            KeyA => vec![Down, Right],
            _ => keypad(to, from).into_iter().rev().map(|a| a.reverse()).collect(),
        },
        Key3 => match to {
            Key4 => vec![Up, Left, Left],
            Key5 => vec![Up, Left],
            Key6 => vec![Up],
            Key7 => vec![Up, Up, Left, Left],
            Key8 => vec![Up, Up, Left],
            Key9 => vec![Up, Up],
            KeyA => vec![Down],
            _ => keypad(to, from).into_iter().rev().map(|a| a.reverse()).collect(),
        },
        Key4 => match to {
            Key5 => vec![Right],
            Key6 => vec![Right, Right],
            Key7 => vec![Up],
            Key8 => vec![Up, Right],
            Key9 => vec![Up, Right, Right],
            KeyA => vec![Right, Right, Down, Down],
            _ => keypad(to, from).into_iter().rev().map(|a| a.reverse()).collect(),
        },
        Key5 => match to {
            Key6 => vec![Right],
            Key7 => vec![Up, Left],
            Key8 => vec![Up],
            Key9 => vec![Up, Right],
            KeyA => vec![Right, Down, Down],
            _ => keypad(to, from).into_iter().rev().map(|a| a.reverse()).collect(),
        },
        Key6 => match to {
            Key7 => vec![Up, Left, Left],
            Key8 => vec![Up, Left],
            Key9 => vec![Up],
            KeyA => vec![Down, Down],
            _ => keypad(to, from).into_iter().rev().map(|a| a.reverse()).collect(),
        },
        Key7 => match to {
            Key8 => vec![Right],
            Key9 => vec![Right, Right],
            KeyA => vec![Right, Right, Down, Down, Down],
            _ => keypad(to, from).into_iter().rev().map(|a| a.reverse()).collect(),
        },
        Key8 => match to {
            Key9 => vec![Right],
            KeyA => vec![Right, Down, Down, Down],
            _ => keypad(to, from).into_iter().rev().map(|a| a.reverse()).collect(),
        },
        Key9 => match to {
            KeyA => vec![Down, Down, Down],
            _ => keypad(to, from).into_iter().rev().map(|a| a.reverse()).collect(),
        },
        KeyA => match to {
            _ => keypad(to, from).into_iter().rev().map(|a| a.reverse()).collect(),
        },
    }
}

/// Generate the needed sequence of arrow keys needed to move the cursor from
/// `from` to `to`.
///```md
///     +---+---+
///     | ^ | A |
/// +---+---+---+
/// | < | v | > |
/// +---+---+---+
/// ```
fn arrowpad(from: ArrowPadKey, to: ArrowPadKey) -> Vec<ArrowPadKey> {
    use ArrowPadKey::*;
    if from == to {
        return Vec::new();
    }
    match from {
        Left => match to {
            Right => vec![Right, Right],
            Down => vec![Right],
            Up => vec![Right, Up],
            Ack => vec![Right, Right, Up],
            _ => Vec::new(),
        },
        Right => match to {
            Up => vec![Left, Up],
            Down => vec![Left],
            Ack => vec![Up],
            _ => arrowpad(to, from)
                .into_iter()
                .rev()
                .map(|a| a.reverse())
                .collect(),
        },
        Up => match to {
            Down => vec![Down],
            Ack => vec![Right],
            _ => arrowpad(to, from)
                .into_iter()
                .rev()
                .map(|a| a.reverse())
                .collect(),
        },
        Down => match to {
            Ack => vec![Up, Right],
            _ => arrowpad(to, from)
                .into_iter()
                .rev()
                .map(|a| a.reverse())
                .collect(),
        },
        Ack => arrowpad(to, from)
            .into_iter()
            .rev()
            .map(|a| a.reverse())
            .collect(),
    }
}

fn mv(pos: &mut Vec<(usize, usize)>, sequences: &mut Vec<Vec<ArrowPadKey>>, i: usize) -> bool {
    if i == sequences.len() { return true; }
    println!("  {} pressed {:?}", i, sequences[i][0]);
    match sequences[i].remove(0) {
        ArrowPadKey::Left => pos[i+1].0 -= 1,
        ArrowPadKey::Right => pos[i+1].0 += 1,
        ArrowPadKey::Up => pos[i+1].1 -= 1,
        ArrowPadKey::Down => pos[i+1].1 += 1,
        ArrowPadKey::Ack => return mv(pos, sequences, i+1),
    }
    false
}

// Print:
//             789
//             456
//  ^A |  ^A | 123
// <v> | <v> |  0A
fn visualize(code: Vec<KeyPadKey>, sequences: Vec<Vec<ArrowPadKey>>) {
    let mut sequences = sequences.into_iter().rev().collect::<Vec<Vec<_>>>();
    let mut key = (2, 3);
    let mut pos = vec![(2, 0); sequences.len()];
    pos.push(key);

    let arrowpad_s = [['.', '^', 'A'], ['<', 'v', '>']];
    let keypad_s = [['7','8','9'], ['4','5','6'], ['1','2','3'],['.','0','A']];

    // Insert A as starting position
    for seq in sequences.iter_mut().skip(1) {
        seq.insert(0, ArrowPadKey::Ack);
    }

    for (i, seq) in sequences.iter().enumerate() {
        print!(" {} ", arrowpad_s[pos[i].1][pos[i].0]);
    }
    println!(" {} \n", keypad_s[pos[sequences.len()].1][pos[sequences.len()].0]);

    while sequences[0].len() > 0 {
        if mv(&mut pos, &mut sequences, 0) {
            println!("  Keypad {} pressed", keypad_s[pos[sequences.len()].1][pos[sequences.len()].0]);
        }
        for (i, seq) in sequences.iter().enumerate() {
            print!(" {} ", arrowpad_s[pos[i].1][pos[i].0]);
        }
        println!(" {} \n", keypad_s[pos[sequences.len()].1][pos[sequences.len()].0]);
    }
}

// Get directions
// For each order (horizonta

fn input() -> String {
    crate::aoc::input_raw(YEAR, DAY)
}

fn compute_sequence(seq0: &str, n_robots: usize) -> Vec<ArrowPadKey> {
    use ArrowPadKey::*;
    use KeyPadKey::*;

    // Parse code into sequence
    let seq0: Vec<_> = seq0.chars().map(KeyPadKey::new).collect();
    println!("code : {:?}", seq0);
    // Get first arrow sequence
    // [[Up, Up, Left, Ack], [Right, Ack], [Ack]]
    let mut seq: Vec<ArrowPadKey> = Vec::new();
    seq.extend(keypad(KeyA, seq0[0]));
    seq.push(Ack);
    seq.extend(keypad(seq0[0], seq0[1]));
    seq.push(Ack);
    seq.extend(keypad(seq0[1], seq0[2]));
    seq.push(Ack);
    seq.extend(keypad(seq0[2], seq0[3]));
    seq.push(Ack);

    println!("seq 0: {:?}", seq);

    let mut seq_v = vec![seq.clone()];

    // Order of vertical and horizontal move does not affect sequence length,
    // as the preceeding
    // seq2: move to v0 (press it n times), move to A (press it for keypress),
    // arrowpad(A, v0), arrowpad(v0, h0)
    for i in 0..n_robots {
        let mut seq_nxt: Vec<_> = arrowpad(Ack, seq[0]);
        seq_nxt.push(Ack);
        for j in 1..seq.len() {
            seq_nxt.extend(arrowpad(seq[j - 1], seq[j]));
            seq_nxt.push(Ack);
        }
        seq = seq_nxt;
        seq_v.push(seq.clone());
        println!("seq 0: {:?}", seq);
    }
    visualize(seq0, seq_v);
    seq
}

/// For each sequence:
///
fn part1(inputs: &str) -> usize {
    let n_robots = 2; // 2 robots + 1 manual user
    let mut sum = 0;
    for seq0 in inputs.lines().map(|l| l.trim()) {
        // Parse numeric value of code
        let code_val = seq0[..2].parse::<usize>().unwrap();
        let seq = compute_sequence(seq0, n_robots);
        sum += seq.len() * code_val;
    }
    sum
}
// 16426 too low
// 16330 too low

fn part2(inputs: &str) -> u32 {
    todo!();
}

lazy_static::lazy_static! {
    static ref TEST_INPUT: Vec<Vec<char>> = vec![
        vec!['0', '2', '9', 'A'],
        vec!['9', '8', '0', 'A'],
        vec!['1', '7', '9', 'A'],
        vec!['4', '5', '6', 'A'],
        vec!['3', '7', '9', 'A'],
    ];
}

#[test]
fn test_2024_day21_part1() {
    let tests = [
        (
            "029A",
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A",
        ),
        (
            "980A",
            "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A",
        ),
        (
            "179A",
            "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
        ),
        (
            "456A",
            "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A",
        ),
        (
            "379A",
            "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
        ),
    ];
    assert_eq!(
        compute_sequence(tests[0].0, 2)
            .iter()
            .map(|k| format!("{}", k))
            .collect::<Vec<_>>()
            .join("").len(),
        tests[0].1.len()
    );
}

#[test]
fn test_2024_day21_part2() {
    // TODO
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
    print!("{} Day {} part 1: ", YEAR, DAY);
    println!("{}", res);
    println!("Took {:?}", pt_start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    let res = part2(&inputs);
    print!("{} Day {} part 2: ", YEAR, DAY);
    println!("{}", res);
    println!("Took {:?}", pt_start.elapsed().unwrap());
    println!("Total time: {:?}", start.elapsed().unwrap());
}
