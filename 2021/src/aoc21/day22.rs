static DAY: i32 = 22;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum State { On, Off }
impl Default for State {
    fn default() -> Self { State::Off }
}
#[derive(Copy, Clone, Default, Debug)]
pub struct Instr {
    state: State,
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
}
impl Instr {
    pub fn intersect(&self, other: &Self) -> bool {
        !(self.x.1 < other.x.0 || other.x.1 < self.x.0) &&
        !(self.y.1 < other.y.0 || other.y.1 < self.y.0) &&
        !(self.z.1 < other.z.0 || other.z.1 < self.z.0)
    }
    pub fn volume(&self) -> isize {
        (self.x.1 - self.x.0 + 1) * (self.y.1 - self.y.0 + 1) * (self.z.1 - self.z.0 + 1)
    }
}

pub fn input(test: i32) -> Vec<Instr> {
    use std::io::{prelude::*, BufReader};
    let f = match test {
        0 => crate::aoc::_test_input_file(DAY, 0),
        1 => crate::aoc::_test_input_file(DAY, 1),
        2 => crate::aoc::_test_input_file(DAY, 2),
        _ => crate::aoc::input_file(DAY)
    };
    let f = BufReader::new(f.unwrap());
    f.lines().map(parse_line).collect::<Vec<_>>()
}

fn parse_line(line: Result<String, std::io::Error>) -> Instr {
    let line = line.unwrap();
    let mut line = line.split_ascii_whitespace();
    let state = match line.next().unwrap() {
        "on" => State::On,
        "off" => State::Off,
        &_ => unreachable!(),
    };
    let (x,y,z) = match sscanf::scanf!(line.next().unwrap(), "x={}..{},y={}..{},z={}..{}", isize,isize,isize,isize,isize,isize) {
        None => panic!(),
        Some((x0,x1,y0,y1,z0,z1)) => ((x0,x1), (y0,y1), (z0,z1))
    };
    Instr { state, x, y, z }
}

pub fn part1(inputs: &Vec<Instr>) -> u64 {
    let mut boot = vec![vec![vec![State::Off; 101]; 101]; 101];

    for i in inputs {
        for x in (i.x.0..=i.x.1).filter(|n| (-50..=50).contains(n)).map(|n| n+50) {
            for y in (i.y.0..=i.y.1).filter(|n| (-50..=50).contains(n)).map(|n| n+50) {
                for z in (i.z.0..=i.z.1).filter(|n| (-50..=50).contains(n)).map(|n| n+50) {
                    boot[x as usize][y as usize][z as usize] = i.state;
                }
            }
        }
    }
    boot.iter()
    .map(|grid| grid.iter()
        .map(|line| line.iter().filter(|&&s| s == State::On).count() as u64).sum::<u64>()
    )
    .sum::<u64>()
}

macro_rules! split_cube {
    ($axis:ident, $d:ident, $split:ident, $cur:ident) => {
        // Split cube c into as few sub-cubes as possible
        if $cur.$axis.0 >= $d.$axis.0 && $cur.$axis.1 <= $d.$axis.1 {
            // Around
            // Continue with all
        }
        if $cur.$axis.0 <= $d.$axis.0 && $cur.$axis.1 >= $d.$axis.1 {
            // Inside
            // Split below
            if $d.$axis.0-1 >= $cur.$axis.0 {
                $split.push(Instr { $axis: ($cur.$axis.0, $d.$axis.0-1), ..$cur });
            }
            // Split above
            if $cur.$axis.1 >= $d.$axis.1+1 {
                $split.push(Instr { $axis: ($d.$axis.1+1, $cur.$axis.1), ..$cur });
            }
            // Continue with middle
            $cur.$axis.0 = $d.$axis.0;
            $cur.$axis.1 = $d.$axis.1;
        }
        if $cur.$axis.0 >= $d.$axis.0 && $cur.$axis.0 <= $d.$axis.1 && $cur.$axis.1 >= $d.$axis.1 {
            // Overlap high
            // Split above
            if $cur.$axis.1 >= $d.$axis.1+1 {
                $split.push(Instr { $axis: ($d.$axis.1+1, $cur.$axis.1), ..$cur });
            }
            // Continue with bottom
            $cur.$axis.1 = $d.$axis.1;
        }
        if $cur.$axis.1 >= $d.$axis.0 && $cur.$axis.1 <= $d.$axis.1 && $cur.$axis.0 <= $d.$axis.0 {
            // Overlap low
            // Split below
            if $d.$axis.0-1 >= $cur.$axis.0 {
                $split.push(Instr { $axis: ($cur.$axis.0, $d.$axis.0-1), ..$cur });
            }
            // Continue with top
            $cur.$axis.0 = $d.$axis.0;
        }
    };
}

