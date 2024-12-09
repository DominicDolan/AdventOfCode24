use crate::utils;
use crate::utils::char_grid::CharGrid;
use crate::utils::grid_cursor::GridCursor;

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
    turn_right_test();
    turn_left_test();
    
    let input = utils::read_input("day_06");
    let track_length = part_01(input.as_str());
    println!("Day 06 Part 1: {}", track_length);
}

fn part_01(input: &str) -> usize {
    let grid = CharGrid::from(input);

    let guard_coord = grid.find_coord('^').expect("No value found for '^'");

    let mut cursor = GridCursor::new(guard_coord);
    
    let mut direction = (0, -1);
    let mut has_finished = false;
    
    while !has_finished {
        cursor.inc_until(direction, |coord| {
            let c = grid.get(coord);
            return if c.is_some() && c.unwrap() == '#' {
                false
            } else if c.is_none() {
                has_finished = true;
                false
            } else {
                true
            }
        });
        
        direction = turn_right(direction);
    }
    
    println!("{}", grid.as_string(|c, coord| {
        return if cursor.track.contains(&(coord.0 as usize, coord.1 as usize)) {
            'X'
        } else {
            c
        }
    }));
    
    cursor.track.len()
}


fn turn_right_test() {
    assert_eq!(turn_right((1, 0)), (0, 1));
    assert_eq!(turn_right((0, 1)), (-1, 0));
    assert_eq!(turn_right((-1, 0)), (0, -1));
    assert_eq!(turn_right((0, -1)), (1, 0));
}

fn turn_left_test() {
    assert_eq!(turn_left((1, 0)), (0, -1));
    assert_eq!(turn_left((0, -1)), (-1, 0));
    assert_eq!(turn_left((-1, 0)), (0, 1));
}

fn turn_right(direction: (i32, i32)) -> (i32, i32) {
    (-direction.1, direction.0)
}
fn turn_left(direction: (i32, i32)) -> (i32, i32) {
    (direction.1, -direction.0)
} 
