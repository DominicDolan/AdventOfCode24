use crate::utils;

pub fn main() {
    let input = utils::read_input("day_01");
    // part_01(input); 
    part_02(String::from(input));
}

fn part_01(input: String) {
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
        let mut diff = left[i] - right[i];
        if diff < 0 {
            diff = -diff;
        }
        total_diff += diff;
    }

    println!("Day 1 Part 1: {}", total_diff);
}

fn part_02(input: String) {
    let (left, right): (Vec<_>, Vec<_>) = input.lines().map(|line| {
        let pairVec = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        (pairVec[0], pairVec[1])
    }).unzip();

    let sum = left.iter().map(|value| {
        let instances = right.iter().filter(|x| { x.clone() == value}).count() as i32;
        instances * value
    }).sum::<i32>();
    
    println!("Day 1 Part 2: {}", sum);
   
}