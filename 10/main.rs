use std::io::{self, BufRead, BufReader};
use std::collections::HashSet;

fn to_digits(s: String) -> Vec<i32> {
    s.as_bytes().into_iter().map(|b| *b as i32 - '0' as i32).collect()
}

fn valid(i: i32, j: i32, grid: &Grid) -> bool {
    return i>=0 && j>=0 && i<grid.len() as i32 && j<grid[0].len() as i32;
}
const DX: [i32; 4] = [-1,0,1,0];
const DY: [i32; 4] = [0,1,0,-1];

type Grid = Vec<Vec<i32>>;
type Point = (i32,i32);

fn solve<T: Container>(grid: &Grid) -> i32 {
    fn dfs<H: Container>(pt: Point, dst: &mut H, grid: &Grid) {
        let (i,j) = pt;
        if grid[i as usize][j as usize]==9 { dst.add((i,j)); return }
        for d in 0..4 {
            let ii = i + DX[d];
            let jj = j + DY[d];
            if !valid(ii,jj, grid) || grid[ii as usize][jj as usize]!=grid[i as usize][j as usize]+1 {
                continue;
            }
            dfs((ii, jj), dst, grid);
        }
    }
    let mut ret = 0;
    for i in 0..grid.len() { for j in 0..grid[0].len() { if grid[i][j]==0 {
        let mut dst = T::new();
        dfs::<T>((i as i32, j as i32), &mut dst, grid);
        ret += dst.len() as i32;
    }}}
    ret
}

pub trait Container {
    fn new() -> Self; 
    fn add(&mut self, p: Point);
    fn len(& self) -> usize;
}

struct Set { data: HashSet<Point>, }
impl Container for Set {
    fn new() -> Self { Self{ data: HashSet::new() } }
    fn add(&mut self, p: Point) { self.data.insert(p); }
    fn len(& self) -> usize { self.data.len() }
}

struct List { data: Vec<Point> }
impl Container for List {
    fn new() -> Self { Self{ data: Vec::new() } }
    fn add(&mut self, p: Point) { self.data.push(p); }
    fn len(& self) -> usize { self.data.len() }
}

fn main() {
    let grid: Grid = BufReader::new(io::stdin()).lines().map(|s| to_digits(s.unwrap())).collect();

    println!("ans1 = {}", solve::<Set>(&grid));
    println!("ans2 = {}", solve::<List>(&grid));

}

