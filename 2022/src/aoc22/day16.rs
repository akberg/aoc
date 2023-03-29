static DAY: usize = 16;

pub fn input() -> String {
    crate::aoc::input_raw(16)
        //.lines()
        //.map(|ls| ls.parse::<_>().unwrap())
        //.collect()
}

#[derive(Debug, Clone)]
struct Valve {
    name: (char, char),
    flow: usize,
    dist: Vec::<usize>,
}

/// 1 min to move, 1 min to open, pressure released is rate * remaining time
pub fn part1(inputs: &str) -> u32 {
    // Number of valves
    let problem_size = inputs.lines().count();
    let mut indices = std::collections::HashMap::new();
    // Give each valve an index
    inputs.lines().enumerate().for_each(|(i, line)| {
        indices.insert(line.split_ascii_whitespace().skip(1).next().unwrap(), i);
    });
    let mut valves = vec![Valve { name: (' ',' '), flow: 0, dist: vec![0; problem_size]}; problem_size];

    inputs.lines()
    .for_each(|line| {
        println!("{}", line.trim());
        let mut line = line.split_ascii_whitespace().skip(1);
        // Source valve
        let src = line.next().unwrap();
        line.next(); line.next();
        // Flow value
        let flow = line.next().unwrap();
        let flow = sscanf::scanf!(flow, "rate={};", usize).unwrap();
        // Destination list
        let dst = line.skip(4).map(|s| s.trim_matches(',')).collect::<Vec<_>>();

        let i = *indices.get(src).unwrap();
        valves[i].name = (src.chars().next().unwrap(), src.chars().skip(1).next().unwrap());
        valves[i].flow = flow;
        valves[i].dist[i] = 0;
        for d in dst {
            let ii = *indices.get(d).unwrap();
            valves[i].dist[ii] = 1;
            valves[ii].dist[i] = 1;
            // for j in 0..problem_size {
            //     valves[i].dist[j] = valves[i].dist[j].min(valves[i].dist[ii] + valves[ii].dist[j]);
            //     valves[ii].dist[j] = valves[ii].dist[j].min(valves[i].dist[ii] + valves[i].dist[j]);
            // }
        }
    });
    println!("{:?}", valves);
    for r in 0..problem_size {
        for v in 0..problem_size {
            for i in 0..problem_size {
                if valves[v].dist[i] == 0 {
                    continue;
                }
                for ii in 0..problem_size {
                    if valves[v].dist[ii] == 0 {
                        continue;
                    }
                    if valves[i].dist[ii] == 0 || valves[ii].dist[i] == 0 || valves[i].dist[ii] > valves[v].dist[i] + valves[v].dist[ii] {
                        valves[i].dist[ii] = valves[v].dist[i] + valves[v].dist[ii];
                        valves[ii].dist[i] = valves[v].dist[i] + valves[v].dist[ii];
                    }
                }
            }
        }
    }

    println!("{:?}", valves);
    let mut time = 30;
    0
}

pub fn part2(inputs: &str) -> u32 {
    0
}

#[test]
fn test_day16_part1() {
    let inputs = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    Valve BB has flow rate=13; tunnels lead to valves CC, AA
    Valve CC has flow rate=2; tunnels lead to valves DD, BB
    Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    Valve EE has flow rate=3; tunnels lead to valves FF, DD
    Valve FF has flow rate=0; tunnels lead to valves EE, GG
    Valve GG has flow rate=0; tunnels lead to valves FF, HH
    Valve HH has flow rate=22; tunnel leads to valve GG
    Valve II has flow rate=0; tunnels lead to valves AA, JJ
    Valve JJ has flow rate=21; tunnel leads to valve II";
    assert_eq!(part1(inputs), 1651);
}

#[test]
fn test_day16_part2() {
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


