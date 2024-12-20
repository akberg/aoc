use image::{Rgb, RgbImage};

/// Keywords: Modulo, 2D Map, Visual Inspection, Variance, Chinese Remainder Theorem
use super::YEAR;
static DAY: usize = 14;

type Vec2 = nalgebra_glm::TVec2<isize>;

/// Parse position and
fn parse_line(line: &str) -> (Vec2, Vec2) {
    let (p, v) = line.trim().split_once(" ").unwrap();
    let p = p[2..].split_once(",").unwrap();
    let p = Vec2::new(p.0.parse::<_>().unwrap(), p.1.parse::<_>().unwrap());
    let v = v[2..].split_once(",").unwrap();
    let v = Vec2::new(v.0.parse::<_>().unwrap(), v.1.parse::<_>().unwrap());
    (p, v)
}

fn input() -> Vec<(Vec2, Vec2)> {
    crate::aoc::input_raw(YEAR, DAY)
        .lines()
        .map(parse_line)
        .collect()
}

fn glm_modulo(n: Vec2, m: Vec2) -> Vec2 {
    Vec2::new(n.x.rem_euclid(m.x), n.y.rem_euclid(m.y))
}

fn track_modulo(
    inputs: &Vec<(Vec2, Vec2)>,
    map: &mut Vec<Vec<isize>>,
    n_rounds: isize,
    width: isize,
    height: isize,
) -> [isize; 4] {
    let q = inputs
        .iter()
        .fold([0, 0, 0, 0], |mut quadrants, (pos, vel)| {
            // Modulo add for positions.
            let p = glm_modulo(pos + n_rounds * vel, Vec2::new(width, height));

            let x = p.x;
            let y = p.y;
            // Don't count resulting position on the middle lines.
            if x != width / 2 && y != height / 2 {
                // Use above/below middle as index
                quadrants[(y > height / 2) as usize * 2 + (x > width / 2) as usize] += 1;
            }
            map[y as usize][x as usize] += 1;
            quadrants
        });
    q
}
fn safety_factor(
    inputs: &Vec<(Vec2, Vec2)>,
    n_rounds: isize,
    width: isize,
    height: isize,
) -> isize {
    let mut map = vec![vec![0; width as usize]; height as usize];
    track_modulo(inputs, &mut map, n_rounds, width, height)
        .iter()
        .fold(1, |p, f| p * f)
}

/// (Solved, 1h) Modulo problem. Given a list of positions and velocities of
/// agents moving in a space with wrap-around, find the number of agents in
/// each quadrant after 100 steps.
fn part1(inputs: &Vec<(Vec2, Vec2)>) -> isize {
    // p + 100v === x (mod (103, 101))
    safety_factor(inputs, 100, 101, 103)
}

/// Finding the step which produces an image (a Christmas tree).
/// - Lowest safety factor is not it.
fn part2(inputs: &Vec<(Vec2, Vec2)>) -> isize {
    let width = 103usize;
    let height = 101usize;
    let mut mosaic = RgbImage::new((width * width) as u32, (height * height) as u32);
    let mut min_sf = isize::MAX;
    let mut min_vx = f32::MAX;
    let mut step_sf = 0;
    let mut step_vx = 0;
    // let mut mosaic = vec![0u8; width*width * height*height];
    for y in 0..height {
        for x in 0..width {
            let tstep = (y * width + x) as isize;
            let mut map = vec![vec![0; width]; height];
            let quads = track_modulo(inputs, &mut map, tstep, width as isize, height as isize);
            let avg_x: f32 = map
                .iter()
                .map(|row| row.iter().sum::<isize>() as f32)
                .sum::<f32>()
                / width as f32;
            let var_x: f32 = map
                .iter()
                .map(|row| (row.iter().sum::<isize>() as f32 - avg_x).powf(2.0))
                .sum();
            let safety_factor = quads.iter().fold(1, |p, f| p * f);
            if safety_factor < min_sf {
                min_sf = safety_factor;
                step_sf = tstep;
                println!("s {} sf={}", tstep, min_sf);
                for j in 0..map.len() {
                    for i in 0..map[0].len() {
                        print!("{}", if map[j][i] > 0 { "#" } else { " " });
                    }
                    println!("");
                }
            }
            if var_x < min_vx {
                min_vx = var_x;
                step_vx = tstep;
            }
            // println!("{} var_x = {}", y*width + x, var_x);
            // For visualization
            for j in 0..height {
                for i in 0..width {
                    mosaic.put_pixel(
                        (width * x + i) as u32,
                        (height * y + j) as u32,
                        Rgb([(map[j][i] > 0) as u8 * 255; 3]),
                    );
                    // mosaic[((y+j) * width + x+i)] = (map[j][i] > 0) as u8 * 255;
                }
            }
        }
    }
    // println!("{:?}", &mosaic[0..200]);
    // let buff = GrayImage::from_vec((width*width) as u32, (height*height) as u32, mosaic).unwrap();
    // let img = ImageReader::new(Cursor::new(bytes))
    // mosaic.save("./2024_day14.png");
    // image::save_buffer(r"./2024_day14.png", &mosaic, (width*width) as u32, (height*height) as u32, image::ColorType::L8).unwrap();
    println!("step sf = {}\nstep vx = {}", step_sf, step_vx);
    step_vx
}

#[test]
fn test_2024_day14_part1() {
    let test_inputs = vec![parse_line("p=2,4 v=2,-3")];
    assert_eq!(safety_factor(&test_inputs, 5, 11, 7), 0);
    let test_inputs = vec![
        parse_line("p=0,4 v=3,-3"),
        parse_line("p=6,3 v=-1,-3"),
        parse_line("p=10,3 v=-1,2"),
        parse_line("p=2,0 v=2,-1"),
        parse_line("p=0,0 v=1,3"),
        parse_line("p=3,0 v=-2,-2"),
        parse_line("p=7,6 v=-1,-3"),
        parse_line("p=3,0 v=-1,-2"),
        parse_line("p=9,3 v=2,3"),
        parse_line("p=7,3 v=-1,2"),
        parse_line("p=2,4 v=2,-3"),
        parse_line("p=9,5 v=-3,-3"),
    ];
    assert_eq!(safety_factor(&test_inputs, 100, 11, 7), 12);
}

#[test]
fn test_2024_day14_part2() {
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
