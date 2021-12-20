static DAY: i32 = 20;

pub fn input(test: bool) -> ([u8;512], Vec<Vec<u16>>) {
    let f = if test { crate::aoc::_test_input_raw(DAY, 0) } else { crate::aoc::input_raw(DAY) };
    let mut f = f.lines();
    let mut alg = [0; 512];
    f.next().unwrap().chars().enumerate().for_each(|(i, c)| alg[i] = if c == '#' { 1 } else { 0 });
    f.next();
    let temp = f.enumerate()
    .map(
        |(m, line)| line
        .chars()
        .enumerate()
        .map(|(n, c)| if c == '#' { 1 } else { 0 })
        .collect::<Vec<_>>()
    )
    .collect::<Vec<Vec<_>>>();
    let mut img = vec![vec![0;temp[0].len()+2];temp.len()+2];
    temp.iter().enumerate()
    .for_each(
        |(m,row)| row.iter().enumerate()
        .for_each(|(n,x)| img[m+1][n+1] = *x)
    );
    (alg, img)
}

fn enhance(alg: &[u8;512], img: &Vec<Vec<u16>>) -> Vec<Vec<u16>> {
    let mut temp = vec![vec![img[0][0];img[0].len()+4];img.len()+4];
    img.iter().enumerate()
    .for_each(
        |(m,row)| row.iter().enumerate()
        .for_each(|(n,x)| temp[m+2][n+2] = *x)
    );
    let mut out = vec![vec![0;temp[1].len()-2];temp.len()-2];

    for m in 1..temp.len()-1 {
        for n in 1..temp[1].len()-1 {
            let idx =
                (temp[m-1][n-1] << 8) +
                (temp[m-1][n  ] << 7) +
                (temp[m-1][n+1] << 6) +
                (temp[m  ][n-1] << 5) +
                (temp[m  ][n  ] << 4) +
                (temp[m  ][n+1] << 3) +
                (temp[m+1][n-1] << 2) +
                (temp[m+1][n  ] << 1) +
                (temp[m+1][n+1] << 0);
            assert!(idx < 512);
            out[m-1][n-1] = alg[idx as usize] as u16;
        }
    }
    out
}

#[allow(unused)]
fn print_img(img: &Vec<Vec<u16>>) {
    for i in 0..img.len(){
        for j in 0..img[0].len() {
            if img[i][j] == 1 { print!("#"); } else { print!("."); }
        }
        println!("");
    }
}

pub fn part1(inputs: &([u8;512], Vec<Vec<u16>>)) -> u64 {
    let img = inputs.1.clone();
    let img = enhance(&inputs.0, &img);
    let img = enhance(&inputs.0, &img);
    img.iter()
    .map(
        |row| row.iter().map(|&x| x as u64).sum::<u64>()
    )
    .sum::<u64>()
}


pub fn part2(inputs: &([u8;512], Vec<Vec<u16>>)) -> u64 {
    let mut img = inputs.1.clone();
    for i in 0..50 {
        img = enhance(&inputs.0, &img);
    }
    img.iter()
    .map(
        |row| row.iter().map(|&x| x as u64).sum::<u64>()
    )
    .sum::<u64>()
}

/* TESTS */

#[test]
fn test_day20_part1() {
    let inputs = input(true);
    assert_eq!(part1(&inputs), 35);
}

#[test]
fn test_day20_part2() {
    let inputs = input(true);
    assert_eq!(part2(&inputs), 3351);
}


#[allow(unused)]
pub fn run() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . .");
    let inputs = input(false);
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day {} part 1: ", DAY);
    println!("{}", part1(&inputs));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day {} part 2: ", DAY);
    println!("{}", part2(&inputs));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    println!("Total time: {:?}", start.elapsed().unwrap());
}
