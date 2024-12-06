use regex::Regex;
use crate::utils;

pub fn main() {
    let input = utils::read_input("day_03");
    part_01(input.as_str())
}

fn part_01(input: &str) {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let results = re.captures_iter(input).map(|exp| {
        let string_expression = String::from(exp.get(0).unwrap().as_str());
        let sanitised_expression = string_expression.strip_prefix("mul(").unwrap().strip_suffix(")").unwrap();
        let multiplicands = sanitised_expression.split(",")
            .map(|m| { m.parse::<i32>().expect("Unable to parse number") })
            .collect::<Vec<i32>>();
        return multiplicands[0]*multiplicands[1]
    });
    
    println!("input: {}", results.sum::<i32>());
}