use std::collections::HashMap;
use crate::utils;
use crate::utils::char_grid::CharGrid;
use crate::utils::ivector2::IVector2;

pub fn main() {

    let sample_input= "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    let file_input = utils::read_input("day_08");
    part_02(file_input.as_str());
}

fn part_01(input: &str) {
    let map = get_antenna_map(input);
    
    let grid = CharGrid::from(input);
    let mut all_antinodes: Vec<IVector2> = vec![];
    map.iter().for_each(|(c, list)| {
        let mut antinodes = get_antinodes(list).clone();
        all_antinodes.append(&mut antinodes);
    });


    let grid_string = grid.as_transformed_string(|c, point| {
        if all_antinodes.iter().find(|v| v.equals(point)).is_some() {
            '#'
        } else {
            c
        }
    });
    
    println!("grid {}",grid_string);
    
    println!("count: {}", grid_string.chars().filter(|c| { *c == '#'}).count())
}

fn part_02(input: &str) {
    let map = get_antenna_map(input);

    let grid = CharGrid::from(input);
    
    let mut all_harmonics: Vec<IVector2> = vec![];
    map.iter().for_each(|(c, list)| {
        let mut harmonic = get_harmonics(list, grid.dimensions()).clone();
        all_harmonics.append(&mut harmonic);
    });


    let grid_string = grid.as_transformed_string(|c, point| {
        if all_harmonics.iter().find(|v| v.equals(point)).is_some() {
            '#'
        } else if c != '.' {
            '#'
        } else { 
            c
        }
    });

    println!("grid {}",grid_string);

    println!("count: {}", grid_string.chars().filter(|c| { *c == '#'}).count())
}

fn get_antenna_map(input: &str) -> HashMap<char, Vec<IVector2>> {
    let mut map = HashMap::<char, Vec<IVector2>>::new();
    input.lines().enumerate().for_each(|(y, line)| {
        for x in 0..line.len() {
            let c = line.chars().nth(x).unwrap();
            if c == '.' {
                continue;
            }

            if map.get(&c).is_some() {
                map.get_mut(&c).unwrap().push(IVector2::new_usize(x, y));
            } else {
                map.insert(c, vec![IVector2::new_usize(x, y)]);
            }
        }
    });

    return map
}

fn get_antinodes(nodes: &Vec<IVector2>) -> Vec<IVector2> {
    let mut antinodes: Vec<IVector2> = vec!();
    let mut nodes_mut = nodes.clone();

    let mut vec_to_compare_opt: Option<IVector2> = Some(IVector2::new(0, 0));
    loop {
        vec_to_compare_opt = nodes_mut.pop();
        
        if vec_to_compare_opt.is_none() { 
            break;
        }
        
        let vec_to_compare = vec_to_compare_opt.unwrap();

        for node in &nodes_mut {
            let diff = vec_to_compare.subtract(node.clone());
            let antinode1 = vec_to_compare.plus(diff);
            let antinode2 = node.subtract(diff);
            
            antinodes.push(antinode1);
            antinodes.push(antinode2);
        }
    }
    
    
    return antinodes
    
}

fn get_harmonics(nodes: &Vec<IVector2>, grid_dimensions: IVector2) -> Vec<IVector2> {
    let mut harmonics: Vec<IVector2> = vec!();
    let mut nodes_mut = nodes.clone();

    let mut vec_to_compare_opt: Option<IVector2> = Some(IVector2::new(0, 0));
    loop {
        vec_to_compare_opt = nodes_mut.pop();

        if vec_to_compare_opt.is_none() {
            break;
        }

        let vec_to_compare = vec_to_compare_opt.unwrap();

        for node in &nodes_mut {
            let diff = vec_to_compare.subtract(node.clone());
            let mut harmonic = vec_to_compare;
            loop {
                harmonic = harmonic.plus(diff);
                if !harmonic.is_in_rect(0, 0, grid_dimensions.x, grid_dimensions.y) { 
                    break
                }
                harmonics.push(harmonic);
            }

            harmonic = node.clone();
            loop {
                harmonic = harmonic.subtract(diff);
                if !harmonic.is_in_rect(0, 0, grid_dimensions.x, grid_dimensions.y) {
                    break
                }
                harmonics.push(harmonic); 
            }
        }
    }


    return harmonics

}