/// Splitting cubes method. Split all intersecting cubes so all cubes in the set
/// are either perfectly overlapping or not intersecting at all, thus giving the
/// total volume by only counting the last occurence of each volume.
/// ```md
/// for each cube `c0` with state On from `inputs`
///     let split_set contain the cube
///     for each cube `c1` intersecting `c0` in `inputs`
///         let split_set_nxt be an empty set
///         for each cube `c2` in `split_set`
///             if `c1` intersects `c2`
///                 check each axis, and split the cube on intersection points,
///                 adding the volumes outside `c1` to split_set_nxt
///             else
///                 keep `c2`, add in to `split_set_nxt`
///         assign split_set_nxt to split_set
///     add the sum of volumes in `split_set` to the total summed volume
/// ```
pub fn part2(inputs: &Vec<Instr>) -> u64 {

    // Splitting cubes
    let mut summed_volume = 0;
    for (i, &c0) in inputs.iter().enumerate().filter(|(_,c)| matches!(c.state, State::On)) {
        let mut split_set = vec![c0];
        // Iterate all intersecting volumes coming later in the inputs
        for c1 in inputs.iter().skip(i+1).filter(|c1| c0.intersect(c1)) {
            let mut split_set_nxt = Vec::new();
            for mut cur in split_set {
                // Shave off cubes so the c0 volume intersecting c1 is deleted
                if cur.intersect(c1) {
                    split_cube!(x, c1, split_set_nxt, cur);
                    split_cube!(y, c1, split_set_nxt, cur);
                    split_cube!(z, c1, split_set_nxt, cur);
                }
                else {
                    // Do nothing if the resulting cube no longer intersects c1
                    split_set_nxt.push(cur);
                }
            }
            split_set = split_set_nxt;
        }
        // Content of split now has no more intersecting volumes
        summed_volume += split_set.iter().map(|c| c.volume()).sum::<isize>();
    }
    summed_volume as u64
}


/* TESTS */
#[test]
fn test_day22_split_cube() {
    let mut split = Vec::new();
    let mut cur = Instr {
        state: State::On,
        x: (0, 10),
        y: (0, 10),
        z: (0, 10)
    };
    let d = Instr {
        state: State::On,
        x: (5, 15),
        y: (5, 15),
        z: (-5, 5)
    };
    split_cube!(x, d, split, cur);
    split_cube!(y, d, split, cur);
    split_cube!(z, d, split, cur);
    println!("{:?}", split);
}

#[test]
fn test_day22_part1() {
    let inputs = input(0);
    assert_eq!(part1(&inputs), 39);
    let inputs = input(1);
    assert_eq!(part1(&inputs), 590784);
}

#[test]
fn test_day22_part2() {
    let inputs = input(0);
    let res = part2(&inputs);
    assert_eq!(res, 39);
    let inputs = input(1);
    let res = part2(&inputs);
    assert_eq!(res, 39_769_202_357_779_u64);
    let inputs = input(2);
    let res = part2(&inputs);
    assert_eq!(res, 2_758_514_936_282_235_u64);
}


#[allow(unused)]
pub fn run() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . .");
    let inputs = input(-1);
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
// 15309606157580578 too high
