use std::io::{self, BufRead, BufReader};

fn solve1(a: &Vec<i32>) -> bool {
    let d = a[1] - a[0];
    let mut ok = d.abs() >= 1 && d.abs() <= 3;
    for i in 1..a.len() {
        if !ok { break; }
        let curd = a[i]-a[i-1];
        ok &= curd.abs() >= 1 && curd.abs() <= 3;
        ok &= (d>0) == (curd>0);
    }
    ok
}

fn solve2(a: &Vec<i32>) -> bool {
    let mut ok = solve1(a);
    for i in 0..a.len() {
        if ok { break; }
        let mut b = a.clone();
        b.remove(i);
        ok = solve1(&b);
    }
    ok
}

fn main() {
    let lines = BufReader::new(io::stdin()).lines();
    let (mut ret1, mut ret2) = (0, 0);
    for line in lines {
        let values = line
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        if solve1(&values) { ret1 += 1; }
        if solve2(&values) { ret2 += 1; }
    }
    println!("ans1 = {}", ret1);
    println!("ans2 = {}", ret2);
}
