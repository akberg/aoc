use itertools::Itertools;

/// Keywords: Lanternfish, 6-2021, 2D Map, Directions, Movement
use super::YEAR;
static DAY: usize = 15;

type Vec2 = nalgebra_glm::TVec2<usize>;

fn vec2_index_get<T: Copy>(mat: &Vec<Vec<T>>, i: &Vec2) -> T {
    mat[i.y][i.x]
}
fn vec2_indexset<T: Copy>(mat: &mut Vec<Vec<T>>, i: &Vec2, obj: &T) {
    mat[i.y][i.x] = *obj;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MapTile {
    Wall,
    Object,
    Free,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up = 0,
    Right,
    Down,
    Left,
}
impl Direction {
    pub fn rot_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
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

/// (Solved?, 1h) Move an agent around a map given a stream of directions,
/// moving loose object in front of it, and ignoring any instruction when
/// blocked by a wall.
fn part1(inputs: &(Vec<Vec<MapTile>>, Vec<Direction>, Vec2)) -> usize {
    let (mut map, directions, mut pos) = inputs.clone();

    for d in directions {
        for i in 1.. {
            // print!("{:?} : {} {}", pos, d.get_char(), i);
            // println!(": {:?}", vec2_index_get(&map, &d.delta(&pos, i)));
            match vec2_index_get(&map, &d.delta(&pos, i)) {
                // Wall: Direction blocked
                MapTile::Wall => break,
                // Object: Check next tile if it can be pushed
                MapTile::Object => continue,
                // Open space: Robot and any objects in front will move
                MapTile::Free => {
                    if i > 1 {
                        vec2_indexset(&mut map, &d.delta(&pos, i), &MapTile::Object);
                    }
                    vec2_indexset(&mut map, &d.delta(&pos, 1), &MapTile::Free);
                    pos = d.delta(&pos, 1);
                    break;
                }
            }
        }
    }

    // Compute result, sum of 100y+x for each object.
    map.into_iter()
        .enumerate()
        .map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .map(|(x, c)| if c == MapTile::Object { y * 100 + x } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn part2(inputs: &(Vec<Vec<MapTile>>, Vec<Direction>, Vec2)) -> u32 {
    let (map, directions, pos) = inputs;
    0
}

// ##########
// #..O..O.O#
// #......O.#
// #.OO..O.O#
// #..O@..O.#
// #O#..O...#
// #O..O..O.#
// #.OO.O.OO#
// #....O...#
// ##########
//
// <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
// vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
// ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
// <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
// ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
// ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
// >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
// <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
// ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
// v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
//
// ########
// #..O.O.#
// ##@.O..#
// #...O..#
// #.#.O..#
// #...O..#
// #......#
// ########
//
// <^^>>>vv<v>>v<<
//
#[test]
fn test_2024_day15_part1() {
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
