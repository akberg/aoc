use std::collections::HashSet;

use itertools::Itertools;

/// Keywords: Lanternfish, 6-2021, 2D Map, Directions, Movement
use super::YEAR;
static DAY: usize = 15;
static LOGGING: bool = false;

type Vec2 = nalgebra_glm::TVec2<usize>;

fn vec2_index_get<T: Copy>(mat: &Vec<Vec<T>>, i: &Vec2) -> T {
    mat[i.y][i.x]
}
#[allow(unused)]
fn vec2_index_set<T: Copy>(mat: &mut Vec<Vec<T>>, i: &Vec2, obj: &T) {
    mat[i.y][i.x] = *obj;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MapTile {
    Wall,
    Object,
    Free,
    ObjLeft,
    ObjRight,
}

/// Variations on Direction enum already used multiple times, should consider
/// generalising it.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up = 0,
    Right,
    Down,
    Left,
}
impl Direction {
    #[allow(unused)]
    pub fn rot_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    #[allow(unused)]
    pub fn rot_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
    pub fn delta(&self, v: &Vec2, scale: usize) -> Vec2 {
        let mut v = v.clone();
        match self {
            Direction::Up => v.y -= scale,
            Direction::Right => v.x += scale,
            Direction::Down => v.y += scale,
            Direction::Left => v.x -= scale,
        };
        v
    }
    #[allow(unused)]
    pub fn get_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        }
    }
}
impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => panic!("Unrecognized direction "),
        }
    }
}
impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => '^',
                Direction::Right => '>',
                Direction::Down => 'v',
                Direction::Left => '<',
            }
        )
    }
}

/// Parse map specific for day 15.
fn parse_map(map: &str) -> (Vec<Vec<MapTile>>, Vec2) {
    let mut pos = Vec2::new(0, 0);
    (
        map.trim()
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.trim()
                    .char_indices()
                    .map(|(x, c)| match c {
                        '#' => MapTile::Wall,
                        'O' => MapTile::Object,
                        '@' => {
                            pos.x = x;
                            pos.y = y;
                            MapTile::Free
                        }
                        '.' => MapTile::Free,
                        _ => unreachable!("Unexpected tile symbol"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
        pos,
    )
}

fn parse_directions(directions: &str) -> Vec<Direction> {
    directions
        .lines()
        .join("")
        .chars()
        .map(Direction::from)
        .collect::<Vec<_>>()
}

fn input() -> (Vec<Vec<MapTile>>, Vec<Direction>, Vec2) {
    let inputs = crate::aoc::input_raw(YEAR, DAY);
    let (map, directions) = inputs.trim().split_once("\n\n").unwrap();
    let (map, pos) = parse_map(map);
    (map, parse_directions(directions), pos)
}

/// Run robot instructions, moving objects accordingly.
fn run_robot(map: &mut Vec<Vec<MapTile>>, directions: &Vec<Direction>, mut pos: Vec2) {
    // Debug
    if LOGGING {
        println!("Initial:");
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if pos == Vec2::new(x, y) {
                    print!("@");
                }
                else {
                    print!("{}", match map[y][x] {
                        MapTile::Wall => "#",
                        MapTile::Object => "O",
                        MapTile::Free => ".",
                        MapTile::ObjLeft => "[",
                        MapTile::ObjRight => "]",
                    });
                }
            }
            println!("");
        }
    }
    'directions: for (step, d) in directions.iter().enumerate() {
        if LOGGING {
            println!("({}) Direction: {:?}", step, d);
        }
        let mut rows = Vec::new();
        // Use a set to avoid duplicating wide objects (causes overwrites in
        // map update).
        let mut heads = HashSet::from([pos]);
        let mut i = 0;

        while heads.len() > 0 {
            i += 1;
            if i > map[0].len() {
                panic!("Detected possible loop");
            }
            let mut heads_next = HashSet::new();

            // Iterate all heads of cluster. Any walls hit will abort movement,
            // free space removes head from queue, as movement is possible.
            for head in heads.iter() {
                // print!("{:?} : {} {}", pos, d.get_char(), i);
                // println!(": {:?}", vec2_index_get(&map, &d.delta(&pos, i)));
                match vec2_index_get(&map, &d.delta(head, 1)) {
                    // Wall: Direction blocked, abort movement
                    MapTile::Wall => continue 'directions,
                    // Object: Check next tile if it can be pushed, add to next
                    // head
                    MapTile::Object => {
                        heads_next.insert(d.delta(head, 1));
                    }
                    // Open space: Robot and any objects in front will move.
                    // Do not add to queue.
                    MapTile::Free => continue,
                    // Wide object: Also add matching half
                    MapTile::ObjLeft => {
                        heads_next.insert(d.delta(head, 1));
                        if *d == Direction::Up || *d == Direction::Down {
                            heads_next.insert(d.delta(&(head + Vec2::new(1,0)), 1));
                        }
                    }
                    MapTile::ObjRight => {
                        heads_next.insert(d.delta(head, 1));
                        if *d == Direction::Up || *d == Direction::Down {
                            heads_next.insert(d.delta(&(head - Vec2::new(1,0)), 1));
                        }
                    }
                }
            }
            // Loop again, if new heads is not empty.
            heads = heads_next.clone();
            rows.push(heads_next);
            // Else, check is done, and loop exits
        }
        // If not aborted, move all collected objects one step (starting at the
        // back of the rows list)
        for row in rows.iter().rev() {
            for p in row.iter() {
                let new_p = d.delta(p, 1);
                map[new_p.y][new_p.x] = map[p.y][p.x];
                map[p.y][p.x] = MapTile::Free;
            }
        }
        let prev_pos = pos;
        pos = d.delta(&pos, 1);
        // Debug
        if LOGGING {
            for y in 0..map.len() {
                for x in 0..map[0].len() {
                    if pos == Vec2::new(x, y) {
                        print!("@");
                    }
                    else if prev_pos == Vec2::new(x, y) {
                        print!("*");
                    }
                    else {
                        print!("{}", match map[y][x] {
                            MapTile::Wall => "#",
                            MapTile::Object => "O",
                            MapTile::Free => ".",
                            MapTile::ObjLeft => "[",
                            MapTile::ObjRight => "]",
                        });
                    }
                }
                println!("");
            }
        }
    }
    // Debug
    if LOGGING {
        println!("Final:");
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if pos == Vec2::new(x, y) {
                    print!("@");
                }
                else {
                    print!("{}", match map[y][x] {
                        MapTile::Wall => "#",
                        MapTile::Object => "O",
                        MapTile::Free => ".",
                        MapTile::ObjLeft => "[",
                        MapTile::ObjRight => "]",
                    });
                }
            }
            println!("");
        }
    }
}

