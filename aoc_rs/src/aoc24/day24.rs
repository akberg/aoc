use std::collections::{HashMap, VecDeque};
use std::io::Write;

use itertools::Itertools;

use super::YEAR;
static DAY: usize = 24;

fn input() -> String {
    crate::aoc::input_raw(YEAR, DAY)
}

fn draw(input: HashMap<&str, usize>, gates: VecDeque<(&str, &str, &str, &str)>) -> Result<(), std::io::Error> {
    let width = input.len() / 2;
    let mut file = std::fs::File::create("circuit.dot").unwrap();
    writeln!(&mut file, "digraph G {{")?;
    writeln!(
        &mut file,
        "subgraph Inputs {{\n{{ rank=same {} }}\n{}\n}}",
        input.keys().join(" "),
        (0..width)
            .map(|i| format!("x{:02} -> y{:02}", i, i))
            .join(" -> ")
    )?;
    for i in 0..width {
        let s = format!("x{:02}", i);
        writeln!(
            &mut file,
            "{} [label=\"{}:{}\" shape=plaintext];",
            s,
            s,
            input[s.as_str()]
        )?;
        let s = format!("y{:02}", i);
        writeln!(
            &mut file,
            "{} [label=\"{}:{}\" shape=plaintext];",
            s,
            s,
            input[s.as_str()]
        )?;
    }

    for (op, ina, inb, out) in gates {
        writeln!(&mut file, "{} [label=\"{} {} {}\n-> {}\" shape=invhouse]", out, ina, op, inb, out)?;
        writeln!(&mut file, "{} -> {}", ina, out)?;
        writeln!(&mut file, "{} -> {}", inb, out)?;


        // // Make and label out node
        // writeln!(
        //     &mut file,
        //     "{} [label=\"{}\" shape=plaintext];",
        //     out, out
        // )?;
        // // Make gate
        // writeln!(&mut file, "{}_{} [label=\"{}\" shape=invhouse]", ina, inb, op)?;
        // // Connect inputs - gate - output
        // writeln!(&mut file, "{{ {} {} }} -> {}_{}", ina, inb, ina, inb)?;
        // // writeln!(&mut file, "{} -> {}_{}", ina, ina, inb)?;
        // // writeln!(&mut file, "{} -> {}_{}", inb, ina, inb)?;
        // // writeln!(&mut file, "{}_{} -> {}", ina, inb, out)?;
    }
    writeln!(&mut file, "}}")?;
    Ok(())
}

fn compute<'a>(
    mut values: HashMap<&'a str, usize>,
    mut nodes: VecDeque<(&'a str, &str, &'a str, &'a str)>,
) -> HashMap<&'a str, usize> {
    while let Some((op, ina, inb, out)) = nodes.pop_front() {
        if values.contains_key(ina) && values.contains_key(inb) {
            values.insert(
                out,
                match op {
                    "AND" => values[ina] & values[inb],
                    "OR" => values[ina] | values[inb],
                    "XOR" => values[ina] ^ values[inb],
                    _ => unreachable!("{} is not a valid op.", op),
                },
            );
            println!("{} = {} {} {} = {}", out, ina, op, inb, values[out]);
        } else {
            nodes.push_back((op, ina, inb, out));
        }
    }
    values
}

/// (Solved, 15min) Propagate signals trough a list of gates, outputting the
/// Z-signals.
fn part1(inputs: &str) -> usize {
    let (xyin, logic) = inputs.split_once("\n\n").unwrap();
    let data_width = xyin.lines().count() / 2;
    // Parse input signals
    let values = xyin
        .lines()
        .map(|line| {
            let (k, v) = line.trim().split_once(": ").unwrap();
            (k, v.parse::<usize>().unwrap())
        })
        .collect::<HashMap<&str, usize>>();
    // Collect operations
    let nodes = logic
        .lines()
        .map(|line| {
            let mut line = line.split_whitespace();
            let ina = line.next().unwrap();
            let op = line.next().unwrap();
            let inb = line.next().unwrap();
            let out = line.skip(1).next().unwrap();
            (op, ina, inb, out)
        })
        .collect::<VecDeque<(&str, &str, &str, &str)>>();
    // Solve as fix-point
    let values = compute(values, nodes);
    // Collect output values for printing
    (0..data_width).fold(0, |az, i| {
        az + values[format!("z{:02}", i).as_str()] * 2usize.pow(i as u32)
    })
}

