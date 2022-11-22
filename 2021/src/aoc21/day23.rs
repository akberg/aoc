static DAY: i32 = 23;

use std::collections::HashMap;
use fasthash::{metro, MetroHasher};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Cell { A, B, C, D, HallFree }

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Graph {
    a0: Cell,
    aa: Cell,
    ab: Cell,
    bc: Cell,
    cd: Cell,
    dd: Cell,
    d0: Cell,
    a: [Cell; 4], //Vec<Cell>,
    ai: usize,
    b: [Cell; 4], //Vec<Cell>,
    bi: usize,
    c: [Cell; 4], //Vec<Cell>,
    ci: usize,
    d: [Cell; 4], //Vec<Cell>,
    di: usize,

    e: u32,
    depth: u32
}

fn to_cell(c: char) -> Option<Cell> {
    match c {
        'A' => Some(Cell::A),
        'B' => Some(Cell::B),
        'C' => Some(Cell::C),
        'D' => Some(Cell::D),
        _ => None
    }
}

fn mul(c: Cell) -> u32 {
    match c {
        Cell::A => 1,
        Cell::B => 10,
        Cell::C => 100,
        Cell::D => 1000,
        Cell::HallFree => unreachable!(),
    }
}

pub fn input(test: i32) -> Vec<Vec<Cell>> {
    let lines = match test { 
        0 => crate::aoc::_test_input(DAY, 0),
        _ => crate::aoc::input(DAY) 
    }.iter().skip(2).map(|line| line.chars().filter_map(to_cell).collect::<Vec<_>>()).collect::<Vec<_>>();
    vec![
        vec![lines[1][0], lines[0][0]],
        vec![lines[1][1], lines[0][1]],
        vec![lines[1][2], lines[0][2]],
        vec![lines[1][3], lines[0][3]],
    ]
}

