use std::io::{self, BufRead, BufReader};
use regex::Regex;

fn solve1(s: &String) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(s).map(|caps| {
        let (_, [a, b]) = caps.extract();
        assert_eq!(caps.len(), 3);
        a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap()
    }).reduce(|a,b| a+b).unwrap()
}

fn solve2(active: &mut bool, s: &String) -> i32 {
    let re = Regex::new(r"(?:do\(\)|don\'t\(\)|mul\((\d+),(\d+)\))").unwrap();
    re.captures_iter(s).map(|caps| {
        let m = caps.get(0).unwrap().as_str();
        match m {
            "do()"=> {
                *active = true;
                0
            },
            "don't()"=> {
                *active = false;
                0
            },
            _ => {
                assert_eq!(caps.len(), 3);
                if *active {
                    caps.get(1).unwrap().as_str().parse::<i32>().unwrap() *
                    caps.get(2).unwrap().as_str().parse::<i32>().unwrap()
                } else { 0 }
            }
        }
    }).reduce(|a,b| a+b).unwrap()
}

fn main() {
    let lines = BufReader::new(io::stdin()).lines();
    let (mut ret1, mut ret2) = (0, 0);
    let mut active = true;
    for line in lines {
        let l = line.unwrap();
        ret1 += solve1(&l);
        ret2 += solve2(&mut active, &l);
    }
    println!("ans1 = {}", ret1);
    println!("ans2 = {}", ret2);
}
