
use std::collections::{HashMap, HashSet};

#[allow(unused)]
pub fn input() -> HashMap<usize, Tile> {
    parse_tiles(&crate::aoc::input_raw(20))
}

fn parse_tiles(tiles: &str) -> HashMap<usize, Tile> {
    tiles
    .split("\n\n")
    .map(|s| {
        let v: Vec<&str> = s.splitn(2, "\n").collect();
        (v[0][5..9].parse().unwrap(), Tile::new(v[0][5..9].parse().unwrap(), v[1]))
    }
    ).collect()
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Rotation { U, D, L, R }
#[derive(Debug, PartialEq)]
pub struct Tile {
    id: usize,
    flip: bool,
    rotation: Rotation,
    bmp: Vec<Vec<u8>>
}
impl Tile {
    pub fn new(id: usize, pattern: &str) -> Self {
        Self {
            id: id,
            flip: false,
            rotation: Rotation::U,
            bmp: pattern.lines()
                .map(|s| s
                    .chars()
                    .map(|c| match c=='#' { false => 0, true => 1 })
                    .collect::<Vec<u8>>()
                )
                .collect()
        }
    }
    // Get binary representation of all possible borders
    pub fn all_borders(&self) -> [u16; 8] {
        let mut ret = [0;8];
        
        for i in 0..self.bmp.len() {
            ret[0] += (self.bmp[0][i] as u16) << i;
            ret[1] += (self.bmp[0][self.bmp.len()-i-1] as u16) << i;
            ret[2] += (self.bmp[self.bmp.len()-1][i] as u16) << i;
            ret[3] += (self.bmp[self.bmp.len()-1][self.bmp.len()-i-1] as u16) << i;
            ret[4] += (self.bmp[i][0] as u16) << i;
            ret[5] += (self.bmp[self.bmp.len()-i-1][0] as u16) << i;
            ret[6] += (self.bmp[i][self.bmp.len()-1] as u16) << i;
            ret[7] += (self.bmp[self.bmp.len()-i-1][self.bmp.len()-1] as u16) << i;
        }
        ret
    }
}

fn make_graph(tiles: &HashMap<usize, Tile>) -> HashMap<u16, HashSet<usize>> {
    let mut graph: HashMap<u16, HashSet<usize>> = HashMap::new();
 
    for tile in tiles.values() {
        for b in &tile.all_borders() {
            let e = graph.entry(*b).or_insert(HashSet::new());
            e.insert(tile.id);
        }
    }
    graph
}

#[allow(unused)]
pub fn part1(inputs: &HashMap<usize, Tile>) -> u64 {
    0
}

#[test]
fn test_day20_parse_tiles() {
    let inputs = parse_tiles("Tile 2311:\n\
        ..##.#..#.\n\
        ##..#.....\n\
        #...##..#.\n\
        ####.#...#\n\
        ##.##.###.\n\
        ##...#.###\n\
        .#.#.#..##\n\
        ..#....#..\n\
        ###...#.#.\n\
        ..###..###\n\
        \n\
        Tile 1951:\n\
        #.##...##.\n\
        #.####...#\n\
        .....#..##\n\
        #...######\n\
        .##.#....#\n\
        .###.#####\n\
        ###.##.##.\n\
        .###....#.\n\
        ..#.#..#.#\n\
        #...##.#..");
    assert_eq!(inputs.values().next().unwrap(), Tile {
        id: 2311,
        flip: false,
        rotation: Rotation::U,
        bmp: vec![
            vec![0,0,1,1,0,1,0,0,1,0],
            vec![1,1,0,0,1,0,0,0,0,0],
            vec![1,0,0,0,1,1,0,0,1,0],
            vec![1,1,1,1,0,1,0,0,0,1],
            vec![1,1,0,1,1,0,1,1,1,0],
            vec![1,1,0,0,0,1,0,1,1,1],
            vec![0,1,0,1,0,1,0,0,1,1],
            vec![0,0,1,0,0,0,0,1,0,0],
            vec![1,1,1,0,0,0,1,0,1,0],
            vec![0,0,1,1,1,0,0,1,1,1]
        ]
    });
    inputs.values().next().unwrap().all_borders().iter().for_each(|x| println!("{:010b}", x));
    [
        0b0011010010,0b0100101100,

        0b1100100000,0b0000010011,

        0b1000110010,0b0100110001,

        0b1111010001,0b1000101111,

        0b1101101110,0b0111011011
    ].iter()
    .for_each(|x| 
        assert!(inputs.values().next().unwrap().all_borders().contains(x))
    );
}


#[test]
fn test_day20_part1() {
    let inputs = parse_tiles("Tile 2311:\n\
        ..##.#..#.\n\
        ##..#.....\n\
        #...##..#.\n\
        ####.#...#\n\
        ##.##.###.\n\
        ##...#.###\n\
        .#.#.#..##\n\
        ..#....#..\n\
        ###...#.#.\n\
        ..###..###\n\
        \n\
        Tile 1951:\n\
        #.##...##.\n\
        #.####...#\n\
        .....#..##\n\
        #...######\n\
        .##.#....#\n\
        .###.#####\n\
        ###.##.##.\n\
        .###....#.\n\
        ..#.#..#.#\n\
        #...##.#..\n\
        \n\
        Tile 1171:\n\
        ####...##.\n\
        #..##.#..#\n\
        ##.#..#.#.\n\
        .###.####.\n\
        ..###.####\n\
        .##....##.\n\
        .#...####.\n\
        #.##.####.\n\
        ####..#...\n\
        .....##...\n\
        \n\
        Tile 1427:\n\
        ###.##.#..\n\
        .#..#.##..\n\
        .#.##.#..#\n\
        #.#.#.##.#\n\
        ....#...##\n\
        ...##..##.\n\
        ...#.#####\n\
        .#.####.#.\n\
        ..#..###.#\n\
        ..##.#..#.\n\
        \n\
        Tile 1489:\n\
        ##.#.#....\n\
        ..##...#..\n\
        .##..##...\n\
        ..#...#...\n\
        #####...#.\n\
        #..#.#.#.#\n\
        ...#.#.#..\n\
        ##.#...##.\n\
        ..##.##.##\n\
        ###.##.#..\n\
        \n\
        Tile 2473:\n\
        #....####.\n\
        #..#.##...\n\
        #.##..#...\n\
        ######.#.#\n\
        .#...#.#.#\n\
        .#########\n\
        .###.#..#.\n\
        ########.#\n\
        ##...##.#.\n\
        ..###.#.#.\n\
        \n\
        Tile 2971:\n\
        ..#.#....#\n\
        #...###...\n\
        #.#.###...\n\
        ##.##..#..\n\
        .#####..##\n\
        .#..####.#\n\
        #..#.#..#.\n\
        ..####.###\n\
        ..#.#.###.\n\
        ...#.#.#.#\n\
        \n\
        Tile 2729:\n\
        ...#.#.#.#\n\
        ####.#....\n\
        ..#.#.....\n\
        ....#..#.#\n\
        .##..##.#.\n\
        .#.####...\n\
        ####.#.#..\n\
        ##.####...\n\
        ##..#.##..\n\
        #.##...##.\n\
        \n\
        Tile 3079:\n\
        #.#.#####.\n\
        .#..######\n\
        ..#.......\n\
        ######....\n\
        ####.#..#.\n\
        .#...#.##.\n\
        #.#####.##\n\
        ..#.###...\n\
        ..#.......\n\
        ..#.###...");
    

    assert_eq!(20899048083289, part1(&inputs));
}