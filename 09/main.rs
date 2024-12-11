use std::collections::BTreeSet;
use std::io::{self,BufRead,BufReader};

fn to_digits(s: String) -> Vec<i64> {
    s.as_bytes().into_iter().map(|b| *b as i64 - '0' as i64).collect()
}

fn solve1(s: &Vec<i64>) -> i64 {
    let mut v = Vec::new();
    let n = s.len();
    for i in 0..n {
        let id = if i%2==1 { -1i64 } else { (i/2) as i64};
        for _ in 0..s[i] { v.push(id); }
    }
    let mut left=0; while v[left]!=-1 { left+=1; }
    let mut right = v.len()-1; while v[right]==-1 { right-=1; }
    while left<right {
        let tmp = v[left];
        v[left] = v[right];
        v[right] = tmp;
        left+=1; right-=1;
        while v[left]!=-1 && left<v.len() { left+=1; }
        while v[right]==-1  { right-=1; }
    }
    v.into_iter().enumerate()
        .map(|(i,id)| if id==-1 { 0i64 } else { (i as i64)*id}).reduce(|a,b|a+b).unwrap()
}

fn solve2(s: &Vec<i64>) -> i64 {
    let n = s.len();
    let mut empties = BTreeSet::new();
    let mut positions = Vec::new();
    let mut pos = 0;
    for i in 0..s.len() {
        if i%2==1 { empties.insert((pos, s[i])); }
        positions.push(pos);
        pos += s[i];
    }
    let mut files: BTreeSet<(i64,i64,i64)> = BTreeSet::new();
    for i in (0..n).rev() { if i%2==0 {
        let filelen = s[i];
        let mut found = None;
        for (pos,len) in &empties {
            if *pos<positions[i] && *len>=filelen {
                found = Some((*pos,*len));
                break;
            }
        }
        if let Some((pos,len)) = found {
            files.insert((pos, (i/2) as i64, filelen));
            empties.remove(&(pos,len));
            let left = len-filelen;
            if left>0 { empties.insert((pos+filelen, left)); }
        } else {
            files.insert((positions[i], (i/2) as i64, filelen));
        }
    }}
    let mut v = vec![-1i64; pos as usize];
    for (pos,id,len) in files {
        for i in pos..pos+len { v[i as usize] = id as i64; }
    }
    let mut ret = 0;
    for i in 0..v.len() { if v[i]!=-1 {
        ret += (i as i64) * v[i];
    }}
    ret
}

fn main() {
    let v: Vec<i64> = to_digits(BufReader::new(io::stdin()).lines().next().unwrap().unwrap());
    println!("ans1 = {}", solve1(&v));
    println!("ans2 = {}", solve2(&v));
}
