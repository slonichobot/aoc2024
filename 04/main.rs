use std::io::{self, BufRead, BufReader};
use std::collections::HashSet;

const DX: [i32;8] = [1,0,-1,0,1,1,-1,-1];
const DY: [i32;8] = [0,-1,0,1,1,-1,-1,1];

fn to_chars(s: String) -> Vec<char> {
    s.as_bytes().into_iter().map(|b| *b as char).collect()
}

fn main() {
    let v: Vec<Vec<char>> = BufReader::new(io::stdin())
                                .lines().map(|s| to_chars(s.unwrap()))
                                .collect();
    let n = v.len() as i32;
    let m = v[0].len() as i32;

    let valid = |i, j| i>=0 && j>=0 && i<n && j<m;

    let mut ret1 = 0;
    let t = to_chars("XMAS".to_string());
    for i in 0..(n as usize) { for j in 0..(m as usize) { if v[i][j]==t[0]  {
        for d in 0..8 {
            let mut ii=i as i32;
            let mut jj=j as i32;
            let mut ok = true;
            for k in 1..t.len() {
                ii += DX[d];
                jj += DY[d];
                if !valid(ii,jj) || v[ii as usize][jj as usize]!=t[k] {
                    ok = false;
                    break;
                }
            }
            if ok { ret1 += 1; }
        }
    }}}
    println!("ans1 = {ret1}");

    let mut ret2 = 0;
    let ms: HashSet<char> = vec!['M','S'].into_iter().collect();
    for i in 0..(n as usize) { for j in 0..(m as usize) { if v[i][j]=='A' {
        let mut s = vec![HashSet::new(); 2];
        let mut ok = true;
        for d in 4..8 {
            let mut ii=i as i32;
            let mut jj=j as i32;
            ii += DX[d]; jj += DY[d];
            if !valid(ii,jj) { ok=false; break; }
            s[d%2].insert(v[ii as usize][jj as usize]);
        }
        ok &= s[0] == s[1] && s[1]==ms;
        if ok { ret2 += 1; }
    }}}
    println!("ans2 = {ret2}");
}

