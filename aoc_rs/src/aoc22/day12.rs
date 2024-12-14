static DAY: usize = 12;

pub fn input() -> String {
    crate::aoc::input_raw(super::YEAR, DAY)
}

fn bfs(map: &Vec<Vec<i32>>, start: (usize, usize), end: (usize, usize)) -> u32 {
    let width = map[0].len();
    let height = map.len();
    let mut dist = vec![vec![0; width]; height];
    let mut q = vec![start];
    while !q.is_empty() {
        let cur = q.remove(0);
        if cur.0 > 0 {
            let nxt = (cur.0-1, cur.1);
            if map[nxt.1][nxt.0] <= map[cur.1][cur.0]+1 {
                if dist[nxt.1][nxt.0] == 0 || dist[nxt.1][nxt.0] > dist[cur.1][cur.0]+1 {
                    dist[nxt.1][nxt.0] = dist[cur.1][cur.0]+1;
                    q.push(nxt);
                }
            }
        }
        if cur.1 > 0 {
            let nxt = (cur.0, cur.1-1);
            if map[nxt.1][nxt.0] <= map[cur.1][cur.0]+1 {
                if dist[nxt.1][nxt.0] == 0 || dist[nxt.1][nxt.0] > dist[cur.1][cur.0]+1 {
                    dist[nxt.1][nxt.0] = dist[cur.1][cur.0]+1;
                    q.push(nxt);
                }
            }
        }
        if cur.1 < height-1 {
            let nxt = (cur.0, cur.1+1);
            if map[nxt.1][nxt.0] <= map[cur.1][cur.0]+1 {
                if dist[nxt.1][nxt.0] == 0 || dist[nxt.1][nxt.0] > dist[cur.1][cur.0]+1 {
                    dist[nxt.1][nxt.0] = dist[cur.1][cur.0]+1;
                    q.push(nxt);
                }
            }
        }
        if cur.0 < width-1 {
            let nxt = (cur.0+1, cur.1);
            if map[nxt.1][nxt.0] <= map[cur.1][cur.0]+1 {
                if dist[nxt.1][nxt.0] == 0 || dist[nxt.1][nxt.0] > dist[cur.1][cur.0]+1 {
                    dist[nxt.1][nxt.0] = dist[cur.1][cur.0]+1;
                    q.push(nxt);
                }
            }
        }
    }
    if dist[end.1][end.0] > 0 { dist[end.1][end.0] } else { std::u32::MAX }
}

/// A breadth-first search does the job for this one.
pub fn part1(inputs: &str) -> u32 {

    let mut start = (0,0);
    let mut end = (0,0);
    let map = inputs.lines()
    .enumerate()
    .map(|(y, line)| {
        line.trim().chars()
        .enumerate()
        .map(|(x, c)| {
            if c == 'S' {
                start = (x,y);
                'a' as i32
            }
            else if c == 'E' {
                end = (x,y);
                'z' as i32
            }
            else {
                c as i32
            }
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

    bfs(&map, start, end)

}

// Apply the BFS on all starting positions, select the minimal value.
pub fn part2(inputs: &str) -> u32 {

    let mut start = Vec::new();
    let mut end = (0,0);
    let map = inputs.lines()
    .enumerate()
    .map(|(y, line)| {
        line.trim().chars()
        .enumerate()
        .map(|(x, c)| {
            if c == 'S' || c == 'a' {
                start.push((x,y));
                'a' as i32
            }
            else if c == 'E' {
                end = (x,y);
                'z' as i32
            }
            else {
                c as i32
            }
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();
    start.iter().map(|start| bfs(&map, *start, end)).min().unwrap()
}

#[test]
fn test_day12_part1() {
    let inputs = "Sabqponm
    abcryxxl
    accszExk
    acctuvwj
    abdefghi";
    assert_eq!(part1(inputs), 31);
}

#[test]
fn test_day12_part2() {
    let inputs = "Sabqponm
    abcryxxl
    accszExk
    acctuvwj
    abdefghi";
    assert_eq!(part2(inputs), 29);
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


