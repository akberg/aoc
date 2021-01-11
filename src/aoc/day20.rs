
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

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
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
            ret[0] += (self.bmp[0][self.bmp.len()-i-1] as u16) << i;                    // upper
            ret[1] += (self.bmp[0][i] as u16) << i;                                     // upper flipped 
            ret[2] += (self.bmp[self.bmp.len()-1][self.bmp.len()-i-1] as u16) << i;     // lower
            ret[3] += (self.bmp[self.bmp.len()-1][i] as u16) << i;                      // lower flipped
            ret[4] += (self.bmp[self.bmp.len()-i-1][0] as u16) << i;                    // left
            ret[5] += (self.bmp[i][0] as u16) << i;                                     // left flipped
            ret[6] += (self.bmp[self.bmp.len()-i-1][self.bmp.len()-1] as u16) << i;     // right
            ret[7] += (self.bmp[i][self.bmp.len()-1] as u16) << i;                      // right flipped
        }
        ret
    }
    // Place tile in image with given orientation
    pub fn print(&self, image: &mut Vec<Vec<bool>>, x: usize, y: usize, rot: Rotation, flip: bool) {
        use Rotation::*;
        match (rot, flip) {
            (U, false) => {
                for i in 0..self.bmp.len()-2 {
                    for j in 0..self.bmp.len()-2 {
                        image[y+i][x+j] = self.bmp[i+1][j+1] == 1;
                    }
                }
            },
            (R, false) => {
                for i in 0..self.bmp.len()-2 {
                    for j in 0..self.bmp.len()-2 {
                        image[y+j][x+self.bmp.len()-i-3] = self.bmp[i+1][j+1] == 1;
                    }
                }
            },
            (D, false) => {
                for i in 0..self.bmp.len()-2 {
                    for j in 0..self.bmp.len()-2 {
                        image[y+i][x+j] = self.bmp[self.bmp.len()-i-2][self.bmp.len()-j-2] == 1;
                    }
                }
            },
            (L, false) => {

            },
            (U, true) => {
                for i in 0..self.bmp.len()-2 {
                    for j in 0..self.bmp.len()-2 {
                        image[y+i][x+j] = self.bmp[i+1][self.bmp.len()-j-2] == 1;
                    }
                }
            },
            (R, true) => {
                for i in 0..self.bmp.len()-2 {
                    for j in 0..self.bmp.len()-2 {
                        image[y+j][x+self.bmp.len()-i-3] = self.bmp[i+1][self.bmp.len()-j-2] == 1;
                    }
                }
            },
            (D, true) => {
                for i in 0..self.bmp.len()-2 {
                    for j in 0..self.bmp.len()-2 {
                        image[y+i][x+j] = self.bmp[self.bmp.len()-i-2][j+1] == 1;
                    }
                }
            },
            (L, true) => {
                
            }
        }
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

fn map_neighbours(graph: &HashMap<u16, HashSet<usize>>) -> HashMap<usize, HashSet<usize>>{
    let mut neighbours = HashMap::new();
    for set in graph.values() {
        for t in set {
            let e = neighbours.entry(*t).or_insert(HashSet::new());
            set.iter().filter(|x| *x != t).for_each(|x| {e.insert(*x);});
        }
    }
    neighbours
}

// Get corners IDs 
fn get_corners(graph: &HashMap<u16, HashSet<usize>>) -> Vec<usize> {
    map_neighbours(graph).iter().filter_map(|(k,v)| if v.len()==2 { Some(*k)} else { None }).collect()
}

fn assemble_image(tiles: &HashMap<usize, Tile>) -> Vec<Vec<bool>> {
    // Arbirarily select first as upper left corner
    let graph = make_graph(tiles);
    let neighbours = map_neighbours(&graph);
    for (c, n) in &graph {
        println!("{:010b} : {:?} ({})", c, n, c);
    }
    let tile_id = get_corners(&graph)[0];
    println!("Upper left corner: {}", tile_id);
    for b in &tiles.get(&tile_id).unwrap().all_borders() {
        println!("{:010b} ({})", b, b)
    }
    let upper_left = tiles.get(&tile_id).unwrap();
    for line in &upper_left.bmp {
        println!("{:?}", line.iter().map(|&c| if c==1 { '#' } else { '.' }).collect::<Vec<_>>());
    }

    let size = (tiles.len() as f32).sqrt() as usize * 8;
    let mut image = vec![vec![false; size]; size];

    // Determine orientation of corner 
    let ul_borders = upper_left.all_borders();
    let mut queue = ul_borders[2];  // Next matching border
    let mut queue_right = ul_borders[6];
    let mut dir = Rotation::D;      // Matching direction
    if graph.get(&ul_borders[6]).unwrap().len() > 1 {
        // Has right neighbour
        if graph.get(&ul_borders[2]).unwrap().len() > 1 {
            // No reorientation needed (R + D)
            println!("No reorientation");
            for i in 0..upper_left.bmp.len()-2 {
                for j in 0..upper_left.bmp.len()-2 {
                    image[i][j] = upper_left.bmp[i+1][j+1] == 1;
                }
            }
        } else {
            // Rotate right 90 degrees (R + U)
            println!("Rotate right");
            for i in 0..upper_left.bmp.len()-2 {
                for j in 0..upper_left.bmp.len()-2 {
                    image[j][upper_left.bmp.len()-i-3] = upper_left.bmp[i+1][j+1] == 1;
                }
            }
            queue = ul_borders[7]; // right flipped
            queue_right = ul_borders[0]; // up
        }
    } else {
        // Necessarily left neighbour
        if graph.get(&ul_borders[2]).unwrap().len() > 1 {
            // Only flipping needed (L + D)
            println!("Flip");
            for i in 0..upper_left.bmp.len()-2 {
                for j in 0..upper_left.bmp.len()-2 {
                    image[i][j] = upper_left.bmp[i+1][upper_left.bmp.len()-j-2] == 1;
                }
            }
            queue = ul_borders[3]; // down flipped
            queue_right = ul_borders[4]; // left
        } else {
            // Rotate right 180 degrees (L + U)
            println!("Rotate 180 degrees");
            for i in 0..upper_left.bmp.len()-2 {
                for j in 0..upper_left.bmp.len()-2 {
                    image[i][j] = upper_left.bmp[upper_left.bmp.len()-i-2][upper_left.bmp.len()-j-2] == 1;
                }
            }
            queue = ul_borders[1]; // up flipped
            queue_right = ul_borders[5]; // left flipped
        }
    }

    println!("next: {:?}", graph.get(&queue));
    // From here, place rest of image
    let mut j = 0;
    //let tile = upper_left.id;
    for n in 1..(size * size) {
        // For every next neighbour, place according to matching border,
        // padding removed. Whenever n % size == 0, go one step to the right.
        if n % size == 0 { 
            // place next tile right
            j += 1; 
            let tile_id = graph.get(&queue_right).unwrap().iter().filter(|x| **x != tile_id).next().unwrap();
            let tile = tiles.get(&tile_id).unwrap();
            let borders = tile.all_borders();
            if borders[0] == queue {
                // Rotate left
                tile.print(&mut image, j, n % size, Rotation::L, false);
            }
            else if borders[1] == queue {
                // Rotate left and flip
                tile.print(&mut image, j, n % size, Rotation::L, true);
            }
            else if borders[2] == queue {
                // Rotate right
                tile.print(&mut image, j, n % size, Rotation::R, false);
            }
            else if borders[3] == queue {
                // Rotate right and flip
                tile.print(&mut image, j, n % size, Rotation::R, true);
            }
            else if borders[4] == queue {
                // No reorientation
                tile.print(&mut image, j, n % size, Rotation::U, false);
            } 
            else if borders[5] == queue {
                // Flip U/D
                tile.print(&mut image, j, n % size, Rotation::D, true);
            }
            else if borders[6] == queue {
                // Flip L/R
                tile.print(&mut image, j, n % size, Rotation::U, true);
            }
            else if borders[7] == queue {
                // Rotate 180 degrees
                tile.print(&mut image, j, n % size, Rotation::D, false);
            }
        } else {
            let tile_id = graph.get(&queue).unwrap().iter().filter(|x| **x != tile_id).next().unwrap();
            let tile = tiles.get(&tile_id).unwrap();
            let borders = tile.all_borders();
            
            if borders[0] == queue {
                // No reorientation
                tile.print(&mut image, j, n % size, Rotation::U, false);
            } 
            else if borders[1] == queue {
                // Flip L/R
                tile.print(&mut image, j, n % size, Rotation::U, true);
            }
            else if borders[2] == queue {
                // Flip U/D
                tile.print(&mut image, j, n % size, Rotation::D, true);
            }
            else if borders[3] == queue {
                // Rotate 180 degrees
                tile.print(&mut image, j, n % size, Rotation::D, false);
            }
            else if borders[4] == queue {
                // Rotate right and flip
                tile.print(&mut image, j, n % size, Rotation::R, true);
            }
            else if borders[5] == queue {
                // Rotate right
                tile.print(&mut image, j, n % size, Rotation::R, false);
            }
            else if borders[6] == queue {
                // Rotate left
                tile.print(&mut image, j, n % size, Rotation::L, false);
            }
            else if borders[7] == queue {
                // Rotate left and flip
                tile.print(&mut image, j, n % size, Rotation::L, true);
            }
        }
    }
    image
}

// Return image with monsters removed
fn search_monster(mut image: Vec<Vec<bool>>, rotation: usize) -> Vec<Vec<bool>> {
    let MONSTER_STR: &str = "                  # \n\
                         #    ##    ##    ###\n\
                         .#..#..#..#  #..#   "; // 20x3 (15x#)
    let monster = MONSTER_STR.lines().map(|line| line.chars()
        .map(|c| c=='#').collect::<Vec<_>>()).collect::<Vec<_>>();
    

    let mut d = 0; // 0 for not found
    for y in 0..=image.len()-monster.len() {
        for x in 0..=image[0].len()-monster[0].len() {
            // if y == 2 && x == 2 {
            //     println!("---{}---", rotation);
            //     for (i,_) in monster.iter().enumerate() {
            //         for (j,_) in monster[i].iter().enumerate() {
            //             print!("{}", if monster[i][j] { 'O' } else if image[y+i][x+j] {'#'} else {'.'});
            //         }
            //         println!("");
            //     }
            //     println!("-flipped-");
            //     for (i,_) in monster.iter().enumerate() {
            //         for (j,_) in monster[i].iter().enumerate() {
            //             print!("{}", if monster[i][monster[0].len()-1-j] { 'O' } else if image[y+i][x+monster[0].len()-1-j] {'#'} else {'.'});
            //         }
            //         println!("");
            //     }
            // }
            // Scan for monster and erase if found
            // Straight
            if (d==0 || d==1) && 
            monster[0].iter().enumerate().filter(|(_i,k)|**k).all(|(i,_)|image[y][x+i]) &&
            monster[1].iter().enumerate().filter(|(_i,k)|**k).all(|(i,_)|image[y+1][x+i]) && 
            monster[2].iter().enumerate().filter(|(_i,k)|**k).all(|(i,_)|image[y+2][x+i]) {
                d = 1;
                for (i,_) in monster.iter().enumerate() {
                    for (j,_) in monster[i].iter().enumerate().filter(|(_i,k)|**k) {
                        image[y+i][x+j] = false;
                    }
                }
            }
            // flipped
            if (d==0 || d==2) && 
            monster[0].iter().enumerate().filter(|(_i,k)|**k).all(|(i,_)|image[y][x+monster[0].len()-1-i]) &&
            monster[1].iter().enumerate().filter(|(_i,k)|**k).all(|(i,_)|image[y+1][x+monster[0].len()-1-i]) && 
            monster[2].iter().enumerate().filter(|(_i,k)|**k).all(|(i,_)|image[y+2][x+monster[0].len()-1-i]) {
                d = 2;
                for (i,_) in monster.iter().enumerate() {
                    for (j,_) in monster[i].iter().enumerate().filter(|(_i,k)|**k) {
                        image[y+i][x+monster[0].len()-1-j] = false;
                    }
                }
            }
        }
    }
    if d != 0 {
        return image
    } else {
        if rotation == 3 { panic!("Searched all rotations")}
        // Rotate right and return recursively
        let mut rot = vec![vec![false;image.len()]; image.len()];
        for i in 0..image.len() {
            for j in 0..image.len() {
                rot[j][image.len()-1-i] = image[i][j];
            }
        }
        search_monster(rot, rotation+1)
    }
}

#[allow(unused)]
pub fn part1(inputs: &HashMap<usize, Tile>) -> u64 {
    get_corners(&make_graph(inputs)).into_iter().product::<usize>() as u64
}

#[allow(unused)]
pub fn part2(inputs: &HashMap<usize, Tile>) -> u64 {
    search_monster(assemble_image(&inputs),0)
        .iter().flatten().filter(|x| **x).count() as u64
}


#[test]
fn test_day20_search_monster() {
    let map = ".#.#..#.##...#.##..#####\n\
        ###....#.#....#..#......\n\
        ##.##.###.#.#..######...\n\
        ###.#####...#.#####.#..#\n\
        ##.#....#.##.####...#.##\n\
        ...########.#....#####.#\n\
        ....#..#...##..#.#.###..\n\
        .####...#..#.....#......\n\
        #..#.##..#..###.#.##....\n\
        #.####..#.####.#.#.###..\n\
        ###.#.#...#.######.#..##\n\
        #.####....##..########.#\n\
        ##..##.#...#...#.#.#.#..\n\
        ...#..#..#.#.##..###.###\n\
        .#.#....#.##.#...###.##.\n\
        ###.#...#..#.##.######..\n\
        .#.#.###.##.##.#..#.##..\n\
        .####.###.#...###.#..#.#\n\
        ..#.#..#..#.#.#.####.###\n\
        #..####...#.#.#.###.###.\n\
        #####..#####...###....##\n\
        #.##..#..#...#..####...#\n\
        .#.###..##..##..####.##.\n\
        ...###...##...#...#..###"
    .lines()
    .map(|line| line.chars()
        .map(|c| c == '#')
        .collect::<Vec<_>>())
    .collect::<Vec<Vec<bool>>>();
    let res = search_monster(map, 0);
    for line in &res {
        println!("{:?}", line.iter().map(|&c| if c { '#' } else { '.' }).collect::<Vec<_>>());
    }
    assert_eq!(273, res.iter().flatten().filter(|x| **x).count());
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

#[test]
fn test_day20_part2() {
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
    
    let res = assemble_image(&inputs);
    for line in &res {
        println!("{:?}", line.iter().map(|&c| if c { '#' } else { '.' }).collect::<Vec<_>>());
    }
    //assert_eq!(273, part2(&inputs));
}

#[test]
fn run_day20() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . . ");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 20 part 1: ");
    println!("{} - in {:?}", part1(&inputs), pt_start.elapsed().unwrap());
    print!("Day 20 part 2: ");
    let pt_start = SystemTime::now();
    println!("{} - in {:?}", part2(&inputs), pt_start.elapsed().unwrap());
    println!("Total duration: {:?}", start.elapsed().unwrap())
}