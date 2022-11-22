static DAY: i32 = 21;

pub fn input(test: bool) -> (usize, usize) {
    let f = if test { crate::aoc::_test_input(DAY, 0) } else { crate::aoc::input(DAY) };
    (
        f[0].chars().last().unwrap().to_digit(10).unwrap() as usize,
        f[1].chars().last().unwrap().to_digit(10).unwrap() as usize,
    )
}

type Muliverse = Vec<Vec<Vec<Vec<Vec<Option<(usize,usize)>>>>>>;

fn dirac_dice(
    pos1: usize, pos2: usize, pt1: usize, pt2: usize, 
    multiverse: &mut Muliverse, d: usize
) -> (usize, usize) {
    if pt1 >= 21 {
        return (1, 0)
    } else if pt2 >= 21 {
        return (0, 1)
    }

    if let Some(x) = multiverse[d % 2][pos1-1][pos2-1][pt1][pt2] {
        return x
    }
    
    else {
        let mut score = (0,0);
        for d0 in 1..=3 {
            for d1 in 1..=3 {
                for d2 in 1..=3 {
                    let (w1, w2) = if d % 2 == 0 {
                        let pos1 = (pos1 + d0+d1+d2 - 1) % 10 + 1;
                        let pt1 = usize::min(21, pt1 + pos1);
                        dirac_dice(pos1, pos2, pt1, pt2, multiverse, d+1)
                    } else {
                        let pos2 = (pos2 + d0+d1+d2 - 1) % 10 + 1;
                        let pt2 = usize::min(21, pt2 + pos2);
                        dirac_dice(pos1, pos2, pt1, pt2, multiverse, d+1)
                    };

                    score.0 += w1; 
                    score.1 += w2;

                }
            }
        }
        multiverse[d%2][pos1-1][pos2-1][pt1][pt2] = Some(score);
        return score
    }
}

pub fn part1(inputs: &(usize, usize)) -> usize {
    let mut pos1 = inputs.0;
    let mut pos2 = inputs.1;
    let mut pt1 = 0;
    let mut pt2 = 0;
    let mut n = 0;
    for i in (0..).step_by(3) {
        let dice = (i%100)+1 + ((i+1)%100)+1 + ((i+2)%100)+1;
        n += 3;
        
        if i % 2 == 0 {
            pos1 = (pos1 + dice - 1) % 10 + 1;
            pt1 += pos1;
        } else {
            pos2 = (pos2 + dice - 1) % 10 + 1;
            pt2 += pos2;
        }
        if !(pt1 < 1000 && pt2 < 1000) { break }
    }
    n * usize::min(pt1, pt2)
}


pub fn part2(inputs: &(usize, usize)) -> usize {
    // 10 pos, 10 pos, 21 pt, 21 pt
    let mut multiverse = vec![vec![vec![vec![vec![Option::<(usize,usize)>::None;21];21];10];10]; 2];
    let (w1, w2) = dirac_dice(inputs.0, inputs.1, 0, 0, &mut multiverse, 0);

    println!("{}, {} (total of {} games", w1, w2, w1+w2);
    usize::max(w1, w2)
}

/* TESTS */

#[test]
fn test_day21_part1() {
    let inputs = input(true);
    assert_eq!(part1(&inputs), 739785);
}

#[test]
fn test_day21_part2() {
    let inputs = input(true);
    assert_eq!(part2(&inputs), 444_356_092_776_315_usize);
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
