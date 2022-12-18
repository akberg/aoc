static DAY: usize = 18;

type Surface = (usize, usize, usize);

pub fn input() -> Vec<Surface> {
    crate::aoc::input_raw(18)
        .lines()
        .map(|line| sscanf::scanf!(line, "{},{},{}", usize, usize, usize).unwrap())
        .collect()
}

/// Decrease surface area of single cubes for each adjacent cube
pub fn part1(inputs: &Vec<Surface>) -> u32 {
    let mut sides = vec![6; inputs.len()];
    for (i, cube) in inputs.into_iter().enumerate() {
        for (j, other) in inputs.into_iter().enumerate().skip(i) {
            if (cube.0 as isize - other.0 as isize).abs()
                + (cube.1 as isize - other.1 as isize).abs()
                + (cube.2 as isize - other.2 as isize).abs() == 1 {
                sides[i] -= 1;
                sides[j] -= 1;
            }
        }
    }
    sides.iter().sum()
}

fn flood_fill(pt: Surface, map: &Vec<Vec<Vec<usize>>>, vis: &mut Vec<Vec<Vec<bool>>>) -> usize {
    if vis[pt.0][pt.1][pt.2] {
        return 0;
    }
    vis[pt.0][pt.1][pt.2] = true;
    map[pt.0][pt.1][pt.2]
    + if pt.0 == map.len()-1 { 0 } else {       flood_fill((pt.0+1,pt.1,pt.2), map, vis) }
    + if pt.0 == 0 { 0 } else {                 flood_fill((pt.0-1,pt.1,pt.2), map, vis) }
    + if pt.1 == map[0].len()-1 { 0 } else {    flood_fill((pt.0,pt.1+1,pt.2), map, vis) }
    + if pt.1 == 0 { 0 } else {                 flood_fill((pt.0,pt.1-1,pt.2), map, vis) }
    + if pt.2 == map[0][0].len()-1 { 0 } else { flood_fill((pt.0,pt.1,pt.2+1), map, vis) }
    + if pt.2 == 0 { 0 } else {                 flood_fill((pt.0,pt.1,pt.2-1), map, vis) }
}


/// Count adjacent cubes for every cell, then flood fill from corner to sum
/// outer surface area.
pub fn part2(inputs: &Vec<Surface>) -> usize {
    let (x,y,z) = inputs.iter().fold((0,0,0), |(x,y,z),(i,j,k)| (x.max(*i),y.max(*j),z.max(*k)));
    let mut map = vec![vec![vec![0usize;z+3];y+3];x+3];
    let mut vis = vec![vec![vec![false;z+3];y+3];x+3];
    for cube in inputs {
        map[1+cube.0+1][1+cube.1][1+cube.2] += 1;
        map[1+cube.0-1][1+cube.1][1+cube.2] += 1;
        map[1+cube.0][1+cube.1+1][1+cube.2] += 1;
        map[1+cube.0][1+cube.1-1][1+cube.2] += 1;
        map[1+cube.0][1+cube.1][1+cube.2+1] += 1;
        map[1+cube.0][1+cube.1][1+cube.2-1] += 1;
        vis[1+cube.0][1+cube.1][1+cube.2] = true;
    }

    flood_fill((x+1,y+1,z+1), &map, &mut vis)
}

#[test]
fn test_day18_part1() {
    let inputs = vec![
        (1,1,1),
        (2,1,1)
    ];
    assert_eq!(part1(&inputs), 10);
    let inputs = vec![
        (2,2,2),
        (1,2,2),
        (3,2,2),
        (2,1,2),
        (2,3,2),
        (2,2,1),
        (2,2,3),
        (2,2,4),
        (2,2,6),
        (1,2,5),
        (3,2,5),
        (2,1,5),
        (2,3,5),
        ];
    assert_eq!(part1(&inputs), 64);
}

#[test]
fn test_day18_part2() {
    let inputs = vec![
        (2,2,2),
        (1,2,2),
        (3,2,2),
        (2,1,2),
        (2,3,2),
        (2,2,1),
        (2,2,3),
        (2,2,4),
        (2,2,6),
        (1,2,5),
        (3,2,5),
        (2,1,5),
        (2,3,5),
        ];
    assert_eq!(part2(&inputs), 58);
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


