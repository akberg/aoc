use criterion_bencher_compat as bencher;
use bencher::{benchmark_group, benchmark_main, Bencher};

use aoc;

fn day_1_part_1(b: &mut Bencher) {
    let inputs = aoc::day1::input();
    b.iter(|| {
        let _ = aoc::day1::part1(&inputs);
    });
}

fn day_1_part_2(b: &mut Bencher) {
    let inputs = aoc::day1::input();
    b.iter(|| {
        let _ = aoc::day1::part2(&inputs);
    })
}

benchmark_group!(benches, day_1_part_1, day_1_part_2);
benchmark_main!(benches);
