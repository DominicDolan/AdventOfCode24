use regex::Regex;
use crate::utils;

pub fn main() {
    let input = utils::read_input("day_03");
    part_02(input.as_str())
}

fn part_01(input: &str) -> i32 {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let results = re.captures_iter(input).map(|exp| {
        let string_expression = String::from(exp.get(0).unwrap().as_str());
        let sanitised_expression = string_expression.strip_prefix("mul(").unwrap().strip_suffix(")").unwrap();
        let multiplicands = sanitised_expression.split(",")
            .map(|m| { m.parse::<i32>().expect("Unable to parse number") })
            .collect::<Vec<i32>>();
        return multiplicands[0]*multiplicands[1]
    });
    
    return results.sum::<i32>();
}

fn part_02(input: &str) {
    let parts = input.split("do()");
    
    let result = parts.map(|part| {
        let enabled_part = part.split("don't()").into_iter().next().unwrap();
        return part_01(enabled_part);
    });
    
    println!("Day 3 part 2: {}", result.sum::<i32>())
}