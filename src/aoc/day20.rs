
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
        for i in 0..self.bmp.len() { // Clockwise (lower flipped is left to right)
            ret[0] += (self.bmp[0][self.bmp[0].len()-i-1] as u16) << i;                    // upper
            ret[1] += (self.bmp[0][i] as u16) << i;                                     // upper flipped 
            ret[2] += (self.bmp[self.bmp.len()-1][self.bmp[0].len()-i-1] as u16) << i;     // lower
            ret[3] += (self.bmp[self.bmp.len()-1][i] as u16) << i;                      // lower flipped
            ret[4] += (self.bmp[self.bmp.len()-i-1][0] as u16) << i;                    // left
            ret[5] += (self.bmp[i][0] as u16) << i;                                     // left flipped
            ret[6] += (self.bmp[self.bmp.len()-i-1][self.bmp[0].len()-1] as u16) << i;     // right
            ret[7] += (self.bmp[i][self.bmp[0].len()-1] as u16) << i;                      // right flipped
        }
        ret
    }
    pub fn get_border(&self, rot: Rotation, flip: bool) -> u16 {
        use Rotation::*;
        let i = match (rot, flip) {
            (U, false) => 0,
            (U, true) => 1,
            (D, true) => 2,
            (D, false) => 3,
            (L, true) => 4,
            (L, false) => 5,
            (R, false) => 6,
            (R, true) => 7
        };
        self.all_borders()[i]
    }
    // Place tile in image with given orientation
    pub fn print(&self, image: &mut Vec<Vec<bool>>, x: usize, y: usize, rot: Rotation, flip: bool) {
        use Rotation::*;
        let x = x * 8;
        let y = y * 8;

        match (rot, flip) {
            (U, false) => {
                for i in 0..self.bmp.len()-2 {
                    for j in 0..self.bmp.len()-2 {
                        image[y+i][x+j] = self.bmp[i+1][j+1] == 1;
                    }
                }
            },
            (R, false) => { // TODO
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
            (L, false) => { // TODO
                for i in 0..self.bmp.len()-2 {
                    for j in 0..self.bmp.len()-2 {
                        image[y+self.bmp.len()-j-3][x+i] = self.bmp[i+1][j+1] == 1;
                    }
                }
            },
            (U, true) => {
                for i in 0..self.bmp.len()-2 {
                    for j in 0..self.bmp.len()-2 {
                        image[y+i][x+j] = self.bmp[i+1][self.bmp.len()-j-2] == 1;
                    }
                }
            },
            (R, true) => { // TODO
                for i in 0..self.bmp.len()-2 {
                    for j in 0..self.bmp.len()-2 {
                        image[y+j][x+i] = self.bmp[i+1][j+1] == 1;
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
                for i in 0..self.bmp.len()-2 {
                    for j in 0..self.bmp.len()-2 {
                        image[y+self.bmp.len()-j-3][x+self.bmp.len()-i-3] = self.bmp[i+1][j+1] == 1;
                    }
                }
                
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
    // for (c, n) in &graph {
    //     println!("{:010b} : {:?} ({})", c, n, c);
    // }
    let mut tile_id = get_corners(&graph)[0];
    let mut tile_top_id = tile_id;
    let mut flip = false;
    let mut flip_top = false;
    println!("Upper left corner: {}", tile_id);
    for b in &tiles.get(&tile_id).unwrap().all_borders() {
        println!("{:010b} ({})", b, b)
    }
    let upper_left = tiles.get(&tile_id).unwrap();
    // for line in &upper_left.bmp {
    //     println!("{:?}", line.iter().map(|&c| if c==1 { '#' } else { '.' }).collect::<Vec<_>>());
    // }

    let size = (tiles.len() as f32).sqrt() as usize; // Number of tiles in sides
    let mut image = vec![vec![false; size*8]; size*8];

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
            queue = upper_left.get_border(Rotation::D, false);
            flip = false;
            queue_right = upper_left.get_border(Rotation::R, false);
            flip_top = false;
        } else {
            // Rotate right 90 degrees (R + U)
            // println!("Rotate right");
            for i in 0..upper_left.bmp.len()-2 {
                for j in 0..upper_left.bmp.len()-2 {
                    image[j][upper_left.bmp.len()-i-3] = upper_left.bmp[i+1][j+1] == 1;
                }
            }
            queue = upper_left.get_border(Rotation::R, false); // right flipped
            flip = false;
            queue_right = upper_left.get_border(Rotation::U, false); // up
            flip_top = false;
        }
    } else {
        // Necessarily left neighbour
        if graph.get(&ul_borders[2]).unwrap().len() > 1 {
            // Only flipping needed (L + D)
            // println!("Flip");
            for i in 0..upper_left.bmp.len()-2 {
                for j in 0..upper_left.bmp.len()-2 {
                    image[i][j] = upper_left.bmp[i+1][upper_left.bmp.len()-j-2] == 1;
                }
            }
            queue = upper_left.get_border(Rotation::D, false); // right flipped
            flip = true;
            queue_right = upper_left.get_border(Rotation::L, false); // up
            flip_top = true;
        } else {
            // Rotate right 180 degrees (L + U)
            // println!("Rotate 180 degrees");
            for i in 0..upper_left.bmp.len()-2 {
                for j in 0..upper_left.bmp.len()-2 {
                    image[i][j] = upper_left.bmp[upper_left.bmp.len()-i-2][upper_left.bmp.len()-j-2] == 1;
                }
            }
            queue = upper_left.get_border(Rotation::U, false); // right flipped
            flip = false;
            queue_right = upper_left.get_border(Rotation::L, false); // up
            flip_top = false;
        }
    }

    println!("\n0");
    for i in 0..image.len() {
        for j in 0..image[0].len() {
            print!("{}", if image[i][j] { '#' } else { '.' });
        }
        println!("");
    }

    println!("next: {:?}", graph.get(&queue));
    // From here, place rest of image
    let mut j = 0;
    //let tile = upper_left.id;
    for n in 1..(size * size) {
        println!("queue_right = {:010b} ({})", queue_right, queue_right);
        println!("queue = {:010b} ({})", queue, queue);
        // For every next neighbour, place according to matching border,
        // padding removed. Whenever n % size == 0, go one step to the right.
        if n % size == 0 { 
            // place next tile right
            println!("New column");
            j += 1; 
            tile_id = *graph.get(&queue_right).unwrap().iter().filter(|x| **x != tile_top_id).next().unwrap();
            tile_top_id = tile_id;
            println!("Next tile: {}", tile_id);
            for b in &tiles.get(&tile_id).unwrap().all_borders() {
                println!("{:010b} ({})", b, b)
            }
            let tile = tiles.get(&tile_id).unwrap();
            for i in 0..tile.bmp.len() {
                for j in 0..tile.bmp[1].len() {
                    print!("{}", if tile.bmp[i][j]==1 { '#' } else { '.' });
                }
                println!("");
            }
            let borders = tile.all_borders();
            if (borders[0] == queue_right && flip_top) || (borders[1] == queue_right && !flip_top) {
                // Rotate left
                tile.print(&mut image, j, n % size, Rotation::L, false);
                queue = tile.get_border(Rotation::L, false);
                flip = false;
                queue_right = tile.get_border(Rotation::D, false);
                flip_top = false;
                println!("L queue(L)={} queue_right(D)={}", queue, queue_right);
            }
            else if (borders[0] == queue_right && !flip_top) || (borders[1] == queue_right && flip_top) {
                // Rotate right and flip
                tile.print(&mut image, j, n % size, Rotation::R, true);
                queue = tile.get_border(Rotation::R, false);
                flip = true;
                queue_right = tile.get_border(Rotation::D, false);
                flip_top = true;
                println!("LF queue(LF)={} queue_right(UF)={}", queue, queue_right);
            }
            else if (borders[2] == queue_right && !flip_top) || (borders[3] == queue_right && flip_top) {
                // Rotate right
                tile.print(&mut image, j, n % size, Rotation::R, false);
                queue = tile.get_border(Rotation::R, false);
                flip = false;
                queue_right = tile.get_border(Rotation::U, false);
                flip_top = false;
                println!("R queue(R)={} queue_right(U)={}", queue, queue_right);
            }
            else if (borders[3] == queue_right && !flip_top) || (borders[2] == queue_right && flip_top) {
                // Rotate left and flip
                tile.print(&mut image, j, n % size, Rotation::L, true);
                queue = tile.get_border(Rotation::L, false);
                flip = true;
                queue_right = tile.get_border(Rotation::U, false);
                flip_top = true;
                println!("RF queue(LF)={} queue_right(DF)={}", queue, queue_right);
            }
            else if (borders[4] == queue_right && !flip_top) || (borders[5] == queue_right && flip_top) {
                // No reorientation
                tile.print(&mut image, j, n % size, Rotation::U, false);
                queue = tile.get_border(Rotation::D, false);
                flip = false;
                queue_right = tile.get_border(Rotation::R, false);
                flip_top = false;
                println!("U queue(D)={} queue_right(R)={}", queue, queue_right);
            } 
            else if (borders[5] == queue_right && !flip_top) || (borders[4] == queue_right && flip_top) {
                // Flip U/D
                tile.print(&mut image, j, n % size, Rotation::D, true);
                queue = tile.get_border(Rotation::U, false);
                flip = true;
                queue_right = tile.get_border(Rotation::R, false);
                flip_top = true;
                println!("DF queue(U)={} queue_right(RF)={}", queue, queue_right);
            }
            else if (borders[6] == queue_right && !flip_top) || (borders[7] == queue_right && flip_top) {
                // Flip L/R
                tile.print(&mut image, j, n % size, Rotation::U, true);
                queue = tile.get_border(Rotation::D, false);
                flip = true;
                queue_right = tile.get_border(Rotation::L, false);
                flip_top = true;
                println!("UF queue(D)={} queue_right(L)={}", queue, queue_right);
            }
            else if (borders[7] == queue_right && !flip_top) || (borders[6] == queue_right && flip_top) {
                // Rotate 180 degrees
                tile.print(&mut image, j, n % size, Rotation::D, false);
                queue = tile.get_border(Rotation::U, false); //borders[0];
                flip = false;
                queue_right = borders[5];
                flip_top = false;
                println!("D queue(U)={} queue_right(L)={}", queue, queue_right);
            }
        } else {
            tile_id = *graph.get(&queue).unwrap().iter().filter(|x| **x != tile_id).next().unwrap();
            println!("Next tile: {}", tile_id);
            for b in &tiles.get(&tile_id).unwrap().all_borders() {
                println!("{:010b} ({})", b, b)
            }
            let tile = tiles.get(&tile_id).unwrap();
            for i in 1..tile.bmp.len() {
                for j in 1..tile.bmp[1].len() {
                    print!("{}", if tile.bmp[i][j]==1 { '#' } else { '.' });
                }
                println!("");
            }
            let borders = tile.all_borders();
            
            if (borders[0] == queue && flip) || (borders[1] == queue && !flip) {
                // No reorientation
                tile.print(&mut image, j, n % size, Rotation::U, false);
                queue = tile.get_border(Rotation::D, false);
                flip = false;
            } 
            else if (borders[0] == queue && !flip) || (borders[1] == queue && flip) {
                // Flip L/R
                tile.print(&mut image, j, n % size, Rotation::U, true);
                queue = tile.get_border(Rotation::D, false);
                flip = true;
            }
            else if (borders[2] == queue && flip) || (borders[3] == queue && !flip) {
                // Flip U/D
                tile.print(&mut image, j, n % size, Rotation::D, true);
                queue = tile.get_border(Rotation::U, false);
                flip = true;
            }
            else if (borders[2] == queue && !flip) || (borders[3] == queue && flip) {
                // Rotate 180 degrees
                tile.print(&mut image, j, n % size, Rotation::D, false);
                queue = tile.get_border(Rotation::U, false);
                flip = false;
            }
            else if (borders[4] == queue && flip) || (borders[5] == queue && !flip) {
                // Rotate right and flip
                tile.print(&mut image, j, n % size, Rotation::R, true);
                queue = tile.get_border(Rotation::R, false);
                flip = true;
            }
            else if (borders[4] == queue && !flip) || (borders[5] == queue && flip) {
                // Rotate right
                tile.print(&mut image, j, n % size, Rotation::R, false);
                queue = tile.get_border(Rotation::R, false);
                flip = false;
            }
            else if (borders[6] == queue && flip) || (borders[7] == queue && !flip) {
                // Rotate left
                tile.print(&mut image, j, n % size, Rotation::L, false);
                queue = tile.get_border(Rotation::L, false);
                flip = false;
            }
            else if (borders[6] == queue && !flip) || (borders[7] == queue && flip) {
                // Rotate left and flip
                tile.print(&mut image, j, n % size, Rotation::L, true);
                queue = tile.get_border(Rotation::L, false);
                flip = true;
            }
        }
        
        println!("\n{} ({}, {})", n, n%size, j);
        for i in 0..image.len() {
            for j in 0..image[0].len() {
                print!("{}", if image[i][j] { '#' } else { '.' });
            }
            println!("");
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
fn test_day20_print_u() {
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
        ..###..###\n");
    let tile = inputs.iter().next().unwrap().1;

    let mut image = vec![vec![false; 8]; 8];
    tile.print(&mut image, 0, 0, Rotation::U, false);

    let res = "\
        #..#....\n\
        ...##..#\n\
        ###.#...\n\
        #.##.###\n\
        #...#.##\n\
        #.#.#..#\n\
        .#....#.\n\
        ##...#.#\n\
        ".lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>()).collect::<Vec<_>>();

    println!("\nOriginal");
    for i in 1..tile.bmp.len()-1 {
        for j in 1..tile.bmp[1].len()-1 {
            print!("{}", if tile.bmp[i][j]==1 { '#' } else { '.' });
        }
        println!("");
    }
    println!("\nUp");
    for i in 0..res.len() {
        for j in 0..res[0].len() {
            print!("{}", if image[i][j] { '#' } else { '.' });
        }
        if i == 3 { print!(" <-> "); } else { print!("     "); }
        for j in 0..res[0].len() {
            print!("{}", if res[i][j] { '#' } else { '.' });
        }
        println!("");
    }
    
    assert_eq!(image, res);
}

#[test]
fn test_day20_print_uf() {
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
        ..###..###\n");
    let tile = inputs.iter().next().unwrap().1;

    let mut image = vec![vec![false; 8]; 8];
    tile.print(&mut image, 0, 0, Rotation::U, true);

    let res = "\
        ....#..#\n\
        #..##...\n\
        ...#.###\n\
        ###.##.#\n\
        ##.#...#\n\
        #..#.#.#\n\
        .#....#.\n\
        #.#...##\n\
        ".lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>()).collect::<Vec<_>>();

    println!("\nOriginal");
    for i in 1..tile.bmp.len()-1 {
        for j in 1..tile.bmp[1].len()-1 {
            print!("{}", if tile.bmp[i][j]==1 { '#' } else { '.' });
        }
        println!("");
    }
    println!("\nUp flipped");
    for i in 0..res.len() {
        for j in 0..res[0].len() {
            print!("{}", if image[i][j] { '#' } else { '.' });
        }
        if i == 3 { print!(" <-> "); } else { print!("     "); }
        for j in 0..res[0].len() {
            print!("{}", if res[i][j] { '#' } else { '.' });
        }
        println!("");
    }
    assert_eq!(image, res);
}

#[test]
fn test_day20_print_r() {
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
        ..###..###\n");
    let tile = inputs.iter().next().unwrap().1;

    let mut image = vec![vec![false; 8]; 8];
    tile.print(&mut image, 0, 0, Rotation::R, false);


    let res = "\
        #.####.#\n\
        ##...#..\n\
        ..#.##..\n\
        ....#.##\n\
        ..##.##.\n\
        #...#...\n\
        .#.##...\n\
        #.###.#.\n\
        ".lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>()).collect::<Vec<_>>();

    println!("\nOriginal");
    for i in 1..tile.bmp.len()-1 {
        for j in 1..tile.bmp[1].len()-1 {
            print!("{}", if tile.bmp[i][j]==1 { '#' } else { '.' });
        }
        println!("");
    }
    println!("\nRight");
    for i in 0..res.len() {
        for j in 0..res[0].len() {
            print!("{}", if image[i][j] { '#' } else { '.' });
        }
        if i == 3 { print!(" <-> "); } else { print!("     "); }
        for j in 0..res[0].len() {
            print!("{}", if res[i][j] { '#' } else { '.' });
        }
        println!("");
    }
    assert_eq!(image, res);
}

#[test]
fn test_day20_print_rf() {
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
        ..###..###\n");
    let tile = inputs.iter().next().unwrap().1;

    let mut image = vec![vec![false; 8]; 8];
    tile.print(&mut image, 0, 0, Rotation::R, true);

    let res = "\
        #.####.#\n\
        ..#...##\n\
        ..##.#..\n\
        ##.#....\n\
        .##.##..\n\
        ...#...#\n\
        ...##.#.\n\
        .#.###.#\n\
        ".lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>()).collect::<Vec<_>>();

    println!("\nOriginal");
    for i in 1..tile.bmp.len()-1 {
        for j in 1..tile.bmp[1].len()-1 {
            print!("{}", if tile.bmp[i][j]==1 { '#' } else { '.' });
        }
        println!("");
    }
    println!("\nRight flipped");
    for i in 0..res.len() {
        for j in 0..res[0].len() {
            print!("{}", if image[i][j] { '#' } else { '.' });
        }
        if i == 3 { print!(" <-> "); } else { print!("     "); }
        for j in 0..res[0].len() {
            print!("{}", if res[i][j] { '#' } else { '.' });
        }
        println!("");
    }
    assert_eq!(image, res);
}

#[test]
fn test_day20_print_d() {
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
        ..###..###\n");
    let tile = inputs.iter().next().unwrap().1;

    let mut image = vec![vec![false; 8]; 8];
    tile.print(&mut image, 0, 0, Rotation::D, false);


    let res = "\
        #.#...##\n\
        .#....#.\n\
        #..#.#.#\n\
        ##.#...#\n\
        ###.##.#\n\
        ...#.###\n\
        #..##...\n\
        ....#..#\n\
        ".lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>()).collect::<Vec<_>>();

    println!("\nOriginal");
    for i in 1..tile.bmp.len()-1 {
        for j in 1..tile.bmp[1].len()-1 {
            print!("{}", if tile.bmp[i][j]==1 { '#' } else { '.' });
        }
        println!("");
    }
    println!("\nDown");
    for i in 0..res.len() {
        for j in 0..res[0].len() {
            print!("{}", if image[i][j] { '#' } else { '.' });
        }
        if i == 3 { print!(" <-> "); } else { print!("     "); }
        for j in 0..res[0].len() {
            print!("{}", if res[i][j] { '#' } else { '.' });
        }
        println!("");
    }
    assert_eq!(image, res);
}

#[test]
fn test_day20_print_df() {
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
        ..###..###\n");
    let tile = inputs.iter().next().unwrap().1;

    let mut image = vec![vec![false; 8]; 8];
    tile.print(&mut image, 0, 0, Rotation::D, true);

    let res = "\
        ##...#.#\n\
        .#....#.\n\
        #.#.#..#\n\
        #...#.##\n\
        #.##.###\n\
        ###.#...\n\
        ...##..#\n\
        #..#....\n\
        ".lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>()).collect::<Vec<_>>();
    
    println!("\nOriginal");
    for i in 1..tile.bmp.len()-1 {
        for j in 1..tile.bmp[1].len()-1 {
            print!("{}", if tile.bmp[i][j]==1 { '#' } else { '.' });
        }
        println!("");
    }
    println!("\nDown flipped");
    for i in 0..res.len() {
        for j in 0..res[0].len() {
            print!("{}", if image[i][j] { '#' } else { '.' });
        }
        if i == 3 { print!(" <-> "); } else { print!("     "); }
        for j in 0..res[0].len() {
            print!("{}", if res[i][j] { '#' } else { '.' });
        }
        println!("");
    }
    assert_eq!(image, res);
}

