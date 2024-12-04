use itertools::Itertools;

static DAY: usize = 05;

pub fn input() -> String {
    crate::aoc::input_raw(super::YEAR, DAY)
    //.lines()
    //.map(|ls| ls.parse::<_>().unwrap())
    //.collect()
}

fn parse_maps(inputs: &str) -> (Vec<u64>, Vec<Vec<(u64, u64, u64)>>) {
    let mut chunks = inputs.split("\n\n");
    let seeds = chunks
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let maps = chunks
        .map(|chunk| {
            chunk
                .lines()
                .skip(1)
                .map(|line| {
                    let mut nums = line.trim().split(" ").map(|x| x.parse::<u64>().unwrap());
                    (
                        nums.next().unwrap(),
                        nums.next().unwrap(),
                        nums.next().unwrap(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (seeds, maps)
}

fn map_seed(seed: u64, maps: &Vec<Vec<(u64, u64, u64)>>) -> u64 {
    let mut mapping = seed;
    // println!("seed:");
    for m in maps {
        // println!("{}", mapping);
        for line in m {
            if mapping >= line.1 && mapping <= line.1 + line.2 {
                mapping = line.0 + (mapping - line.1);
                break; // Next map
            }
        }
    }
    // println!("{}", mapping);
    mapping
}

/// (Solved)
pub fn part1(inputs: &str) -> u64 {
    let (seeds, maps) = parse_maps(inputs);
    // maps.into_iter().for_each(|c| println!("{:?}", c));

    seeds
        .into_iter()
        .map(|seed| map_seed(seed, &maps))
        .min()
        .unwrap()
}

fn map_seeds(seeds: &mut Vec<(u64, u64)>, map: &Vec<(u64, u64, u64)>) -> Vec<(u64, u64)> {
    let mut ret = Vec::new();
    'main: while let Some((start, range)) = seeds.pop() {
        for line in map {
            if start >= line.1 && start < line.1 + line.2 {
                let mstart = line.0 + (start - line.1);
                // println!("{}, {} in {}+{}, maps to {}+{}", start, range, line.1, line.2, line.0, start-line.1);

                if start + range < line.1 + line.2 {
                    ret.push((mstart, range));
                } else {
                    let mrange = line.2 - (start - line.1);
                    ret.push((mstart, mrange));
                    // println!("split off: ({}, {})", start + mrange, range - mrange);
                    seeds.push((start + mrange, range - mrange));
                }
                continue 'main;
            }
        }
        // println!("{}, {} maps to itself", start, range);
        ret.push((start, range));
    }
    ret
}

pub fn part2(inputs: &str) -> u64 {
    let (seeds, maps) = parse_maps(inputs);

    let mut seeds = seeds.into_iter().tuples().collect::<Vec<_>>();
    // // println!("{} seed elements: {:?}", seeds.len(), seeds);
    let mut mapping = map_seeds(&mut seeds, &maps[0]); // seed-soil
                                                       // // println!("{} soil elements: {:?}", mapping.len(), mapping);
    mapping = map_seeds(&mut mapping, &maps[1]); // soil-fert
                                                         // // println!("{} fertilizer elements: {:?}", mapping.len(), mapping);
    mapping = map_seeds(&mut mapping, &maps[2]); // fert-water
                                                         // // println!("{} water elements: {:?}", mapping.len(), mapping);
    mapping = map_seeds(&mut mapping, &maps[3]); // water-light
                                                         // // println!("{} light elements: {:?}", mapping.len(), mapping);
    mapping = map_seeds(&mut mapping, &maps[4]); // light-temp
                                                         // // println!("{} temperature elements: {:?}", mapping.len(), mapping);
    mapping = map_seeds(&mut mapping, &maps[5]); // temp-humid
                                                         // // println!("{} humidity elements: {:?}", mapping.len(), mapping);
    mapping = map_seeds(&mut mapping, &maps[6]); // humid-loc
                                                         // // println!("{} location elements: {:?}", mapping.len(), mapping);
    mapping.iter().map(|(loc, _range)| *loc).min().unwrap()

    // for (start, range) in seeds.iter().tuples() {
    // }
    // seeds
    //     .iter()
    //     .tuples()
    //     .flat_map(|(start, range)| (*start..(start + range)).map(|seed| map_seed(seed, &maps)))
    //     .min()
    //     .unwrap()
}
// 105230362 too high

#[test]
fn test_day5_part1() {
    let inputs = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    assert_eq!(35, part1(inputs));
}

#[test]
fn test_day5_part2() {
    let inputs = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    assert_eq!(46, part2(inputs));
}
// 105230362 too high

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
