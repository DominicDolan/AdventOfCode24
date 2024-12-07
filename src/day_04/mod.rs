use regex::Regex;
use crate::utils;

const SAMPLE_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
pub fn main() {

    let input = utils::read_input("day_04");
    // part_1(&input);
    part_02(input.as_str());
}

fn part_1(input: &str) {
    let lines = input.lines();
    let columns = lines.clone().into_iter().fold(Vec::<Vec<char>>::new(), |mut acc, row| {
        row.chars().enumerate().for_each(|(i, c)| {
            if i >= acc.len() {
                acc.push(Vec::new());
            }

            acc[i].push(c);
        });
        return acc;
    }).into_iter().map(|col_chars| {
        let col_string = col_chars.iter()
            .map(|c| *c)
            .collect::<String>();

        col_string.clone()
    }).collect::<Vec<String>>();

    let rows = lines.collect::<Vec<&str>>();
    
    let diagonals_left = (0..(columns.len() + rows.len())).into_iter().map(|i| {
        let mut chars = Vec::<char>::new();
        let mut current_col_index = i;
        let mut current_row_index = 0;
        loop {
            let row = rows[current_row_index].as_bytes();
            if current_col_index < row.len() {
                chars.push(char::from(row[current_col_index]))
            }
            
            if current_col_index == 0 || current_row_index == rows.len() - 1 {
                break;
            }
            current_row_index += 1;
            current_col_index -= 1;
        }
        chars.iter().collect::<String>()
    }).collect::<Vec<String>>();
    
    let diagonals_right = (0..(columns.len() + rows.len())).into_iter().map(|i| {
        let mut chars = Vec::<char>::new();
        let mut current_col_index = rows.len() as i32 - i as i32 - 1;
        let mut current_row_index = 0;
        loop {
            let row = rows[current_row_index].as_bytes();
            
            if current_col_index >= 0 && current_col_index < row.len() as i32 {
                chars.push(char::from(row[current_col_index as usize]))
            }
            if current_col_index == row.len() as i32 || current_row_index == rows.len() - 1 {
                break;
            }
            current_col_index += 1;
            current_row_index += 1;
        }
        chars.iter().collect::<String>()
    }).collect::<Vec<String>>();
    

    let re = Regex::new("XMAS|SAMX");
    let re2 = Regex::new("XMASAMX|SAMXMAS");
    let xmas_count_cols: i32 = columns.iter().map(|col| {
        let count1 = re.as_ref().unwrap().captures_iter(col.as_str()).count() as i32;
        let count2 = re2.as_ref().unwrap().captures_iter(col).count() as i32;
        count1 + count2
    }).sum();

    let xmas_count_rows: i32 = rows.iter().map(|row| {
        let count1 = re.as_ref().unwrap().captures_iter(row).count() as i32;
        let count2 = re2.as_ref().unwrap().captures_iter(row).count() as i32;
        count1 + count2
    }).sum();
    
    let xmas_count_diagonals_left: i32 = diagonals_left.iter().map(|diagonal| {
        let count1 = re.as_ref().unwrap().captures_iter(diagonal).count() as i32;
        let count2 = re2.as_ref().unwrap().captures_iter(diagonal).count() as i32;
        count1 + count2
    }).sum();


    let xmas_count_diagonals_right: i32 = diagonals_right.iter().map(|diagonal| {
        let count1 = re.as_ref().unwrap().captures_iter(diagonal).count() as i32;
        let count2 = re2.as_ref().unwrap().captures_iter(diagonal).count() as i32;
        count1 + count2
    }).sum();
    
    println!("{}", xmas_count_rows + xmas_count_cols + xmas_count_diagonals_left + xmas_count_diagonals_right);
}

fn part_02(input: &str) {
    let row_strings = input.lines().collect::<Vec<&str>>();
    let rows = row_strings.into_iter().map(|row| {
        return String::from(row).into_bytes().into_iter().map(|c| char::from(c)).collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>();

    let all_a_locations = find_all(&rows, 'A');
    
    let mut found = 0;
    let row_length = rows.len();
    let col_length = rows[0].len();
    for coord in all_a_locations {
        if coord.0 == 0 || coord.1 == 0 || coord.1 == row_length - 1 || coord.0 == col_length - 1 { 
            continue
        }
        let (before, after) = diagonal_left(&rows, coord);
        
        if !((before == 'S' && after == 'M') || (before == 'M' && after == 'S')) {
            continue
        }
        let (before, after) = diagonal_right(&rows, coord);
        
        if !((before == 'S' && after == 'M') || (before == 'M' && after == 'S')) {
            continue
        }
        found += 1;
    }
    println!("{}", found)
}

fn find_all(haystack: &Vec<Vec<char>>, pin: char) -> Vec<(usize, usize)> {
    return haystack.into_iter().enumerate().map(|(i, row)| {
        row.into_iter().enumerate().filter(|(_j, col)| {
            **col == pin
        }).map(|(j, _col)| {
            (j, i)
        }).collect::<Vec<(usize, usize)>>()
    }).flatten().collect::<Vec<(usize, usize)>>();
}

fn diagonal_left(haystack: &Vec<Vec<char>>, coord: (usize, usize)) -> (char, char) {
    (haystack[coord.1 - 1][coord.0 - 1], haystack[coord.1 + 1][coord.0 + 1])
}

fn diagonal_right(haystack: &Vec<Vec<char>>, coord: (usize, usize)) -> (char, char) {
    (haystack[coord.1 - 1][coord.0 + 1], haystack[coord.1 + 1][coord.0 - 1])
}
