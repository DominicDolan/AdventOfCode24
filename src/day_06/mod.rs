use crate::utils;
use crate::utils::char_grid::CharGrid;
use crate::utils::grid_cursor::GridCursor;
use crate::utils::ortho_line::OrthoLine;
use std::collections::HashSet;
use crate::utils::directions::turn_right;

const SAMPLE_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

pub fn main() {
    let input = utils::read_input("day_06");
    // let part_01_answer = part_01(input.as_str());
    // assert_eq!(part_01_answer, 5131);

    let part_02_answer = part_02(input.as_str());
    println!("Day 06 Part 2: {}", part_02_answer);
}

fn part_01(input: &str) -> usize {
    let grid = CharGrid::from(input);

    let guard_coord = grid.find_coord('^').expect("No value found for '^'");

    let mut cursor = GridCursor::new(guard_coord);

    let mut direction = (0, -1);
    let mut has_finished = false;

    while !has_finished {
        cursor = cursor.inc_until(direction, |coord, _| {
            let c = grid.get(coord);
            return if c.is_some() && c.unwrap() == '#' {
                false
            } else if c.is_none() {
                has_finished = true;
                false
            } else {
                true
            };
        });
        direction = turn_right(direction);
    }

    cursor.track_len()
}

fn part_02(input: &str) -> usize {
    let grid = CharGrid::from(input);

    let guard_coord = grid.find_coord('^').expect("No value found for '^'");

    let mut direction = (0, -1);
    let mut has_finished = false;
    let mut locations = HashSet::<(i32, i32)>::new();

    let mut cursor = GridCursor::new(guard_coord);
    while !has_finished {
        cursor = cursor.inc_until(direction, |next, grid_cursor: &GridCursor| {
            let c = grid.get(next);
            
            let next_char = grid.get(next).unwrap_or_default();
            let current_char = grid.get(grid_cursor.location()).unwrap_or_default();
            if grid_cursor.coincides_right_moving_line(next, direction) && next_char != '#' && current_char != '^' {
                locations.insert((next.0 + direction.0, next.1 + direction.1));
            }
            
            return if c.is_some() && c.unwrap() == '#' {
                false
            } else if c.is_none() {
                has_finished = true;
                false
            } else {
                true
            };
        });

        direction = turn_right(direction);
    }
    print_grid(&grid, &cursor, &locations);
    locations.len()
}

fn print_grid(grid: &CharGrid, cursor: &GridCursor, locations: &HashSet<(i32, i32)>) {
    println!(
        "\n\n{}",
        grid.as_string(|c, coord| {
            return if locations.contains(&coord) {
                'O'
            } else if cursor.has_passed(coord) {
                'X'
            } else {
                c
            };
        })
    );
}

