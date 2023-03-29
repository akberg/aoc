static DAY: i32 = 19;

use std::{collections::HashSet, hash::Hash};

use nalgebra_glm as glm;
type Point = glm::I32Vec3;

fn parse_line(line: &str) -> Option<Point> {
    if line.is_empty() {
        None
    } else {
        let mut pts = line.split(',').map(|s| s.parse::<i32>().unwrap());
        Some(Point::new(pts.next().unwrap(), pts.next().unwrap(), pts.next().unwrap()))
    }
}

pub fn input(test: bool) -> Vec<Vec<Point>> {
    let f = if test { crate::aoc::_test_input_raw(DAY, 0) } else { crate::aoc::input_raw(DAY) };
    let mut scanners = Vec::new();
    let mut beacons = Vec::new();
    let mut new_scanner = true;
    for line in f.lines() {
        if new_scanner {
            new_scanner = false;
            scanners.push(beacons);
            beacons = Vec::new();
        } else {
            match parse_line(line) {
                Some(pt) => beacons.push(pt),
                None => new_scanner = true,
            };
        }
    }
    scanners.push(beacons);
    scanners
}

/// Find the orientation t, if any, that makes s1 match with s0, return the
/// correctly transformed s0 or None
fn match_scanners(s0: &Vec<Point>, s1: &Vec<Point>) -> Option<Vec<Point>> {
    unimplemented!()
}

/// Build array of transformation matrices for all possible orientations
fn make_transformation_matrices() -> Vec<glm::TMat3<i32>> {
    let mat_i = glm::identity();

    let rot_x90 = glm::TMat3::new( //
         1, 0, 0,
         0, 0,-1,
         0, 1, 0
    );
    let rot_y90 = glm::TMat3::new( //
         0, 0, 1,
         0, 1, 0,
        -1, 0, 0
    );
    let rot_z90 = glm::TMat3::new( //
         0,-1, 0,
         1, 0, 0,
         0, 0, 1
    );

    let rot_x180 = glm::TMat3::new( //
         1, 0, 0,
         0, 1, 0,
         0, 0, 1
    );
    let rot_y180 = glm::TMat3::new( //
        -1, 0, 0,
         0, 1, 0,
         0, 0,-1
    );
    let rot_z180 = glm::TMat3::new( //
        -1, 0, 0,
         0,-1, 0,
         0, 0, 1
    );

    let rot_x270 = glm::TMat3::new( //
        1, 0, 0,
        0, 0, 1,
        0,-1, 0
   );
   let rot_y270 = glm::TMat3::new( //
        0, 0,-1,
        0, 1, 0,
        1, 0, 0
   );
   let rot_z270 = glm::TMat3::new( //
        0, 1, 0,
       -1, 0, 0,
        0, 0, 1
   );

    let mut transforms = HashSet::new();
    for x in [rot_x90, rot_x180, rot_x270, mat_i] {
        for y in [rot_y90, rot_y180, rot_y270, mat_i] {
            for z in [rot_z90, rot_z180, rot_z270, mat_i] {
                transforms.insert(x*y*z);
            }
        }
    }
    transforms.into_iter().collect()
}

///
pub fn part1(inputs: &Vec<Vec<Point>>) -> u64 {
    // Pool of unmatched scanners
    let mut unmatched = inputs.clone();
    // Pool of oriented and matched scanners
    let mut oriented = vec![unmatched.remove(0)];
    // 24 possible transformations of a scanner
    let transforms = make_transformation_matrices();
    println!("{:?}, {}", transforms, transforms.len());
    assert_eq!(transforms.iter().collect::<HashSet<_>>().len(), 24);

    while !unmatched.is_empty() {
        let scanner = unmatched.remove(0);
        'outer: for so in oriented.clone() {
            for t in &transforms {
                let st = scanner.iter().map(|&p| t * p).collect::<Vec<_>>();
                if let Some(st) = match_scanners(&so, &st) {
                    oriented.push(st);
                    break 'outer;
                }
            }
        }
        // Add back to pool if there still were no matches
        unmatched.push(scanner);
    }
    let mut beacons = HashSet::new();
    for scanner in oriented {
        for pt in scanner {
            beacons.insert(pt);
        }
    }
    beacons.len() as u64
}

/// Parse packet and compute its value
pub fn part2(inputs: &Vec<Vec<Point>>) -> u64 {
    0
}

/* TESTS */

#[test]
fn test_day19_part1() {
    let inputs = input(true);
    assert_eq!(part1(&inputs), 79);
}

#[test]
fn test_day19_part2() {
    let inputs = input(true);
    assert_eq!(part2(&inputs), 3);
}


#[allow(unused)]
pub fn run() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . .");
    let inputs = input(false);
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
