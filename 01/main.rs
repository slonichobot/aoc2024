use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

fn solve1(a: &mut Vec<i32>, b: &mut Vec<i32>) -> i32 {
    a.sort();
    b.sort();
    a.iter().zip(b.iter())
            .map(|(a,b)| (a-b).abs())
            .reduce(|a,b| a+b)
            .unwrap()
}

fn solve2(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    let mut num: HashMap<i32,i32> = HashMap::new();
    for i in b {
        num.insert(
            i.clone(),
            num.get(i).unwrap_or(&0)+1);
    }
    a.iter().map(|a| a * num.get(a).unwrap_or(&0))
            .reduce(|a,b| a+b)
            .unwrap()
}

fn main() {
    let lines = BufReader::new(io::stdin()).lines();
    let (mut a, mut b) = (vec![], vec![]);
    for line in lines {
        let values = line
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i32>())
            .collect::<Vec<_>>();
        assert!(values.len() == 2);
        a.push(values[0].clone().unwrap());
        b.push(values[1].clone().unwrap());
    }
    println!("ans1 = {}", solve1(&mut a,&mut b));
    println!("ans2 = {}", solve2(&a,&b));
}
