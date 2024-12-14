static DAY: usize = 11;

pub fn input() -> String {
    crate::aoc::input_raw(super::YEAR, DAY)
}

fn parse_monkey(inputs: &str) -> (Vec<usize>, char, usize, usize, usize, usize) {
    let mut lines = inputs.lines();
    let _monkey_id = sscanf::scanf!(lines.next().unwrap().trim(),
        "Monkey {}:", usize).unwrap();
    let items = sscanf::scanf!(lines.next().unwrap().trim(),
        "Starting items: {}", String).unwrap()
        .split(", ")
        .map(|c|c.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let (op, opnd) = sscanf::scanf!(lines.next().unwrap().trim(),
        "Operation: new = old {} {}", char, String).unwrap();
    // 0 will mean old
    let opnd = opnd.parse::<usize>().unwrap_or(0);
    let cmp = sscanf::scanf!(lines.next().unwrap().trim(),
        "Test: divisible by {}", usize).unwrap();
    let t = sscanf::scanf!(lines.next().unwrap().trim(),
        "If true: throw to monkey {}", usize).unwrap();
    let f = sscanf::scanf!(lines.next().unwrap().trim(),
        "If false: throw to monkey {}", usize).unwrap();
    (
        items, op, opnd, cmp, t, f
    )
}

pub fn part1(inputs: &str) -> usize {
    let mut monkeys = inputs.split("\n\n").map(parse_monkey).collect::<Vec<_>>();
    let n = monkeys.len();
    let mut activity = vec![0; monkeys.len()];
    for _round in 0..20 {
        for i in 0..n {
            // Move items out
            let items = monkeys[i].0.clone();
            monkeys[i].0.clear();
            // Count inspected items
            activity[i] += items.len();
            // Inspect items
            for item in items {
                // Compute new value
                let val = match monkeys[i].1 {
                    '+' => item + if monkeys[i].2 > 0 { monkeys[i].2 } else { item },
                    '*' => item * if monkeys[i].2 > 0 { monkeys[i].2 } else { item },
                    _ => unreachable!()
                };
                let val = val / 3;
                // Determine destination monkey
                let idx = if val % monkeys[i].3 == 0 {
                    monkeys[i].4
                } else { monkeys[i].5};
                monkeys[idx].0.push(val);
            }
        }

    }
    activity.sort_by(|a,b|b.cmp(a));
    // Level of monkey business
    activity[0] * activity[1]
}

/// Numbers can now reach hilarously high values, simply applying a rather large
/// BigInt is not going to solve this. However, we have a fixed set of factors
/// used for the division tests. The numbers never really need to get larger
/// than the product of these factors. This assumption is valid because we are
/// no longer dividing the value by 3.
pub fn part2(inputs: &str) -> usize {
    let mut monkeys = inputs.split("\n\n").map(parse_monkey).collect::<Vec<_>>();
    let n = monkeys.len();
    let factor = monkeys.iter().fold(1, |acc,item| acc*item.3);
    let mut activity = vec![0; monkeys.len()];
    for _round in 0..10000 {
        for i in 0..n {
            // Move items out
            let items = monkeys[i].0.clone();
            monkeys[i].0.clear();
            // Count inspected items
            activity[i] += items.len();
            // Inspect items
            for item in items {
                // Compute new value
                let val = match monkeys[i].1 {
                    '+' => item + if monkeys[i].2 > 0 { monkeys[i].2 } else { item },
                    '*' => item * if monkeys[i].2 > 0 { monkeys[i].2 } else { item },
                    _ => unreachable!()
                } % factor;
                // Determine destination monkey
                let idx = if val % monkeys[i].3 == 0 {
                    monkeys[i].4
                } else { monkeys[i].5};
                monkeys[idx].0.push(val);
            }
        }

    }
    activity.sort_by(|a,b|b.cmp(a));
    // Level of monkey business
    activity[0] * activity[1]
}

#[test]
fn test_day11_part1() {
    let inputs = "Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
      If true: throw to monkey 2
      If false: throw to monkey 3

  Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
      If true: throw to monkey 2
      If false: throw to monkey 0

  Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
      If true: throw to monkey 1
      If false: throw to monkey 3

  Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
      If true: throw to monkey 0
      If false: throw to monkey 1";
    assert_eq!(part1(inputs), 10605)
}

#[test]
fn test_day11_part2() {
    let inputs = "Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
      If true: throw to monkey 2
      If false: throw to monkey 3

  Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
      If true: throw to monkey 2
      If false: throw to monkey 0

  Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
      If true: throw to monkey 1
      If false: throw to monkey 3

  Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
      If true: throw to monkey 0
      If false: throw to monkey 1";
    assert_eq!(part2(inputs), 2713310158)
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


