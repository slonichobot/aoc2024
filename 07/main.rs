use std::io::{self,BufRead,BufReader};

fn solve1(target: u64, nums: &Vec<u64>) -> u64 {
    let mut can = vec![nums[0]];
    for j in &nums[1..] {
        let mut tmp = vec![];
        for i in can {
            tmp.push(i + j);
            tmp.push(i * j);
        }
        can = vec![];
        for i in tmp { if i<=target { can.push(i); } }
    }
    match can.into_iter().find(|x| *x==target) {
        Some(_) => { target }
        None => { 0 }
    }
}

fn solve2(target: u64, nums: &Vec<u64>) -> u64 {
    let mut can = vec![nums[0]];
    for j in &nums[1..] {
        let mut tmp = vec![];
        for i in can {
            tmp.push(i + j);
            tmp.push(i * j);
            let mut tmp_str = i.to_string(); tmp_str.push_str(&j.to_string());
            tmp.push(tmp_str.parse::<u64>().unwrap());
        }
        can = vec![];
        for i in tmp { if i<=target { can.push(i); } }
    }
    match can.into_iter().find(|x| *x==target) {
        Some(_) => { target }
        None => { 0 }
    }
}

fn parse(ln: String) -> (u64, Vec<u64>) {
    let split: Vec<&str> = ln.split(": ").collect();
    let target = split[0].parse::<u64>().unwrap();
    let nums: Vec<u64> = split[1].split(" ").map(|s| s.parse::<u64>().unwrap()).collect();
    (target, nums)
}

fn main() {
    let lines: Vec<(u64,Vec<u64>)> = BufReader::new(io::stdin()).lines().map(|s|parse(s.unwrap())).collect();
    let (mut ret1, mut ret2) = (0,0);
    for (target, nums) in lines {
        ret1 += solve1(target, &nums);
        ret2 += solve2(target, &nums);
    }
    println!("ans1 = {ret1}");
    println!("ans2 = {ret2}");
}

