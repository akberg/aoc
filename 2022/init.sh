# $1 - day
# $2 - year
function init_day {
    echo "args " $1 " " $2
    printf -v day "%02d" $1
    echo "create file " src/aoc$2/day$day.rs
    echo "static DAY: usize = $day;" > src/aoc$2/day$day.rs
    echo "
pub fn input() -> String {
    crate::aoc::input_raw(1)
        //.lines()
        //.map(|ls| ls.parse::<_>().unwrap())
        //.collect()
}

pub fn part1(inputs: &str) -> u32 {
    todo!();
    0
}

pub fn part2(inputs: &str) -> u32 {
    todo!();
    0
}

#[test]
fn test_day$1_part1() {
    // TODO
}

#[test]
fn test_day$1_part2() {
    // TODO
}

#[allow(unused)]
pub fn run() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!(\"Parsing input . . .\");
    let inputs = input();
    println!(\"{:?}\", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    let res = part1(&inputs);
    print!(\"Day {} part 1: \", DAY);
    println!(\"{}\", res);
    println!(\"Took {:?}\", pt_start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    let res = part2(&inputs);
    print!(\"Day {} part 2: \", DAY);
    println!(\"{}\", res);
    println!(\"Took {:?}\", pt_start.elapsed().unwrap());
    println!(\"Total time: {:?}\", start.elapsed().unwrap());
}

" >> src/aoc$2/day$day.rs
}

if [ -z $1 ]; then
    echo "usage: $0 <year> [first_day=1]"
    exit 1
fi
year=$1
first_day=$2
if [ -z $first_day ]; then
    day=1
fi
echo "year is $year first day is $first_day"

for i in {1..25}
do
    if [[ $i -ge $first_day ]]; then
        init_day $i $year
    fi
done

echo "
/* Days */
pub static RUNS: [fn(); 25] = [
    day01::run,
    day02::run,
    day03::run,
    day04::run,
    day05::run,
    day06::run,
    day07::run,
    day08::run,
    day09::run,
    day10::run,
    day11::run,
    day12::run,
    day13::run,
    day14::run,
    day15::run,
    day16::run,
    day17::run,
    day18::run,
    day19::run,
    day20::run,
    day21::run,
    day22::run,
    day23::run,
    day24::run,
    day25::run,
];

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

" > src/aoc$year.rs
