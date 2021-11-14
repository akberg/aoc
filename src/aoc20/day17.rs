use std::collections::{HashMap, HashSet};



#[allow(unused)]
pub fn input() -> (HashSet<(i32, i32, i32)>, HashSet<(i32, i32, i32, i32)>) {
    let inputs = crate::aoc::input_raw(20, 17);
    (
        parse_map(&inputs), 
        parse_map_4d(&inputs)
    )
}

#[allow(unused)]
pub fn parse_map(inputs: &str) -> HashSet<(i32, i32, i32)> {
    inputs.lines()
    .enumerate()
    .flat_map(|(i, line)|
        line.chars().enumerate()
        .filter_map(move |(j, c)| match c { '#' => Some((i as i32, j as i32, 0)), _=> None})
    )
    .collect::<HashSet<_>>()
}

#[allow(unused)]
pub fn parse_map_4d(inputs: &str) -> HashSet<(i32, i32, i32, i32)> {
    inputs.lines()
    .enumerate()
    .flat_map(|(i, line)|
        line.chars().enumerate()
        .filter_map(move |(j, c)| match c { '#' => Some((i as i32, j as i32, 0, 0)), _=> None})
    )
    .collect::<HashSet<_>>()
}

fn conway_cube_iteration(state: HashSet<(i32, i32, i32)>) -> HashSet<(i32, i32, i32)> {
    let mut mapping = HashMap::new();

    // map number of neighbors
    for point in state.iter() {
        let (i, j, k) = point;
        for x in i-1..=i+1 {
            for y in j-1..=j+1 {
                for z in k-1..=k+1 {
                    if x!=*i || y!=*j || z!=*k {
                        let entry = mapping.entry((x, y, z)).or_insert(0);
                        *entry += 1;
                    }
                }
                
            }
        }
    }
    let mut new_state = HashSet::new();
    for (point, n) in mapping.drain() {
        // Next state according to rules
        if (state.contains(&point) && (2..=3).contains(&n)) 
        || (!state.contains(&point) && n == 3) {
            new_state.insert(point);
        }
    }
    new_state
}

fn conway_hypercube_iteration(state: HashSet<(i32, i32, i32, i32)>) -> HashSet<(i32, i32, i32, i32)> {
    let mut mapping = HashMap::new();

    // map number of neighbors
    for point in state.iter() {
        let (i, j, k, l) = point;
        for x in i-1..=i+1 {
            for y in j-1..=j+1 {
                for z in k-1..=k+1 {
                    for w in l-1..=l+1 {
                        if x!=*i || y!=*j || z!=*k || w!=*l {
                            let entry = mapping.entry((x, y, z, w)).or_insert(0);
                            *entry += 1;
                        }
                    }
                }
                
            }
        }
    }
    let mut new_state = HashSet::new();
    for (point, n) in mapping.drain() {
        // Next state according to rules
        if (state.contains(&point) && (2..=3).contains(&n)) 
        || (!state.contains(&point) && n == 3) {
            new_state.insert(point);
        }
    }
    new_state
}

#[allow(unused)]
pub fn part1(inputs: &HashSet<(i32, i32, i32)>) -> usize {
    let mut state = inputs.clone();
    for i in 0..6 {
        state = conway_cube_iteration(state);
    }
    state.len()
}

#[allow(unused)]
pub fn part2(inputs: &HashSet<(i32, i32, i32, i32)>) -> usize {
    let mut state = inputs.clone();
    for i in 0..6 {
        state = conway_hypercube_iteration(state);
    }
    state.len()
}

#[test]
fn test_part1() {
    let map = parse_map(".#.\n..#\n###");
    assert_eq!(112, part1(&map));
}

#[test]
fn test_part2() {
    let map = parse_map_4d(".#.\n..#\n###");
    assert_eq!(848, part2(&map));
}


#[test]
fn run_day17() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . . ");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 16 part 1: ");
    println!("{} - in {:?}", part1(&inputs.0), pt_start.elapsed().unwrap());
    print!("Day 15 part 2: ");
    let pt_start = SystemTime::now();
    println!("{} - in {:?}", part2(&inputs.1), pt_start.elapsed().unwrap());
    println!("Total duration: {:?}", start.elapsed().unwrap())
}