static DAY: usize = 19;

#[derive(Debug, Copy, Clone)]
pub struct Blueprint {
    id: usize,
    ore: usize,                 // ore
    clay: usize,                // ore
    obsidian: (usize, usize),   // ore, clay
    geode: (usize, usize),      // ore, obsidian
}
#[derive(Debug, Default, Copy, Clone)]
struct State {
    time: usize,
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,

    ore_robot: usize,
    clay_robot: usize,
    obsidian_robot: usize,
    geode_robot: usize
}
impl State {
    pub fn new() -> Self { Self { ore_robot: 1, ..Default::default() }}
}

pub fn input() -> Vec<Blueprint> {
    crate::aoc::input_raw(super::YEAR, DAY)
        .lines()
        .map(|line| {
            let line = line.split_ascii_whitespace().collect::<Vec<_>>();
            Blueprint {
                id: sscanf::scanf!(line[1], "{}:", usize).unwrap(),
                ore: sscanf::scanf!(line[6], "{}", usize).unwrap(),
                clay: sscanf::scanf!(line[12], "{}", usize).unwrap(),
                obsidian: (
                    sscanf::scanf!(line[18], "{}", usize).unwrap(),
                    sscanf::scanf!(line[21], "{}", usize).unwrap(),
                ),
                geode: (
                    sscanf::scanf!(line[27], "{}", usize).unwrap(),
                    sscanf::scanf!(line[30], "{}", usize).unwrap(),
                ),
            }
        })
        .collect()
}
impl Blueprint {
    fn max_out(&self) -> usize {
        // Ratio of materials for composite robots
        // let obsidian_ratio = (self.obsidian.1 / self.obsidian.0);
        // let geode_ratio = (self.geode.1 / self.geode.0) as usize;
        let ore_max = self.ore.max(self.clay).max(self.obsidian.0).max(self.geode.0);

        let mut states = std::collections::VecDeque::from([State::new()]);
        let mut max_yield = 0;

        while let Some(state) = states.pop_front() {
            if state.time == 24 {
                max_yield = max_yield.max(state.geode);
                continue;
            }

            // Save up and build a geode robot
            if state.obsidian_robot > 0 {
                let rem_obs = self.geode.1.checked_sub(state.obsidian).unwrap_or(0);
                let rem_ore = self.geode.0.checked_sub(state.ore).unwrap_or(0);
                let rem_rounds = 1 + (rem_obs as f32 / state.obsidian_robot as f32).max(rem_ore as f32 / state.ore_robot as f32).ceil() as usize;
                let mut new_state = state;
                new_state.ore += rem_rounds * state.ore_robot;
                new_state.ore -= self.geode.0;
                new_state.clay += rem_rounds * state.clay_robot;
                new_state.obsidian += rem_rounds * state.obsidian_robot;
                new_state.obsidian -= self.geode.1;
                new_state.geode += rem_rounds * state.geode_robot;
                new_state.time += rem_rounds;
                new_state.geode_robot += 1;

                if new_state.time <= 24 {
                    println!("{:?}", new_state);
                    states.push_back(new_state);
                }
            }
            // Save up and build an obsidian robot
            if state.clay_robot > 0 {
                let rem_cly = self.obsidian.1.checked_sub(state.clay).unwrap_or(0);
                let rem_ore = self.obsidian.0.checked_sub(state.ore).unwrap_or(0);
                let rem_rounds = 1 + (rem_cly as f32 / state.clay_robot as f32).max(rem_ore as f32 / state.ore_robot as f32).ceil() as usize;
                let mut new_state = state;
                new_state.ore += rem_rounds * state.ore_robot;
                new_state.ore -= self.obsidian.0;
                new_state.clay += rem_rounds * state.clay_robot;
                new_state.clay -= self.obsidian.1;
                new_state.obsidian += rem_rounds * state.obsidian_robot;
                new_state.geode += rem_rounds * state.geode_robot;
                new_state.time += rem_rounds;
                new_state.obsidian_robot += 1;

                if new_state.time <= 24 {
                    states.push_back(new_state);
                }
            }
            // Save up and build a clay robot
            if state.clay_robot < self.obsidian.1 {
                let rem_ore = self.clay.checked_sub(state.ore).unwrap_or(0);
                let rem_rounds = 1 + (rem_ore as f32 / state.ore_robot as f32).ceil() as usize;
                let mut new_state = state;
                new_state.ore += rem_rounds * state.ore_robot;
                new_state.ore -= self.clay;
                new_state.clay += rem_rounds * state.clay_robot;
                new_state.obsidian += rem_rounds * state.obsidian_robot;
                new_state.geode += rem_rounds * state.geode_robot;
                new_state.time += rem_rounds;
                new_state.clay_robot += 1;

                if new_state.time <= 24 {
                    states.push_back(new_state);
                }

            }
            // Save up and build an ore robot
            if state.clay_robot < ore_max {
                let rem_ore = self.ore.checked_sub(state.ore).unwrap_or(0);
                let rem_rounds = 1 + (rem_ore as f32 / state.ore_robot as f32).ceil() as usize;
                let mut new_state = state;
                new_state.ore += rem_rounds * state.ore_robot;
                new_state.ore -= self.ore;
                new_state.clay += rem_rounds * state.clay_robot;
                new_state.obsidian += rem_rounds * state.obsidian_robot;
                new_state.geode += rem_rounds * state.geode_robot;
                new_state.time += rem_rounds;
                new_state.clay_robot += 1;

                if new_state.time <= 24 {
                    states.push_back(new_state);
                }
            }
            let rem_rounds = 24 - state.time;
            max_yield = max_yield.max(state.geode + rem_rounds * state.geode_robot);
        }
        println!("yields {}", max_yield);
        max_yield
    }
}


pub fn part1(inputs: &Vec<Blueprint>) -> usize {
    inputs.into_iter().map(|bp| bp.id * bp.max_out()).sum()
}

pub fn part2(inputs: &Vec<Blueprint>) -> usize {
    0
}

#[test]
fn test_day19_part1() {
    let inputs = vec![
        Blueprint {
            id: 1,
            ore: 4,
            clay: 2,
            obsidian: (3, 14),
            geode: (2, 7)
        },
        Blueprint {
            id: 2,
            ore: 2,
            clay: 3,
            obsidian: (3, 8),
            geode: (3, 12)
        },
    ];
    assert_eq!(part1(&inputs), 33);
}

#[test]
fn test_day19_part2() {
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