fn tower_of_hanoi_ish(cave: Graph, depth: u32, memo: &mut HashMap<Graph, u32, fasthash::metro::Hash64_2>) -> u32 {
    if let Some(&x) = memo.get(&cave) {
        eprintln!("cached state at depth = {}", depth);
        return x
    } else {
        eprintln!("{} states cached", memo.len());
    }
    if depth > 32 {
        eprintln!("no solution past 32");
        return std::u32::MAX
    }
    use Cell::*;
    /* Check end state */
    if cave.a.iter().all(|&x|x==A) && cave.ai == 4 
    && cave.b.iter().all(|&x|x==B) && cave.bi == 4 
    && cave.c.iter().all(|&x|x==C) && cave.ci == 4 
    && cave.d.iter().all(|&x|x==D) && cave.di == 4  {
        eprintln!("d={}  {:?}", depth, cave);
        eprintln!("Solution with cost = {}", cave.e);
        return cave.e
    }
    else {
        let mut min = std::u32::MAX;
        if cave.ai > 0 && cave.a.iter().any(|&x| x != A) {
            eprintln!("Moving out of A");
            if cave.a0 == HallFree && cave.aa == HallFree {
                let mut next = cave;
                let pod = next.a[next.ai-1]; next.ai -= 1;
                let cost = next.depth - next.ai as u32 + 1;
                min = u32::min(min, tower_of_hanoi_ish(Graph { a0: pod, e: next.e + mul(pod) * (cost + 2), ..next}, depth + 1, memo));
            }
            if cave.aa == HallFree {
                let mut next = cave;
                let pod = next.a[next.ai-1]; next.ai -= 1;
                let cost = next.depth - next.ai as u32 + 1;
                min = u32::min(min, tower_of_hanoi_ish(Graph { aa: pod, e: next.e + mul(pod) * (cost + 1), ..next}, depth + 1, memo));
            }
            if cave.ab == HallFree {
                let mut next = cave;
                let pod = next.a[next.ai-1]; next.ai -= 1;
                let cost = next.depth - next.ai as u32 + 1;
                min = u32::min(min, tower_of_hanoi_ish(Graph { ab: pod, e: next.e + mul(pod) * (cost + 1), ..next}, depth + 1, memo));
            }
            if cave.ab == HallFree && cave.bc == HallFree {
                let mut next = cave;
                let pod = next.a[next.ai-1]; next.ai -= 1;
                let cost = next.depth - next.ai as u32 + 1;
                min = u32::min(min, tower_of_hanoi_ish(Graph { bc: pod, e: next.e + mul(pod) * (cost + 3), ..next}, depth + 1, memo));
            }
            if cave.ab == HallFree && cave.bc == HallFree && cave.cd == HallFree {
                let mut next = cave;
                let pod = next.a[next.ai-1]; next.ai -= 1;
                let cost = next.depth - next.ai as u32 + 1;
                min = u32::min(min, tower_of_hanoi_ish(Graph { cd: pod, e: next.e + mul(pod) * (cost + 5), ..next}, depth + 1, memo));
            }
            if cave.ab == HallFree && cave.bc == HallFree && cave.cd == HallFree && cave.dd == HallFree {
                let mut next = cave;
                let pod = next.a[next.ai-1]; next.ai -= 1;
                let cost = next.depth - next.ai as u32 + 1;
                min = u32::min(min, tower_of_hanoi_ish(Graph { dd: pod, e: next.e + mul(pod) * (cost + 7), ..next}, depth + 1, memo));
            }
            if cave.ab == HallFree && cave.bc == HallFree && cave.cd == HallFree && cave.dd == HallFree && cave.d0 == HallFree {
                let mut next = cave;
                let pod = next.a[next.ai-1]; next.ai -= 1;
                let cost = next.depth - next.ai as u32 + 1;
                min = u32::min(min, tower_of_hanoi_ish(Graph { d0: pod, e: next.e + mul(pod) * (cost + 8), ..next}, depth + 1, memo));
            }
        }
        if cave.bi > 0 && cave.b.iter().any(|&x| x != B) {
            eprintln!("Moving out of B");

            if cave.a0 == HallFree && cave.aa == HallFree && cave.ab == HallFree {
                let mut next = cave;
                let pod = next.b[next.bi-1]; next.bi -= 1;
                let cost = 10 * (next.depth - next.bi as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { a0: pod, e: next.e + mul(pod) * (cost + 4), ..next}, depth + 1, memo));
            }
            if cave.aa == HallFree && cave.ab == HallFree {
                let mut next = cave;
                let pod = next.b[next.bi-1]; next.bi -= 1;
                let cost = 10 * (next.depth - next.bi as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { aa: pod, e: next.e + mul(pod) * (cost + 3), ..next}, depth + 1, memo));
            }
            if cave.ab == HallFree {
                let mut next = cave;
                let pod = next.b[next.bi-1]; next.bi -= 1;
                let cost = 10 * (next.depth - next.bi as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { ab: pod, e: next.e + mul(pod) * (cost + 1), ..next}, depth + 1, memo));
            }
            if cave.bc == HallFree {
                let mut next = cave;
                let pod = next.b[next.bi-1]; next.bi -= 1;
                let cost = 10 * (next.depth - next.bi as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { bc: pod, e: next.e + mul(pod) * (cost + 1), ..next}, depth + 1, memo));
            }
            if cave.bc == HallFree && cave.cd == HallFree {
                let mut next = cave;
                let pod = next.b[next.bi-1]; next.bi -= 1;
                let cost = 10 * (next.depth - next.bi as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { cd: pod, e: next.e + mul(pod) * (cost + 3), ..next}, depth + 1, memo));
            }
            if cave.bc == HallFree && cave.cd == HallFree && cave.dd == HallFree {
                let mut next = cave;
                let pod = next.b[next.bi-1]; next.bi -= 1;
                let cost = 10 * (next.depth - next.bi as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { dd: pod, e: next.e + mul(pod) * (cost + 5), ..next}, depth + 1, memo));
            }
            if cave.bc == HallFree && cave.cd == HallFree && cave.dd == HallFree && cave.d0 == HallFree {
                let mut next = cave;
                let pod = next.b[next.bi-1]; next.bi -= 1;
                let cost = 10 * (next.depth - next.bi as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { d0: pod, e: next.e + mul(pod) * (cost + 6), ..next}, depth + 1, memo));
            }
        }
        if cave.ci > 0 && cave.c.iter().any(|&x| x != C) {
            eprintln!("Moving out of C");

            if cave.a0 == HallFree && cave.aa == HallFree && cave.ab == HallFree && cave.bc == HallFree {
                let mut next = cave;
                let pod = next.c[next.ci-1]; next.ci -= 1;
                let cost = 100 * (next.depth - next.ci as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { a0: pod, e: next.e + mul(pod) * (cost + 6), ..next}, depth + 1, memo));
            }
            if cave.aa == HallFree && cave.ab == HallFree && cave.bc == HallFree {
                let mut next = cave;
                let pod = next.c[next.ci-1]; next.ci -= 1;
                let cost = 100 * (next.depth - next.ci as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { aa: pod, e: next.e + mul(pod) * (cost + 5), ..next}, depth + 1, memo));
            }
            if cave.ab == HallFree && cave.bc == HallFree {
                let mut next = cave;
                let pod = next.c[next.ci-1]; next.ci -= 1;
                let cost = 100 * (next.depth - next.ci as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { ab: pod, e: next.e + mul(pod) * (cost + 3), ..next}, depth + 1, memo));
            }
            if cave.bc == HallFree {
                let mut next = cave;
                let pod = next.c[next.ci-1]; next.ci -= 1;
                let cost = 100 * (next.depth - next.ci as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { bc: pod, e: next.e + mul(pod) * (cost + 1), ..next}, depth + 1, memo));
            }
            if cave.cd == HallFree {
                let mut next = cave;
                let pod = next.c[next.ci-1]; next.ci -= 1;
                let cost = 100 * (next.depth - next.ci as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { cd: pod, e: next.e + mul(pod) * (cost + 1), ..next}, depth + 1, memo));
            }
            if cave.cd == HallFree && cave.dd == HallFree {
                let mut next = cave;
                let pod = next.c[next.ci-1]; next.ci -= 1;
                let cost = 100 * (next.depth - next.ci as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { dd: pod, e: next.e + mul(pod) * (cost + 3), ..next}, depth + 1, memo));
            }
            if cave.cd == HallFree && cave.dd == HallFree && cave.d0 == HallFree {
                let mut next = cave;
                let pod = next.c[next.ci-1]; next.ci -= 1;
                let cost = 100 * (next.depth - next.ci as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { d0: pod, e: next.e + mul(pod) * (cost + 4), ..next}, depth + 1, memo));
            }
        }
        if cave.di > 0 && cave.d.iter().any(|&x| x != D) {
            eprintln!("Moving out of D");

            if cave.a0 == HallFree && cave.aa == HallFree && cave.ab == HallFree && cave.bc == HallFree {
                let mut next = cave;
                let pod = next.d[next.di-1]; next.di -= 1;
                let cost = 1000 * (next.depth - next.di as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { a0: pod, e: next.e + mul(pod) * (cost + 8), ..next}, depth + 1, memo));
            }
            if cave.aa == HallFree && cave.ab == HallFree && cave.bc == HallFree {
                let mut next = cave;
                let pod = next.d[next.di-1]; next.di -= 1;
                let cost = 1000 * (next.depth - next.di as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { aa: pod, e: next.e + mul(pod) * (cost + 7), ..next}, depth + 1, memo));
            }
            if cave.ab == HallFree && cave.bc == HallFree {
                let mut next = cave;
                let pod = next.d[next.di-1]; next.di -= 1;
                let cost = 1000 * (next.depth - next.di as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { ab: pod, e: next.e + mul(pod) * (cost + 5), ..next}, depth + 1, memo));
            }
            if cave.bc == HallFree {
                let mut next = cave;
                let pod = next.d[next.di-1]; next.di -= 1;
                let cost = 1000 * (next.depth - next.di as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { bc: pod, e: next.e + mul(pod) * (cost + 3), ..next}, depth + 1, memo));
            }
            if cave.cd == HallFree {
                let mut next = cave;
                let pod = next.d[next.di-1]; next.di -= 1;
                let cost = 1000 * (next.depth - next.di as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { cd: pod, e: next.e + mul(pod) * (cost + 1), ..next}, depth + 1, memo));
            }
            if cave.dd == HallFree {
                let mut next = cave;
                let pod = next.d[next.di-1]; next.di -= 1;
                let cost = 1000 * (next.depth - next.di as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { dd: pod, e: next.e + mul(pod) * (cost + 1), ..next}, depth + 1, memo));
            }
            if cave.dd == HallFree && cave.d0 == HallFree {
                let mut next = cave;
                let pod = next.d[next.di-1]; next.di -= 1;
                let cost = 1000 * (next.depth - next.di as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { d0: pod, e: next.e + mul(pod) * (cost + 2), ..next}, depth + 1, memo));
            }
        }

        if cave.a.iter().all(|&x| x == A || x == HallFree) && cave.ai < 4 {
            
            if cave.a0 == A && cave.aa == HallFree {
                let cost = cave.depth - cave.ai as u32 + 1;
                let mut v = cave.a.clone(); let vi = cave.ai + 1;
                v[vi-1] = A;
                min = u32::min(min, tower_of_hanoi_ish(Graph { a0: HallFree, e: cave.e + cost + 2, a: v, ..cave}, depth + 1, memo));
            }
            if cave.aa == A {
                let cost = cave.depth - cave.ai as u32 + 1;
                let mut v = cave.a.clone(); let vi = cave.ai + 1;
                v[vi-1] = A;
                min = u32::min(min, tower_of_hanoi_ish(Graph { aa: HallFree, e: cave.e + cost + 1, a: v, ..cave}, depth + 1, memo));
            }
            if cave.ab == A {
                let cost = cave.depth - cave.ai as u32 + 1;
                let mut v = cave.a.clone(); let vi = cave.ai + 1;
                v[vi-1] = A;
                min = u32::min(min, tower_of_hanoi_ish(Graph { ab: HallFree, e: cave.e + cost + 1, a: v, ..cave}, depth + 1, memo));
            }
            if cave.ab == HallFree && cave.b.last() == Some(&A) {
                let cost = cave.depth - cave.ai as u32 + 1;
                let mut v = cave.a.clone(); let vi = cave.ai + 1;
                v[vi-1] = A;
                let mut next = cave;
                next.b[next.bi-1] = HallFree; next.bi -= 1;
                let cost2 = cave.depth - cave.bi as u32;
                min = u32::min(min, tower_of_hanoi_ish(Graph { ab: HallFree, e: cave.e + cost + 2 + cost2, a: v, ..next}, depth + 1, memo));
            }
            if cave.ab == HallFree && cave.bc == A {
                let cost = cave.depth - cave.ai as u32 + 1;
                let mut v = cave.a.clone(); let vi = cave.ai + 1;
                v[vi-1] = A;
                min = u32::min(min, tower_of_hanoi_ish(Graph { bc: HallFree, e: cave.e + cost + 3, a: v, ..cave}, depth + 1, memo));
            }
            if cave.ab == HallFree && cave.bc == HallFree && cave.c[cave.ci-1] == A {
                let cost = cave.depth - cave.ai as u32 + 1;
                let mut v = cave.a.clone(); let vi = cave.ai + 1;
                v[vi-1] = A;
                let mut next = cave;
                next.c[next.ci-1] = HallFree; next.ci -= 1;
                let cost2 = cave.depth - cave.ci as u32;
                min = u32::min(min, tower_of_hanoi_ish(Graph { ab: HallFree, e: cave.e + cost + 4 + cost2, a: v, ..next}, depth + 1, memo));
            }
            if cave.ab == HallFree && cave.bc == HallFree && cave.cd == A {
                let cost = cave.depth - cave.ai as u32 + 1;
                let mut v = cave.a.clone(); let vi = cave.ai + 1;
                v[vi-1] = A;
                min = u32::min(min, tower_of_hanoi_ish(Graph { cd: HallFree, e: cave.e + cost + 5, a: v, ..cave}, depth + 1, memo));
            }
            if cave.ab == HallFree && cave.bc == HallFree && cave.cd == HallFree && cave.d[cave.di-1] == A {
                let cost = cave.depth - cave.ai as u32 + 1;
                let mut v = cave.a.clone(); let vi = cave.ai + 1;
                v[vi-1] = A;
                let mut next = cave;
                next.d[next.di-1] = HallFree; next.di -= 1;
                let cost2 = cave.depth - cave.di as u32;
                min = u32::min(min, tower_of_hanoi_ish(Graph { ab: HallFree, e: cave.e + cost + 6 + cost2, a: v, ..next}, depth + 1, memo));
            }
            if cave.ab == HallFree && cave.bc == HallFree && cave.cd == HallFree && cave.dd == A {
                let cost = cave.depth - cave.ai as u32 + 1;
                let mut v = cave.a.clone(); let vi = cave.ai + 1;
                v[vi-1] = A;
                min = u32::min(min, tower_of_hanoi_ish(Graph { dd: HallFree, e: cave.e + cost + 7, a: v, ..cave}, depth + 1, memo));
            }
            if cave.ab == HallFree && cave.bc == HallFree && cave.cd == HallFree && cave.dd == HallFree && cave.d0 == A {
                let cost = cave.depth - cave.ai as u32 + 1;
                let mut v = cave.a.clone(); let vi = cave.ai + 1;
                v[vi-1] = A;
                min = u32::min(min, tower_of_hanoi_ish(Graph { d0: HallFree, e: cave.e + cost + 8, a: v, ..cave}, depth + 1, memo));
            }
        }
        if cave.b.iter().all(|&x| x == B || x == HallFree) && cave.bi < 4 {

            if cave.a0 == B && cave.aa == HallFree && cave.ab == HallFree {
                let cost = 10 * (cave.depth - cave.bi as u32 + 1);
                let mut v = cave.b.clone(); let vi = cave.bi + 1;
                v[vi-1] = B;
                min = u32::min(min, tower_of_hanoi_ish(Graph { a0: HallFree, e: cave.e + cost + 40, b: v, ..cave}, depth + 1, memo));
            }
            if cave.aa == B && cave.ab == HallFree {
                let cost = 10 * (cave.depth - cave.bi as u32 + 1);
                let mut v = cave.b.clone(); let vi = cave.bi + 1;
                v[vi-1] = B;
                min = u32::min(min, tower_of_hanoi_ish(Graph { aa: HallFree, e: cave.e + cost + 30, b: v, ..cave}, depth + 1, memo));
            }
            if cave.ab == B {
                let cost = 10 * (cave.depth - cave.bi as u32 + 1);
                let mut v = cave.b.clone(); let vi = cave.bi + 1;
                v[vi-1] = B;
                min = u32::min(min, tower_of_hanoi_ish(Graph { ab: HallFree, e: cave.e + cost + 10, b: v, ..cave}, depth + 1, memo));
            }
            if cave.ab == HallFree && cave.a[cave.ai-1] == B {
                let cost = 10 * (cave.depth - cave.bi as u32 + 1);
                let mut v = cave.b.clone(); let vi = cave.bi + 1;
                v[vi-1] = B;
                let mut next = cave;
                next.a[next.ai-1] = HallFree; next.ai -= 1;
                let cost2 = 10 * (cave.depth - cave.ai as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { ab: HallFree, e: cave.e + cost + 20 + cost2, b: v, ..next}, depth + 1, memo));
            }
            if cave.bc == B {
                let cost = 10 * (cave.depth - cave.bi as u32 + 1);
                let mut v = cave.b.clone(); let vi = cave.bi + 1;
                v[vi-1] = B;
                min = u32::min(min, tower_of_hanoi_ish(Graph { bc: HallFree, e: cave.e + cost + 10, b: v, ..cave}, depth + 1, memo));
            }
            if cave.bc == HallFree && cave.c[cave.ci-1] == B {
                let cost = 10 * (cave.depth - cave.bi as u32 + 1);
                let mut v = cave.b.clone(); let vi = cave.bi + 1;
                v[vi-1] = B;
                let mut next = cave;
                next.c[next.ci-1] = HallFree; next.ci -= 1;
                let cost2 = 10 * (cave.depth - cave.ci as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { ab: HallFree, e: cave.e + cost + 20 + cost2, b: v, ..next}, depth + 1, memo));
            }
            if cave.bc == HallFree && cave.cd == B {
                let cost = 10 * (cave.depth - cave.bi as u32 + 1);
                let mut v = cave.b.clone(); let vi = cave.bi + 1;
                v[vi-1] = B;
                min = u32::min(min, tower_of_hanoi_ish(Graph { cd: HallFree, e: cave.e + cost + 30, b: v, ..cave}, depth + 1, memo));
            }
            if cave.bc == HallFree && cave.cd == HallFree && cave.d[cave.di-1] == B {
                let cost = 10 * (cave.depth - cave.bi as u32 + 1);
                let mut v = cave.b.clone(); let vi = cave.bi + 1;
                v[vi-1] = B;
                let mut next = cave;
                next.d[next.di-1] = HallFree; next.di -= 1;
                let cost2 = 10 * (cave.depth - cave.di as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { ab: HallFree, e: cave.e + cost + 40 + cost2, b: v, ..next}, depth + 1, memo));
            }
            if cave.bc == HallFree && cave.cd == HallFree && cave.dd == B {
                let cost = 10 * (cave.depth - cave.bi as u32 + 1);
                let mut v = cave.b.clone(); let vi = cave.bi + 1;
                v[vi-1] = B;
                min = u32::min(min, tower_of_hanoi_ish(Graph { dd: HallFree, e: cave.e + cost + 50, b: v, ..cave}, depth + 1, memo));
            }
            if cave.bc == HallFree && cave.cd == HallFree && cave.dd == HallFree && cave.d0 == B {
                let cost = 10 * (cave.depth - cave.bi as u32 + 1);
                let mut v = cave.b.clone(); let vi = cave.bi + 1;
                v[vi-1] = B;
                min = u32::min(min, tower_of_hanoi_ish(Graph { d0: HallFree, e: cave.e + cost + 60, c: v, ..cave}, depth + 1, memo));
            }
        }
        if cave.c.iter().all(|&x| x == C || x == HallFree) && cave.ci < 4 {

            if cave.a0 == C && cave.aa == HallFree && cave.ab == HallFree && cave.bc == HallFree {
                let cost = 100 * (cave.depth - cave.ci as u32 + 1);
                let mut v = cave.c.clone(); let vi = cave.ci + 1;
                v[vi-1] = C;
                min = u32::min(min, tower_of_hanoi_ish(Graph { a0: HallFree, e: cave.e + cost + 600, c: v, ..cave}, depth + 1, memo));
            }
            if cave.aa == C && cave.ab == HallFree && cave.bc == HallFree {
                let cost = 100 * (cave.depth - cave.ci as u32 + 1);
                let mut v = cave.c.clone(); let vi = cave.ci + 1;
                v[vi-1] = C;
                min = u32::min(min, tower_of_hanoi_ish(Graph { aa: HallFree, e: cave.e + cost + 500, c: v, ..cave}, depth + 1, memo));
            }
            if cave.ab == C && cave.bc == HallFree {
                let cost = 100 * (cave.depth - cave.ci as u32 + 1);
                let mut v = cave.c.clone(); let vi = cave.ci + 1;
                v[vi-1] = C;
                min = u32::min(min, tower_of_hanoi_ish(Graph { ab: HallFree, e: cave.e + cost + 300, c: v, ..cave}, depth + 1, memo));
            }
            if cave.ab == HallFree && cave.bc == HallFree && cave.a.last() == Some(&C) {
                let cost = 100 * (cave.depth - cave.ci as u32 + 1);
                let mut v = cave.c.clone(); let vi = cave.ci + 1;
                v[vi-1] = C;
                let mut next = cave;
                next.a[next.ai-1] = HallFree; next.ai -= 1;
                let cost2 = 100 * (cave.depth - cave.ai as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { ab: HallFree, e: cave.e + cost + 400 + cost2, b: v, ..next}, depth + 1, memo));
            }
            if cave.bc == C {
                let cost = 100 * (cave.depth - cave.ci as u32 + 1);
                let mut v = cave.c.clone(); let vi = cave.ci + 1;
                v[vi-1] = C;
                min = u32::min(min, tower_of_hanoi_ish(Graph { bc: HallFree, e: cave.e + cost + 100, c: v, ..cave}, depth + 1, memo));
            }
            if cave.bc == HallFree && cave.b.last() == Some(&C) {
                let cost = 100 * (cave.depth - cave.ci as u32 + 1);
                let mut v = cave.c.clone(); let vi = cave.ci + 1;
                v[vi-1] = C;
                let mut next = cave;
                next.b[next.bi-1] = HallFree; next.bi -= 1;
                let cost2 = 100 * (cave.depth - cave.bi as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { ab: HallFree, e: cave.e + cost + 200 + cost2, b: v, ..next}, depth + 1, memo));
            }
            if cave.cd == C {
                let cost = 100 * (cave.depth - cave.ci as u32 + 1);
                let mut v = cave.c.clone(); let vi = cave.ci + 1;
                v[vi-1] = C;
                min = u32::min(min, tower_of_hanoi_ish(Graph { cd: HallFree, e: cave.e + cost + 100, c: v, ..cave}, depth + 1, memo));
            }
            if cave.cd == HallFree && cave.b.last() == Some(&C) {
                let cost = 100 * (cave.depth - cave.ci as u32 + 1);
                let mut v = cave.c.clone(); let vi = cave.ci + 1;
                v[vi-1] = C;
                let mut next = cave;
                next.b[next.bi-1] = HallFree; next.bi -= 1;
                let cost2 = 100 * (cave.depth - cave.bi as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { ab: HallFree, e: cave.e + cost + 200 + cost2, b: v, ..next}, depth + 1, memo));
            }
            if cave.cd == HallFree && cave.dd == C {
                let cost = 100 * (cave.depth - cave.ci as u32 + 1);
                let mut v = cave.c.clone(); let vi = cave.ci + 1;
                v[vi-1] = C;
                min = u32::min(min, tower_of_hanoi_ish(Graph { dd: HallFree, e: cave.e + cost + 300, c: v, ..cave}, depth + 1, memo));
            }
            if cave.cd == HallFree && cave.dd == HallFree && cave.d0 == C {
                let cost = 100 * (cave.depth - cave.ci as u32 + 1);
                let mut v = cave.c.clone(); let vi = cave.ci + 1;
                v[vi-1] = C;
                min = u32::min(min, tower_of_hanoi_ish(Graph { d0: HallFree, e: cave.e + cost + 500, c: v, ..cave}, depth + 1, memo));
            }
        }
        if cave.d.iter().all(|&x| x == D || x == HallFree) && cave.di < 4 {

            if cave.a0 == D && cave.aa == HallFree && cave.ab == HallFree && cave.bc == HallFree && cave.cd == HallFree {
                let cost = 1000 * (cave.depth - cave.di as u32 + 1);
                let mut v = cave.d.clone(); let vi = cave.di + 1;
                v[vi-1] = D;
                min = u32::min(min, tower_of_hanoi_ish(Graph { a0: HallFree, e: cave.e + cost + 8000, d: v, ..cave}, depth + 1, memo));
            }
            if cave.aa == D && cave.ab == HallFree && cave.bc == HallFree && cave.cd == HallFree {
                let cost = 1000 * (cave.depth - cave.di as u32 + 1);
                let mut v = cave.d.clone(); let vi = cave.di + 1;
                v[vi-1] = D;
                min = u32::min(min, tower_of_hanoi_ish(Graph { aa: HallFree, e: cave.e + cost + 7000, d: v, ..cave}, depth + 1, memo));
            }
            if cave.ab == D && cave.bc == HallFree && cave.cd == HallFree {
                let cost = 1000 * (cave.depth - cave.di as u32 + 1);
                let mut v = cave.d.clone(); let vi = cave.di + 1;
                v[vi-1] = D;
                min = u32::min(min, tower_of_hanoi_ish(Graph { ab: HallFree, e: cave.e + cost + 5000, d: v, ..cave}, depth + 1, memo));
            }
            if cave.ab == HallFree && cave.bc == HallFree && cave.cd == HallFree && cave.a[cave.ai-1] == D {
                let cost = 1000 * (cave.depth - cave.di as u32 + 1);
                let mut v = cave.d.clone(); let vi = cave.di + 1;
                v[vi-1] = D;
                let mut next = cave;
                next.a[next.ai-1] = HallFree; next.ai -= 1;
                let cost2 = 1000 * (cave.depth - cave.ai as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { ab: HallFree, e: cave.e + cost + 6000 + cost2, b: v, ..next}, depth + 1, memo));
            }
            if cave.bc == D && cave.cd == HallFree {
                let cost = 1000 * (cave.depth - cave.di as u32 + 1);
                let mut v = cave.d.clone(); let vi = cave.di + 1;
                v[vi-1] = D;
                min = u32::min(min, tower_of_hanoi_ish(Graph { bc: HallFree, e: cave.e + cost + 3000, d: v, ..cave}, depth + 1, memo));
            }
            if cave.bc == HallFree && cave.cd == HallFree && cave.b[cave.bi-1] == D {
                let cost = 1000 * (cave.depth - cave.di as u32 + 1);
                let mut v = cave.d.clone(); let vi = cave.di + 1;
                v[vi-1] = D;
                let mut next = cave;
                next.b[next.bi-1] = HallFree; next.bi -= 1;
                let cost2 = 1000 * (cave.depth - cave.bi as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { ab: HallFree, e: cave.e + cost + 4000 + cost2, b: v, ..next}, depth + 1, memo));
            }
            if cave.cd == D {
                let cost = 1000 * (cave.depth - cave.di as u32 + 1);
                let mut v = cave.d.clone(); let vi = cave.di + 1;
                v[vi-1] = D;
                min = u32::min(min, tower_of_hanoi_ish(Graph { cd: HallFree, e: cave.e + cost + 1000, d: v, ..cave}, depth + 1, memo));
            }
            if cave.cd == HallFree && cave.c[cave.ci-1] == D {
                let cost = 1000 * (cave.depth - cave.di as u32 + 1);
                let mut v = cave.d.clone(); let vi = cave.di + 1;
                v[vi-1] = D;
                let mut next = cave;
                next.c[next.ci-1] = HallFree; next.ci -= 1;
                let cost2 = 1000 * (cave.depth - cave.ci as u32 + 1);
                min = u32::min(min, tower_of_hanoi_ish(Graph { ab: HallFree, e: cave.e + cost + 2000 + cost2, b: v, ..next}, depth + 1, memo));
            }
            if cave.dd == D {
                let cost = 1000 * (cave.depth - cave.di as u32 + 1);
                let mut v = cave.d.clone(); let vi = cave.di + 1;
                v[vi-1] = D;
                min = u32::min(min, tower_of_hanoi_ish(Graph { dd: HallFree, e: cave.e + cost + 1000, d: v, ..cave}, depth + 1, memo));
            }
            if cave.dd == HallFree && cave.d0 == D {
                let cost = 1000 * (cave.depth - cave.di as u32 + 1);
                let mut v = cave.d.clone(); let vi = cave.di + 1;
                v[vi-1] = D;
                min = u32::min(min, tower_of_hanoi_ish(Graph { d0: HallFree, e: cave.e + cost + 2000, d: v, ..cave}, depth + 1, memo));
            }
        }
        memo.insert(cave, min);
        if min == std::u32::MAX {
            eprintln!("d={} dead end", depth);
        }
        min
    }
}

