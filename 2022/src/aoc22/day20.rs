static DAY: usize = 20;

pub fn input() -> Vec<isize> {
    crate::aoc::input_raw(20)
        .lines()
        .map(|ls| ls.parse::<_>().unwrap())
        .collect()
}

/// Mixing numbers by moving them positions equal to their value, in the order
/// they first appeared.
///
/// Numbers might not be unique, adding instead a "done" flag, taking every time
/// the first unmodified number
pub fn part1(inputs: &Vec<isize>) -> isize {
    let mut outputs = inputs.into_iter()
        .map(|&e| (e,false))
        .collect::<Vec<_>>();
    for _ in 0..inputs.len() {
        let (i, &(e,_)) = outputs.iter().enumerate().find(|x|!x.1.1).unwrap();
        outputs.remove(i);

        let mut new_i = (i as isize + e) % (inputs.len()-1) as isize;
        if i as isize + e <= 0 {
            new_i += inputs.len() as isize - 1;
        }
        outputs.insert(new_i as usize, (e, true));
    }
    let i = outputs.iter().enumerate().find(|x|x.1.0==0).unwrap().0;

    outputs[(i+1000) % outputs.len()].0
    + outputs[(i+2000) % outputs.len()].0
    + outputs[(i+3000) % outputs.len()].0
}

/// Changing the "done" flag to an iteration flag, showing if the number has
/// been shuffled this round.
pub fn part2(inputs: &Vec<isize>) -> isize {
    // Adjust inputs and add index
    let mut outputs = inputs.into_iter()
        .enumerate()
        .map(|(i, e)| (*e*811589153,i))
        .collect::<Vec<_>>();
    // Mix 10 times
    for _ in 0..10 {
        for mix in 0..inputs.len() {
            let (i, &(e,mix_idx)) = outputs.iter()
                .enumerate()
                .find(|x|mix==x.1.1)
                .unwrap();
            outputs.remove(i);

            let mut new_i = (i as isize + e) % (inputs.len()-1) as isize;
            if i as isize + e <= 0 {
                new_i += inputs.len() as isize - 1;
            }
            outputs.insert(new_i as usize, (e, mix_idx));
        }

    }
    let i = outputs.iter()
        .enumerate()
        .find(|x|x.1.0==0)
        .unwrap().0;
    outputs[(i+1000) % outputs.len()].0
    + outputs[(i+2000) % outputs.len()].0
    + outputs[(i+3000) % outputs.len()].0
}

#[test]
fn test_day20_part1() {
    let inputs = vec![
        1,2,-3, 3,-2,0,4,
    ];
    assert_eq!(part1(&inputs), 3);
    let inputs = vec![
        1,2,3, 3,-2,0,4,
    ];
    assert_eq!(part1(&inputs), 9);
    let inputs = vec![
        1,10,-8, 3,-2,0,5,
    ];
    assert_eq!(part1(&inputs), 9);
    let inputs = vec![
        1,16,-8, 3,-12,0,25,
    ];
    assert_eq!(part1(&inputs), 29);
}

#[test]
fn test_day20_part2() {
    let inputs = vec![
        1,2,-3, 3,-2,0,4,
    ];
    assert_eq!(part2(&inputs), 1623178306);
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


