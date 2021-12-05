
#[derive(Copy, Clone, Debug)]
pub struct BingoBoard {
    nums: [[usize; 5]; 5],
    mark: [[bool; 5]; 5],
}

impl BingoBoard {
    /**Sum all unmarked fields */
    pub fn sum_unmarked(&self) -> usize {
        (0..5).map(|m| 
            (0..5).filter(|&n| !self.mark[m][n])
                .map(|n| self.nums[m][n])
                .sum::<usize>())
            .sum::<_>()
    }
    /**Mark the number if it exists and is not already marked */
    pub fn mark_number(&mut self, num: usize) -> bool {
        for m in 0..5 {
            for n in 0..5 {
                if self.nums[m][n] == num {     // Could use Iterator::find
                    let ret = !self.mark[m][n];
                    self.mark[m][n] = true;
                    return ret
                }
            }
        }
        return false
    }
    /**Check if a board has a bingo: All marked in any column or row */
    pub fn check_bingo(&self) -> bool {
        (0..5).any(|i| (0..5).all(|j| self.mark[i][j]) || (0..5).all(|j| self.mark[j][i]))
    }
}


pub fn input() -> (Vec<usize>, Vec<BingoBoard>) {
    use std::io::{prelude::*, BufReader};
    let f = crate::aoc::input_file(4).unwrap();
    let mut f = BufReader::new(f).lines();

    let nums = f.next().unwrap().unwrap();
    let nums = nums.split(",").map(|e| e.parse::<usize>().unwrap()).collect::<Vec<_>>();
    f.next(); // Empty line

    let mut boards = Vec::new();
    let mut b = BingoBoard { nums: [[0; 5]; 5], mark: [[false; 5]; 5] };
    let mut m = 0;

    for line in f {
        let line = line.unwrap();
        if line.trim().is_empty() {
            boards.push(b);
            b = BingoBoard { nums: [[0; 5]; 5], mark: [[false; 5]; 5] };
            m = 0;
        } else {
            for (n, e) in line.split_whitespace().enumerate() {
                b.nums[m][n] = e.parse::<_>().unwrap();
            }
            m += 1;
        }
    }
    (nums, boards)
}

/**Run bingo game naively, returning when a board wins */
pub fn part1(nums: &Vec<usize>, boards: &Vec<BingoBoard>) -> usize {
    let mut boards = boards.clone();

    for &number in nums.iter() {
        for b in boards.iter_mut() {
            b.mark_number(number);
            if b.check_bingo() {
                return b.sum_unmarked() * number
            }
        }
    }
    0
}

/**Run bingo game and set a flag and save latest value whenever a board wins,
 * eventually returning the last winner. 
 * Improvements: 
 * - Should remove winners from game to avoid wasting computation
 * on them.
 * - Could parallelize running game using threads or gpu, saving the round at 
 * which every board won, and select the latest one.
 */
pub fn part2(nums: &Vec<usize>, boards: &Vec<BingoBoard>) -> usize {
    let mut boards = boards.clone();
    let mut winner = vec![false; boards.len()];
    let mut ret = 0;

    for &number in nums.iter() {
        for (i, b) in boards.iter_mut().enumerate() {
            b.mark_number(number);
            if b.check_bingo() && !winner[i] {
                ret = b.sum_unmarked() * number;
                winner[i] = true;
            }
        }
    }
    ret
}


/* TESTS */
#[allow(unused)]
static TEST_NUMS: &'static [usize] = &[7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1];
#[allow(unused)]
static TEST_BOARDS: &'static [BingoBoard] = &[
    BingoBoard { nums: [
        [22, 13, 17, 11,  0],
        [8,  2, 23,  4, 24],
        [21,  9, 14, 16,  7],
        [6, 10,  3, 18,  5],
        [1, 12, 20, 15, 19],
        ],
        mark: [[false; 5]; 5]
    },

    BingoBoard { nums: [
        [3, 15,  0,  2, 22],
        [9, 18, 13, 17,  5],
        [19,  8,  7, 25, 23],
        [20, 11, 10, 24,  4],
        [14, 21, 16, 12,  6],
        ],
        mark: [[false; 5]; 5]
    },

    BingoBoard { nums: [
        [14, 21, 17, 24,  4],
        [10, 16, 15,  9, 19],
        [18,  8, 23, 26, 20],
        [22, 11, 13,  6,  5],
        [2,  0, 12,  3,  7],
        ],
        mark: [[false; 5]; 5]
    },
];

#[test]
fn test_day04_part1() {
    assert_eq!(part1(&Vec::from(TEST_NUMS), &Vec::from(TEST_BOARDS)), 4512);
}

#[test]
fn test_day04_part2() {
    assert_eq!(part2(&Vec::from(TEST_NUMS), &Vec::from(TEST_BOARDS)), 1924);
}

#[test]
fn test_sum_unmarked() {
    let mut board = TEST_BOARDS[2];
    board.mark = [
            [true, true, true, true, true],
            [false, false, false, true, false],
            [false, false, true, false, false],
            [false, true, false, false, true],
            [true, true, false, false, true]
        ];
    assert_eq!(board.sum_unmarked(), 188);
}

#[test]
fn test_mark_number() {
    let mut board = TEST_BOARDS[2];

    assert_eq!(board.mark_number(14), true);
    assert_eq!(board.mark_number(14), false);
    assert_eq!(board.mark[0][0], true);
}

#[allow(unused)]
pub fn run() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . .");
    let (nums, boards) = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 1 part 1: ");
    println!("{}", part1(&nums, &boards));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 1 part 2: ");
    println!("{}", part2(&nums, &boards));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    println!("Total time: {:?}", start.elapsed().unwrap());
}