/// Compute sum of Goods Positioning System (100y+x for each object)
fn comp_gps(map: &Vec<Vec<MapTile>>) -> usize {
    map.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, &c)| match c {
                    MapTile::Object | MapTile::ObjLeft => y * 100 + x,
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

/// (Solved?, 1h) Move an agent around a map given a stream of directions,
/// moving loose object in front of it, and ignoring any instruction when
/// blocked by a wall.
fn part1(inputs: &(Vec<Vec<MapTile>>, Vec<Direction>, Vec2)) -> usize {
    let (mut map, directions, mut pos) = inputs.clone();
    run_robot(&mut map, &directions, pos);
    // Compute result, sum of 100y+x for each object.
    comp_gps(&map)
}

/// (Solved, 1h) Repeat part 1, but widen map, adding partial overlaps as
/// additional rules.
fn part2(inputs: &(Vec<Vec<MapTile>>, Vec<Direction>, Vec2)) -> usize {
    let (map1, directions, mut pos) = inputs.clone();
    // Translate to extended map
    let mut map = Vec::new();
    for row in map1.into_iter() {
        let mut e_row = Vec::new();
        for tile in row.into_iter() {
            match tile {
                MapTile::Free => {
                    e_row.push(MapTile::Free);
                    e_row.push(MapTile::Free);
                }
                MapTile::Wall => {
                    e_row.push(MapTile::Wall);
                    e_row.push(MapTile::Wall);
                }
                MapTile::Object => {
                    e_row.push(MapTile::ObjLeft);
                    e_row.push(MapTile::ObjRight);
                }
                _ => unreachable!("Not a part 1 tile type."),
            }
        }
        map.push(e_row);
    }
    pos.x *= 2;
    run_robot(&mut map, &directions, pos);
    comp_gps(&map)
}

#[test]
fn test_2024_day15_part1() {
    println!("test 1");
    let (map, pos) = parse_map(
        "##########
         #..O..O.O#
         #......O.#
         #.OO..O.O#
         #..O@..O.#
         #O#..O...#
         #O..O..O.#
         #.OO.O.OO#
         #....O...#
         ##########",
    );
    let directions = parse_directions(
        "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
    );
    assert_eq!(part1(&(map, directions, pos)), 10092);
    let (map, pos) = parse_map(
        "########
         #..O.O.#
         ##@.O..#
         #...O..#
         #.#.O..#
         #...O..#
         #......#
         ########",
    );
    let directions = parse_directions("<^^>>>vv<v>>v<<");
    assert_eq!(part1(&(map, directions, pos)), 2028);
}

#[test]
fn test_2024_day15_part2() {
    let (map, pos) = parse_map(
        "#######
         #...#.#
         #.....#
         #..OO@#
         #..O..#
         #.....#
         #######",
    );
    let directions = parse_directions("<vv<<^^<<^^");
    assert_eq!(part2(&(map, directions, pos)), 618);
    println!("test 1");
    let (map, pos) = parse_map(
        "##########
         #..O..O.O#
         #......O.#
         #.OO..O.O#
         #..O@..O.#
         #O#..O...#
         #O..O..O.#
         #.OO.O.OO#
         #....O...#
         ##########",
    );
    let directions = parse_directions(
        "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
    );
    assert_eq!(part2(&(map, directions, pos)), 9021);
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
