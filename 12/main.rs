use std::io::{self, BufRead, BufReader};
use std::collections::HashSet;

type Grid = Vec<Vec<char>>;
fn to_chars(s: String) -> Vec<char> {
    s.as_bytes().into_iter().map(|b| *b as char).collect()
}

type Pt = (i32,i32);
type Ptd = (i32,i32,usize);

fn valid(i: i32, j: i32, g: &Grid) -> bool {
    return i>=0 && j>=0 && i<g.len() as i32 && j<g[0].len() as i32;
}
const DX: [i32; 4] = [-1,0,1,0];
const DY: [i32; 4] = [0,1,0,-1];

fn dfs(i: i32, j: i32, g: &Grid, vis: &mut HashSet<Pt>, sides: &mut HashSet<Ptd>) -> (i32,i32) {
    vis.insert((i,j));
    let (mut o, mut s) = (0, 1);
    for d in 0..4 {
        let (ii, jj) = (i+DX[d], j+DY[d]);
        if !valid(ii,jj,g) { o+=1; sides.insert((ii,jj,d)); continue; }
        if g[i as usize][j as usize]!=g[ii as usize][jj as usize] {
            o += 1;
            sides.insert((ii,jj,d));
            continue;
        }
        if vis.contains(&(ii,jj)) { continue; }
        let (no, ns) = dfs(ii,jj,g,vis,sides);
        o += no; s += ns;
    }
    (o,s)
}

fn calc(sides: &HashSet<Ptd>) -> i32 {
    let mut ret = 0i32;
    let mut vis = HashSet::new();
    for ptd in sides {
        let (i,j,sd) = *ptd;
        if !vis.contains(&(i,j,sd)) {
        ret+=1;
        vis.insert((i,j,sd));
        for d in 0..4 { if d%2!=sd%2 {
            let (mut ii, mut jj) = (i,j);
            while sides.contains(&(ii+DX[d],jj+DY[d], sd)) {
                ii+=DX[d]; jj+=DY[d];
                vis.insert((ii,jj,sd));
            }
        }}
    }}
    ret
}

fn solve(g: &Grid) -> (i32,i32) {
    let mut vis = HashSet::new();
    let (mut ret1, mut ret2) = (0,0);
    for i in 0..g.len() {
        for j in 0..g.len() {
            let pos = (i as i32,j as i32);
            if vis.contains(&pos) { continue }
            let mut sides = HashSet::new();
            let (s,o) = dfs(pos.0, pos.1, g, &mut vis, &mut sides);
            ret1 += s * o;
            ret2 += calc(&sides) * o;
        }
    }
    (ret1, ret2)
}

fn main() {
    let grid: Grid = BufReader::new(io::stdin()).lines()
        .map(|s| to_chars(s.unwrap())).collect();
    let (ret1,ret2) = solve(&grid);
    println!("ans1 = {ret1}");
    println!("ans2 = {ret2}");
}
