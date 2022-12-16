static DAY: usize = 15;

pub fn input() -> Vec::<(isize, isize, isize, isize)> {
    crate::aoc::input_raw(15)
        .lines()
        .map(parse_line)
        .collect()
}
fn parse_line(line: &str) -> (isize, isize, isize, isize) {
    sscanf::scanf!(
        line.trim(),
        "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
        isize, isize, isize, isize
    ).unwrap()
}

/// Suboptimal but barely working solution
pub fn part1(inputs: &Vec<(isize, isize, isize, isize)>) -> usize {
    let (width, height) = inputs.into_iter().fold(((0,0),(0,0)), |(w,h), (sx,sy,bx,by)| (
        (w.0.min(*sx.min(bx)), w.1.max(*sx.max(bx))),
        (h.0.min(*sy.min(by)), h.1.max(*sy.max(by))),
    ));
    let target = if height.1 >= 2000000 { 2000000 } else { 10 }; // Test case
    println!("{:?} x {:?}\n", height, width);
    let mut target_row = std::collections::HashSet::new();
    // Fill coordinates on the target row where there cannot be a beacon
    for (sx,sy,bx,by) in inputs.iter() {
        // dist >= 0
        let dist = (bx-sx).abs() + (by-sy).abs();
        for d in -dist..=dist {
            if sy+d == target {
                // Intersects the target row
                // dx >= 0
                let dx = dist - d.abs();
                // println!("({},{}) : ({},{}), dist={}, dx={}", sx,sy,bx,by, dist, dx);
                for x in -dx..=dx {
                    target_row.insert(sx+x);
                }
                // println!("{:?}", target_row);
            }
        }
    }
    for (_,_,_,by) in inputs.iter() {
        if *by == target {
            target_row.remove(by);
        }
    }
    let mut target_row = target_row.iter().collect::<Vec<_>>();
    target_row.sort();
    // println!("{:?}", target_row);
    target_row.len()
}

pub fn part2(inputs: &Vec<(isize, isize, isize, isize)>) -> isize {
    let inputs = inputs.iter()
    .map(|(sx,sy,bx,by)| (*sx,*sy, (bx-sx).abs() + (by-sy).abs()))
    .collect::<Vec<_>>();
    let target = if inputs.len() > 15 { 4_000_000 } else { 20 }; // Test case

    for x in 0..target {
        for y in 0..target {
            if inputs.iter().all(|(sx,sy,d)| (x-sx).abs() + (y-sy).abs() > *d) {
                return x * 4_000_000 + y;
            }
        }
    }
    0
}

#[test]
fn test_day15_part1() {
    let inputs = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    Sensor at x=9, y=16: closest beacon is at x=10, y=16
    Sensor at x=13, y=2: closest beacon is at x=15, y=3
    Sensor at x=12, y=14: closest beacon is at x=10, y=16
    Sensor at x=10, y=20: closest beacon is at x=10, y=16
    Sensor at x=14, y=17: closest beacon is at x=10, y=16
    Sensor at x=8, y=7: closest beacon is at x=2, y=10
    Sensor at x=2, y=0: closest beacon is at x=2, y=10
    Sensor at x=0, y=11: closest beacon is at x=2, y=10
    Sensor at x=20, y=14: closest beacon is at x=25, y=17
    Sensor at x=17, y=20: closest beacon is at x=21, y=22
    Sensor at x=16, y=7: closest beacon is at x=15, y=3
    Sensor at x=14, y=3: closest beacon is at x=15, y=3
    Sensor at x=20, y=1: closest beacon is at x=15, y=3".lines()
    .map(parse_line)
    .collect();
    assert_eq!(part1(&inputs), 26);
}

#[test]
fn test_day15_part2() {
    let inputs = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    Sensor at x=9, y=16: closest beacon is at x=10, y=16
    Sensor at x=13, y=2: closest beacon is at x=15, y=3
    Sensor at x=12, y=14: closest beacon is at x=10, y=16
    Sensor at x=10, y=20: closest beacon is at x=10, y=16
    Sensor at x=14, y=17: closest beacon is at x=10, y=16
    Sensor at x=8, y=7: closest beacon is at x=2, y=10
    Sensor at x=2, y=0: closest beacon is at x=2, y=10
    Sensor at x=0, y=11: closest beacon is at x=2, y=10
    Sensor at x=20, y=14: closest beacon is at x=25, y=17
    Sensor at x=17, y=20: closest beacon is at x=21, y=22
    Sensor at x=16, y=7: closest beacon is at x=15, y=3
    Sensor at x=14, y=3: closest beacon is at x=15, y=3
    Sensor at x=20, y=1: closest beacon is at x=15, y=3".lines()
    .map(parse_line)
    .collect();
    assert_eq!(part2(&inputs), 56000011);
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


