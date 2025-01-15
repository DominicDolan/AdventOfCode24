use crate::utils;

pub fn main() {
    let sample_input= "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    
    let sample_input_single = "16: 10 2 4";
    
    let file_input = utils::read_input("day_07");
    part_01(file_input.as_str());
    
}

fn part_01(input: &str) {
    let sum = input.lines().map(|line| {
        let (answer, mut params) = parse_line(line);

        let success = find_operators(answer, &mut params);

        return if success {
            Some(answer)
        } else {
            None
        }
    })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .sum::<i64>();
    
    println!("{}", sum);
}

fn parse_line(line: &str) -> (i64, Vec<i64>) {
    let split = line.split(":").collect::<Vec<&str>>();
    let answer = split[0].parse::<i64>().expect(&format!("Can't parse <{}>, line: {}", split[0], line));

    let params = split[1].trim().split(" ").map(|param| {
        param.parse::<i64>().expect(&format!("Can't parse {}",param))
    }).collect::<Vec<i64>>();
    
    (answer, params)
}

fn find_operators(answer: i64, stack: &mut Vec<i64>) -> bool {
    let opt = stack.pop();
    if opt.is_none() { return false }
    
    let param = opt.unwrap();
    
    if answer == param { 
        return true;
    }
    
    if answer%param == 0 { 
        let new_answer = answer/param;
        let success = find_operators(new_answer, stack);
        if success {
            return true;
        }
    }
    
    let new_answer = answer - param;
    find_operators(new_answer, stack)
   
}