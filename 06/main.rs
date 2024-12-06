use std::io::{self, BufRead, BufReader};
use std::collections::HashSet;

fn to_chars(s: String) -> Vec<char> {
    s.as_bytes().into_iter().map(|b| *b as char).collect()
}

fn valid(i: i32, j: i32, grid: &Vec<Vec<char>>) -> bool {
    return i>=0 && j>=0 && i<grid.len() as i32 && j<grid[0].len() as i32;
}
const DX: [i32; 4] = [-1,0,1,0];
const DY: [i32; 4] = [0,1,0,-1];

type Grid = Vec<Vec<char>>;
type Point = (i32,i32);

fn get_start(grid: &Grid) -> Point {
    let (mut si, mut sj) = (0,0);
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j]=='^' { si=i as i32; sj=j as i32; }
        }
    }
    (si,sj)
}

fn solve1(start: Point, grid: &mut Grid) -> i32 {
    let (mut ci, mut cj, mut d) = (start.0, start.1, 0);
    while valid(ci, cj, grid) {
        grid[ci as usize][cj as usize] = 'X';
        let ni = ci + DX[d];
        let nj = cj + DY[d];
        if valid(ni, nj, grid) && grid[ni as usize][nj as usize]=='#' { d = (d+1)%4; }
        else { ci = ni; cj = nj; }
    }
    grid.into_iter().fold(0, |acc,l|
        l.into_iter().fold(0, |acc,c| if *c=='X' {acc+1}else{acc})
        + acc)
}

fn solve2(start: Point, grid: &Grid) -> i32 {
    let mut ret = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j]!='X' { continue }
            if (i as i32,j as i32) == start { continue }
            let mut g = grid.clone();
            let mut vis: HashSet<(i32,i32,usize)> = HashSet::new();
            g[i][j] = '#';
            let (mut ci, mut cj, mut d) = (start.0, start.1, 0);
            let mut ok = false;
            while valid(ci, cj, &g) {
                if vis.contains(&(ci,cj,d)) { ok=true; break; }
                vis.insert((ci,cj,d));
                let ni = ci + DX[d];
                let nj = cj + DY[d];
                if valid(ni, nj, &g) && g[ni as usize][nj as usize]=='#' { d = (d+1)%4; }
                else { ci = ni; cj = nj; }
            }
            if ok { ret += 1 }
        }
    }
    ret
}

fn main() {
    let mut grid: Vec<Vec<char>> = BufReader::new(io::stdin()).lines().map(|s| to_chars(s.unwrap())).collect();
    let start = get_start(&grid);
    println!("ans1 = {}", solve1(start, &mut grid));
    println!("ans2 = {}", solve2(start, &grid));
}

