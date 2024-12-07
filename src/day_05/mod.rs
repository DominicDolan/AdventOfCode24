use crate::utils;

const SAMPLE_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

pub fn main() {
    let input = utils::read_input("day_05");
    part_01(input.as_str())
}

fn part_01(input: &str) {
    let split = input.split("\n\n").collect::<Vec<&str>>();
    
    let notations = split[0].lines().map(|n| { 
        let mut split_notation = n.split("|");
        (split_notation.next().unwrap(), split_notation.next().unwrap())
    }).collect::<Vec<(&str, &str)>>();
    
    let mut protocols = split[1].lines();
    
    let total = protocols.map( |protocol: &str| {
        let mut pages = protocol.split(",").collect::<Vec<&str>>();
        
        let relevant_notations = notations.clone().into_iter().filter(|&c| {
            pages.iter_mut().find(|p| { p.contains(c.0) || p.contains(c.1) }).is_some()
        }).collect::<Vec<(&str, &str)>>();
        
        let is_sorted = is_sorted_by_notations(&relevant_notations, &pages);
        
        if is_sorted {
            return String::from(pages[(pages.len() - 1)/2])
        } else {
            return String::from("")
        }
    }).map(|protocol| {
        let parsed_protocol = protocol.parse::<i32>();
        if parsed_protocol.is_ok() {
            parsed_protocol.unwrap()
        } else {
            0
        }
    }).sum::<i32>();
    
    println!("Day 05 Part 01: {}", total);
}

fn is_sorted_by_notations(protocols: &Vec<(&str, &str)>, pages: &Vec<&str>) -> bool {
    pages.iter().is_sorted_by(|a, b| {
        let not_found = protocols.iter().find(|(low, high)| {
            *a == high && *b == low
        }).is_none();
        return not_found;
    })
}
