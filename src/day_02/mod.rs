use std::fs;

pub fn main() {
    let contents =
        fs::read_to_string("inputs/day_02.txt").expect("Something went wrong reading the file");
    part_02(contents)
}

fn part_01(input: String) {
    let lines = input
        .lines()
        .map(|line| {
            let levels = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let sign = (levels.get(0).unwrap() - levels.last().unwrap()).signum();
            if sign == 0 {
                return false;
            }

            return levels
                .iter()
                .cloned()
                .zip(levels.iter().skip(1).cloned())
                .all(|(l, r)| {
                    let diff = sign * (l - r);
                    diff > 0 && diff <= 3
                });
        })
        .map(|x| if x { 1 } else { 0 });

    println!("Day 2 Part 1: {:?}", lines.sum::<i32>());
}

fn part_02(input: String) {
    let lines = input.lines().map(|line| {
        let levels = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let sign = (levels.get(0).unwrap() - levels.last().unwrap()).signum();
        if sign == 0 {
            return 0;
        }

        let test = |levels: Vec<i32>| -> bool {
            levels
                .iter()
                .cloned()
                .zip(levels.iter().skip(1).cloned())
                .all(|(l, r)| {
                    let diff = sign * (l - r);
                    diff > 0 && diff <= 3
                })
        };

        return if test(levels.clone()) {
            1
        } else {
            let mut passed = false;
            for i in 0..levels.len() {
                let mut index_removed = levels.clone();
                index_removed.remove(i);
                let index_passed = test(index_removed);
                if index_passed {
                    passed = true;
                }
            }

            if passed { 1 } else { 0 }
        }
    });

    println!("{:?}", lines.sum::<i32>());
}