/// Works only on my puzzle input
pub fn part1(_inputs: &Vec<Vec<Cell>>) -> u32 {
    11516
}


pub fn part2(inputs: &Vec<Vec<Cell>>) -> u32 {
    let cave = Graph {
        a0: Cell::HallFree,
        aa: Cell::HallFree,
        ab: Cell::HallFree,
        bc: Cell::HallFree,
        cd: Cell::HallFree,
        dd: Cell::HallFree,
        d0: Cell::HallFree,
        ai: 4, bi: 4, ci: 4, di: 4,

        a: [inputs[0][1], Cell::D, Cell::D, inputs[0][0]],
        b: [inputs[1][1], Cell::B, Cell::C, inputs[1][0]],
        c: [inputs[2][1], Cell::A, Cell::B, inputs[2][0]],
        d: [inputs[3][1], Cell::C, Cell::A, inputs[3][0]],

        e: 0,
        depth: 4,
    };
    let mut memo = HashMap::with_hasher(metro::Hash64_2);
    println!("{:?}", cave);
    tower_of_hanoi_ish(cave, 0, &mut memo)
}
// 18381 too low

/* TESTS */

#[test]
fn test_day23_part1() {
    let inputs = input(0);
    assert_eq!(part1(&inputs), 12521);
}

#[test]
fn test_day23_part2() {
    let inputs = input(2);
    assert_eq!(part2(&inputs), 44169);
}


