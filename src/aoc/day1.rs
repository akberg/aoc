
use std::fs;
#[allow(unused)]
pub fn input() -> Vec<i32> {
    let filename = format!("inputs/day1.txt");
    fs::read_to_string(filename)
        .iter()
        .map(|x| x.parse::<i32>()
        .unwrap() )
        .collect()
}

#[allow(unused)]
pub fn part1(inputs: &Vec<i32>) -> Result<i32, &'static str> {
    for i in 0..inputs.len() {
        for j in i+1..inputs.len() {
            if inputs[i] + inputs[j] == 2020 {
                println!("{} + {} = 2020", inputs[i], inputs[j]);
                return Ok(inputs[i]*inputs[j])
            }
        }
    }
    return Err("No result found")
}

#[allow(unused)]
pub fn part2(inputs: &Vec<i32>) -> Result<i32, &'static str> {
    println!("{:?}", inputs);
    for i in 0..inputs.len()-2 {
        for j in i+1..inputs.len()-1 {
            for k in j+1..inputs.len() {
                println!("{}", inputs[i] + inputs[j] + inputs[k]);
                if inputs[i] + inputs[j] + inputs[k] == 2020 {
                    return Ok(inputs[i]*inputs[j]*inputs[k])
                }
            }
        }
    }
    return Err("No result found")
}

#[test]
fn test_d1p1() {
    let content = vec![1721, 979, 366, 299, 675, 1456];
    let res = part1(&content);
    assert_eq!(res, Ok(514579))
}

#[test]
fn test_d1p2() {
    let content = vec![1721, 979, 366, 299, 675, 1456];
    let res = part2(&content);
    assert_eq!(res, Ok(241861950))
}
