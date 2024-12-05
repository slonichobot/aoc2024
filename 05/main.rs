use std::io::{self, BufRead, BufReader};
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::iter::FromIterator;

fn toposort(g: &HashMap<u32,Vec<u32>>) -> Vec<u32> {
    fn rec(g: &HashMap<u32,Vec<u32>>, u: u32, path: &mut Vec<u32>, vis: &mut HashSet<u32>) {
        vis.insert(u);
        if g.contains_key(&u) {
            for v in g.get(&u).unwrap() {
                if !vis.contains(&v) { rec(g, *v, path, vis); }
            }
        }
        path.push(u);
    }
    let mut path = Vec::new();
    let mut vis = HashSet::new();
    for u in g.keys() {
        if !vis.contains(u) {
            rec(g, *u, &mut path, &mut vis);
        }
    }
    path.reverse();
    return path;
}

fn check(g: &HashMap<u32,Vec<u32>>, ord: &Vec<u32>) -> bool {
    let mut pos = HashMap::new();
    for (i,u) in ord.into_iter().enumerate() {
        pos.insert(*u,i);
    }
    for (u, ngs) in g {
        for v in ngs {
            if pos.contains_key(u) && pos.contains_key(v) && pos[u] > pos[v] {
                return false;
            }
        }
    }
    return true;
}

fn add_edge(g: &mut HashMap<u32,Vec<u32>>, u: u32, v: u32) {
    match g.entry(u) {
        Entry::Vacant(e) => { e.insert(vec![v]); },
        Entry::Occupied(mut e) => { e.get_mut().push(v); }
    }
}

fn induced_graph(g: &HashMap<u32,Vec<u32>>, verts: &Vec<u32>) -> HashMap<u32,Vec<u32>> {
    let mut ret = HashMap::new();
    let verts: HashSet<&u32> = HashSet::from_iter(verts);
    for (u, ngs) in g {
        if !verts.contains(u) { continue; }
        for v in ngs {
            if !verts.contains(v) { continue; }
            add_edge(&mut ret, *u, *v);
        }
    }
    ret
}


fn main() {
    let mut lines: Vec<String> = BufReader::new(io::stdin()).lines().map(|s| s.unwrap()).collect();
    let split_pos: usize = (&lines).into_iter().position(|s| s=="").unwrap();
    let orderings = lines.split_off(split_pos+1);
    lines.pop();

    let mut g = HashMap::new();
    for edge_str in lines {
        let uv: Vec<u32> = edge_str.split("|").map(|s| s.parse::<u32>().unwrap()).collect();
        let u = uv[0]; let v = uv[1];
        add_edge(&mut g, u, v);
    }

    let mut ret1 = 0;
    let mut ret2 = 0;
    for ord_str in orderings {
        let ord: Vec<u32> = ord_str.split(",").map(|s| s.parse::<u32>().unwrap()).collect();
        let ok = check(&g, &ord);
        if ok {
            ret1 += ord[ord.len()/2];
        } else {
            let fixed_ord = toposort(&induced_graph(&g, &ord));
            ret2 += fixed_ord[fixed_ord.len()/2];
        }
    }
    println!("ans1 = {ret1}");
    println!("ans2 = {ret2}");
}