#[test]
fn test_day20_print_l() {
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
        ..###..###\n");
    let tile = inputs.iter().next().unwrap().1;

    let mut image = vec![vec![false; 8]; 8];
    tile.print(&mut image, 0, 0, Rotation::L, false);

    let res = "\
        .#.###.#\n\
        ...##.#.\n\
        ...#...#\n\
        .##.##..\n\
        ##.#....\n\
        ..##.#..\n\
        ..#...##\n\
        #.####.#\n\
        ".lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>()).collect::<Vec<_>>();
    
    println!("\nOriginal");
    for i in 1..tile.bmp.len()-1 {
        for j in 1..tile.bmp[1].len()-1 {
            print!("{}", if tile.bmp[i][j]==1 { '#' } else { '.' });
        }
        println!("");
    }
    println!("\nLeft");
    for i in 0..res.len() {
        for j in 0..res[0].len() {
            print!("{}", if image[i][j] { '#' } else { '.' });
        }
        if i == 3 { print!(" <-> "); } else { print!("     "); }
        for j in 0..res[0].len() {
            print!("{}", if res[i][j] { '#' } else { '.' });
        }
        println!("");
    }
    assert_eq!(image, res);
}

#[test]
fn test_day20_print_lf() {
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
        ..###..###\n");
    let tile = inputs.iter().next().unwrap().1;

    let mut image = vec![vec![false; 8]; 8];
    tile.print(&mut image, 0, 0, Rotation::L, true);

    let res = "\
        #.###.#.\n\
        .#.##...\n\
        #...#...\n\
        ..##.##.\n\
        ....#.##\n\
        ..#.##..\n\
        ##...#..\n\
        #.####.#\n\
        ".lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>()).collect::<Vec<_>>();
    
    println!("\nOriginal");
    for i in 1..tile.bmp.len()-1 {
        for j in 1..tile.bmp[1].len()-1 {
            print!("{}", if tile.bmp[i][j]==1 { '#' } else { '.' });
        }
        println!("");
    }
    println!("\nLeft flipped");
    for i in 0..res.len() {
        for j in 0..res[0].len() {
            print!("{}", if image[i][j] { '#' } else { '.' });
        }
        if i == 3 { print!(" <-> "); } else { print!("     "); }
        for j in 0..res[0].len() {
            print!("{}", if res[i][j] { '#' } else { '.' });
        }
        println!("");
    }
    assert_eq!(image, res);
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
    assert_eq!(273, part2(&inputs));
}

#[test]
fn run_day20_run() {
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