/// (Solved by visual inspection, >3h) Figure out which 4 individual pairs of
/// output wires need to be swapped in order for the circuit to produce the sum
/// of X and Y into Z.
///
/// The circuit should be a chained bitwise adder with carry
///
/// C <- OR(AND(C0, AND(X, Y)), AND(C0, XOR(X, Y)))
/// S <- OR(AND(C0, AND(X, Y)), XOR(C0, XOR(X, Y)))
fn part2(inputs: &str) -> String {
    let (xyin, logic) = inputs.split_once("\n\n").unwrap();
    let data_width = xyin.lines().count() / 2;
    // Parse input signals
    let values = xyin
        .lines()
        .map(|line| {
            let (k, v) = line.trim().split_once(": ").unwrap();
            (k, v.parse::<usize>().unwrap())
        })
        .collect::<HashMap<&str, usize>>();
    // Collect operations, swapping wires
    let nodes = logic
        .lines()
        .map(|line| {
            let mut line = line.split_whitespace();
            let ina = line.next().unwrap();
            let op = line.next().unwrap();
            let inb = line.next().unwrap();
            let out = match line.skip(1).next().unwrap() {
                "z06" => "jmq",
                "jmq" => "z06",
                "z13" => "gmh",
                "gmh" => "z13",
                "z38" => "qrh",
                "qrh" => "z38",
                "cbd" => "rqf",
                "rqf" => "cbd",
                s => s,
            };
            // if out == "x06" { out = "x06"; }
            // if out
            (op, ina, inb, out)
        })
        .collect::<VecDeque<(&str, &str, &str, &str)>>();
    // Collect input values for printing
    let (xin, yin) = (0..data_width).fold((0, 0), |(ax, ay), i| {
        (
            ax + values[format!("x{:02}", i).as_str()] * 2usize.pow(i as u32),
            ay + values[&format!("y{:02}", i).as_str()] * 2usize.pow(i as u32),
        )
    });
    let zexp = xin + yin;
    println!(
        "Target circuit: {} + {} = {}\n{:0b} + {:0b} = {:0b}",
        xin,
        yin,
        zexp,
        xin,
        yin,
        zexp
    );
    let _ = draw(values.clone(), nodes.clone());
    // Solve as fix-point
    let values = compute(values, nodes.clone());
    // Collect output values for printing
    let zout = (0..data_width+1).fold(0, |az, i| {
        az + values[format!("z{:02}", i).as_str()] * 2usize.pow(i as u32)
    });
    println!("act: {:048b}\nexp: {:048b}", zout, zexp);
    for i in 0..data_width+1 {
        if zexp & (2usize.pow(i as u32) - 1) != zout & (2usize.pow(i as u32) - 1) {
            println!("error at bit {}", i-1);
            break;
        }
    }

    // Visual inspection:
    // x06-cjt, y06-sfm, z13-gmh

    // while let Some(op, ina, inb, out)

    // Find x00 AND y00 -> r00
    // Find r00 XOR y00 -> r01, expect r01 == z00
    // Find x01 AND y01 -> r02
    // Find x01 XOR y02 -> r03
    // Find r03
    let mut wires = vec![
        "z06",
        "jmq",
        "z13",
        "gmh",
        "z38",
        "qrh",
        "rqf",
        "cbd",
    ].into_iter().sorted();

    String::from(wires.join(","))
}

#[allow(unused)]
static TEST_INPUT: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

#[test]
fn test_2024_day24_part1() {
    assert_eq!(part1(TEST_INPUT), 2024);
}

#[test]
fn test_2024_day24_part2() {
    let test_input = "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00";
    assert_eq!(part2(test_input), String::from("z00,z01,z02,z05"));
    assert_eq!(
        part2(TEST_INPUT),
        String::from("aaa,aoc,bbb,ccc,eee,ooo,z24,z99")
    );
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
