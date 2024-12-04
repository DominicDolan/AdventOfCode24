use std::{fs, num};

pub fn main() {
    let contents = fs::read_to_string("inputs/day_02.txt").expect("Something went wrong reading the file");
    
    let lines = contents.lines().map(|line| {
        let levels = line.split_whitespace()
            .map(|x| {x.parse::<i32>().unwrap()})
            .collect::<Vec<i32>>();
        
        let sign = (levels.get(0).unwrap() - levels.last().unwrap()).signum();
        if sign == 0 { 
            return false
        }
        
        return levels.iter().cloned()
            .zip(levels.iter().skip(1).cloned())
            .all(|(l, r)| {
                let diff = sign*(l - r);
                diff > 0 && diff <= 3
            });
    }).map(|x| { if x { 1 } else { 0 } });
    
    println!("{:?}", lines.sum::<i32>());
}