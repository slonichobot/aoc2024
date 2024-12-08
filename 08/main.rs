use std::io::{self,BufRead,BufReader};
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;

type Pt = (i32,i32);

fn to_chars(s: String) -> Vec<char> {
    s.as_bytes().into_iter().map(|b| *b as char).collect()
}

fn solve1(antennas: &HashMap<char, Vec<Pt>>, bd: &Vec<Vec<char>>) -> usize {
    let valid = |pt: Pt| { pt.0 >= 0 && pt.1 >=0 && pt.0 < bd.len() as i32 && pt.1 < bd[0].len() as i32 };
    let mut antinodes: HashSet<Pt> = HashSet::new();
    for coords in antennas.values() {
        for i in 0..coords.len() { for j in (i+1)..coords.len() {
            let a = coords[i];
            let b = coords[j];
            let diff = (a.0-b.0, a.1-b.1);
            for (pt,mul) in vec![(a,1), (b,-1)] {
                let cur = (pt.0 + mul * diff.0, pt.1 + mul * diff.1);
                if valid(cur) {
                    antinodes.insert(cur);
                }
            }
        }}
    }
    antinodes.len()
}

fn solve2(antennas: &HashMap<char, Vec<Pt>>, bd: &Vec<Vec<char>>) -> usize {
    let valid = |pt: Pt| { pt.0 >= 0 && pt.1 >=0 && pt.0 < bd.len() as i32 && pt.1 < bd[0].len() as i32 };
    let mut antinodes: HashSet<Pt> = HashSet::new();
    for coords in antennas.values() {
        for i in 0..coords.len() { for j in (i+1)..coords.len() {
            let a = coords[i];
            let b = coords[j];
            let diff = (a.0-b.0, a.1-b.1);
            for (pt,mul) in vec![(a,1),(b,-1)] {
                let mut cur = pt;
                while valid(cur) {
                    antinodes.insert(cur);
                    cur.0 += mul * diff.0;
                    cur.1 += mul * diff.1;
                }
            }
        }}
    }
    antinodes.len()
}

fn main() {
    let bd: Vec<Vec<char>> = BufReader::new(io::stdin()).lines().map(|s| to_chars(s.unwrap())).collect();
    let mut antennas: HashMap<char, Vec<Pt>> = HashMap::new();
    for i in 0..bd.len() { for j in 0..bd[0].len() { if bd[i][j]!='.' {
        let c = bd[i][j];
        let p = (i as i32, j as i32);
        match antennas.entry(c) {
            Entry::Vacant(e) => { e.insert(vec![p]); },
            Entry::Occupied(mut e) => { e.get_mut().push(p); }
    }}}}
    println!("ans1 = {}", solve1(&antennas, &bd));
    println!("ans2 = {}", solve2(&antennas, &bd));
}

