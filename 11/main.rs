use std::io::{self,BufRead,BufReader};
use std::collections::HashMap;


fn dp(stone: u64, steps: u64, cache: &mut HashMap<(u64,u64),u64>) -> u64 {
    if cache.contains_key(&(stone,steps)) {
        return cache[&(stone,steps)];
    }
    let mut s = stone.to_string();
    let ret = if steps==0  { 1 }
        else if stone==0 { dp(1, steps-1, cache) }
        else if (s.len()%2)==0 {
            let t = s.split_off(s.len() / 2);
            let (a,b) = (s.parse::<u64>().unwrap(), t.parse::<u64>().unwrap());
            dp(a, steps-1, cache) + dp(b, steps-1, cache)
        } else {
            dp(stone*2024, steps-1, cache)
        };
    cache.insert((stone,steps), ret);
    ret
}

fn solve(v: &Vec<u64>, steps: u64) -> u64 {
    let mut cache = HashMap::new();
    v.into_iter().map(|stone| dp(*stone, steps, &mut cache)).reduce(|a,b| a+b).unwrap()
}

fn main() {
    let v: Vec<u64> = BufReader::new(io::stdin()).lines().next().unwrap().unwrap()
        .split(" ").map(|s| s.parse::<u64>().unwrap()).collect();
    println!("ans1 = {}", solve(&v, 25));
    println!("ans2 = {}", solve(&v, 75));
}
