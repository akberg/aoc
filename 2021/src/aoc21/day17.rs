static DAY: i32 = 17;

pub fn input() -> ((isize, isize), (isize, isize)) {
    let (rx, ry) = ((34,67), (-215,-186));
    (rx, ry)
}

// x paths = vx0(vx0 + 1)/2
// 0 1 1
// 0 2 3 3
// 0 3 5 6 6
// 0 4 7 

/// Find the trajectory with the highest max y that hits the target
/// Bruteforced...
pub fn part1(inputs: &((isize, isize), (isize, isize))) -> isize {
    fn sum_to(n: isize) -> isize { n * (n + 1) / 2 }

    let t_hit = (0..).skip_while(|&n| sum_to(n) < inputs.0.0).take_while(|&n| sum_to(n) < inputs.0.1);

    let mut ymax = 0;
    for _ in t_hit {
        // Find the highest y that is inside target after time t
        // t is time and initial x velocity
        for y in 0..3000 {
            let top = sum_to(y);
            if let Some(_) = (0..)
            .skip_while(|&n| top - sum_to(n) > inputs.1.1)
            .take_while(|&n| top - sum_to(n) >= inputs.1.0).next() {
                if top > ymax {
                    ymax = top;
                }
            }
        }
    }
    ymax
}

/// TODO
pub fn part2(inputs: &((isize, isize), (isize, isize))) -> isize {
    //fn sum_to(n: isize) -> isize { n * (n + 1) / 2 }
    fn sum_from_to(n0: isize, n1 :isize) -> isize { (isize::pow(n1,2) + n1 - isize::pow(n0,2) - n0) / 2 }

    let mut count = 0;

    for x1 in 0..=inputs.0.1+100 {
        for y1 in inputs.1.0-100..50000 {
            let mut posy = 0;
            let mut vely = y1;
            let mut posx = 0;
            for (t, x0) in (-50..=x1).rev().enumerate() {
                if x0 >= 0 { posx = sum_from_to(x0, x1); }
                if posx > inputs.0.1 {
                    break
                }
                else if posx >= inputs.0.0 {
                    if inputs.1.0 <= posy && posy <= inputs.1.1 {
                        eprintln!("x<{}>({}) = {}, y<{}>({}) = {}", x1, t, posx, y1, t, posy);
                        count += 1;
                        break;
                    }
                }
                posy += vely;
                vely -= 1;
            }
        }
    }
    count
}

/* TESTS */
#[allow(unused)]
static TEST_INPUT: &((isize, isize), (isize, isize)) = &((20,30), (-10,-5));

#[test]
fn test_day17_part1() {
    assert_eq!(part1(TEST_INPUT), 45);
}

#[test]
fn test_day17_part2() {
    assert_eq!(part2(TEST_INPUT), 112);
}

#[allow(unused)]
pub fn run() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . .");
    let inputs = input();
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
