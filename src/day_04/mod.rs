use regex::Regex;
use crate::utils;

pub fn main() {

    let input = utils::read_input("day_04");
    part_1(&input)
//     part_1("MMMSXXMASM
// MSAMXMSMSA
// AMXSXMAAMM
// MSAMASMSMX
// XMASAMXAMM
// XXAMMXXAMA
// SMSMSASXSS
// SAXAMASAAA
// MAMMMXMMMM
// MXMXAXMASX")
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
/*
  1  2  3  4
1 11 21 31 41
2 12 22 32 42
3 13 23 33 43
4 14 24 34 44
 */