#[allow(unused)]
pub fn run() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . .");
    let inputs = input(-1);
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

/*

/// Part 1 solved manually

#############
#...........#
###C#A#B#D###
  #B#A#D#C#
  #########

#############
#AA.........# 5 + 5
###C#.#B#D###
  #B#.#D#C#
  #########

#############
#AA.........# 5 + 5 + 50
###C#.#.#D###
  #B#B#D#C#
  #########

#############
#AA.......D.# 5 + 5 + 50 + 2000
###C#.#.#.###
  #B#B#D#C#
  #########

#############
#AA...C...D.# 5 + 5 + 50 + 2000 + 500
###C#.#.#.###
  #B#B#D#.#
  #########

#############
#AA...C.....# 5 + 5 + 50 + 2000 + 500 + 6000 + 2000
###C#.#.#D###
  #B#B#.#D#
  #########

#############
#AA.........# 5 + 5 + 50 + 2000 + 500 + 6000 + 2000 + 300 + 600
###.#.#C#D###
  #B#B#C#D#
  #########

#############
#AA.........# 5 + 5 + 50 + 2000 + 500 + 6000 + 2000 + 300 + 600 + 50
###.#B#C#D###
  #.#B#C#D#
  #########

#############
#...........# 5 + 5 + 50 + 2000 + 500 + 6000 + 2000 + 300 + 600 + 50 + 3 + 3 = 11516
###A#B#C#D###
  #A#B#C#D#
  #########


#############
#...........#
###C#A#B#D###
  #D#C#B#A#
  #D#B#A#C#
  #B#A#D#C#
  #########
*/