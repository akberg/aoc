use std::path::Display;

use super::YEAR;
static DAY: usize = 09;

fn input() -> String {
    crate::aoc::input_raw(YEAR, DAY)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Entry {
    Free(usize),
    Alloc(usize, usize),
}
impl Entry {
    pub fn get_id(&self) -> Option<usize> {
        match self {
            Entry::Free(_) => None,
            Entry::Alloc(i, _) => Some(*i),
        }
    }
    pub fn get_size(&self) -> usize {
        match self {
            Entry::Free(s) => *s,
            Entry::Alloc(_i, s) => *s,
        }
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Entry::Free(_) => write!(f, "."),
            Entry::Alloc(i, _) => write!(f, "{}", i),
        }
    }
}

/// (Solved, 40min) Defragment a memory space by moving blocks from the back to the first block
/// of free space.
fn part1(inputs: &str) -> usize {
    // Expand format
    let mut file = inputs
        .trim()
        .char_indices()
        .fold(Vec::new(), |mut arr, (i, e)| {
            let size = e.to_digit(10).unwrap() as usize;
            if i % 2 == 0 {
                arr.append(&mut vec![Entry::Alloc(i / 2, size); size]);
            } else {
                arr.append(&mut vec![Entry::Free(size); size]);
            }
            arr
        });
    let mut i = 0;
    while i < file.len() {
        // At next free space.
        if let Entry::Free(_) = file[i] {
            // First ignore any trailing free space.
            while let Some(&Entry::Free(_)) = file.last() {
                file.pop();
            }
            // Move last block to the free space.
            if i < file.len() - 1 {
                file[i] = file.pop().unwrap();
            }
        }
        i += 1;
    }
    // "Checksum", sum file index * block position for every block.
    file.iter()
        .enumerate()
        .map(|(i, n)| i * n.get_id().unwrap())
        .sum::<usize>()
}

/// (Solved, 1h) Instead of moving one block at the time, keep entire files in contiguous space.
/// Had to reverse operations, since the order of finding file to move/free space to fill matters.
fn part2(inputs: &str) -> usize {
    // Translate input
    let mut space = inputs
        .trim()
        .char_indices()
        .fold(Vec::new(), |mut space, (idx, e)| {
            let size = e.to_digit(10).unwrap() as usize;
            if idx % 2 == 0 {
                space.push(Entry::Alloc(idx / 2, size));
            } else {
                space.push(Entry::Free(size));
            }
            space
        });
    // println!("Fragmented {:?}", space);
    // Try to move backmost file forward.
    let mut i = space.len() - 1;
    while i > 0 {
        if let Entry::Alloc(idx, size) = space[i] {
            // Find first sufficiently large space front front.
            for j in 1..i {
                if let Entry::Free(mut free_size) = space[j] {
                    if free_size >= size {
                        // Reduce size of free block.
                        free_size -= size;
                        space[j] = Entry::Free(free_size);
                        // Remove file front original position.
                        space[i] = Entry::Free(size);
                        // Insert file in front.
                        space.insert(j, Entry::Alloc(idx, size));
                        break;
                    }
                }
            }
        }
        i -= 1;
    }
    // println!("After defragmentation {:?}", space);

    // "Checksum", sum file index * block position for every block.
    space
        .iter()
        .fold((0, 0usize), |(pos, sum), entry| match entry {
            Entry::Free(size) => (pos + size, sum),
            Entry::Alloc(idx, size) => (
                pos + size,
                sum + (pos..(pos + size)).map(|p| p * idx).sum::<usize>(),
            ),
        })
        .1
}

#[test]
fn test_2024_day9_part1() {
    assert_eq!(part1("2333133121414131402"), 1928);
}

#[test]
fn test_2024_day9_part2() {
    assert_eq!(part2("2333133121414131402"), 2858);
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
