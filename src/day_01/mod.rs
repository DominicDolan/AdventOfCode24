use crate::utils;

pub fn main() {
    let input = utils::read_input("day_01");
    
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    
    input.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        
        let left_num_str = parts.next().unwrap();
        left.push(left_num_str.parse::<i32>().unwrap());
        
        let right_num_str = parts.next().unwrap();
        right.push(right_num_str.parse::<i32>().unwrap());
    });
    
    left.sort();
    right.sort();
    
    let mut total_diff = 0;
    for i in 0..left.len() {
        let mut diff = (left[i] - right[i]);
        if diff < 0 { 
            diff = -diff;
        }
        total_diff += diff;
    }
    
    println!("Day 1 Part 1: {}", total_diff);
}