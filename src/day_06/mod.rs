use std::collections::HashSet;
use std::iter::Map;
use std::path::Display;

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
    part_01(SAMPLE_INPUT);
}

fn part_01(input: &str) {
    let rows = input
        .lines()
        .map(|row| row.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let grid = CharGrid { grid: rows };

    let guard_coord = grid.find_coord('^').expect("No value found for '^'");

    let cursor = Cursor2D::new(guard_coord);
}

struct CharGrid {
    grid: Vec<Vec<char>>,
}

impl CharGrid {
    fn find_coord(&self, character: char) -> Option<(usize, usize)> {
        for (x, row) in self.grid.iter().enumerate() {
            for (y, c) in row.iter().enumerate() {
                if character == *c {
                    return Some((x, y));
                }
            }
        }

        None
    }
}

struct Cursor2D {
    x: usize,
    y: usize,

    track: HashSet<(usize, usize)>,
}

impl Cursor2D {
    fn east(&mut self, distance: i32) -> bool {
        let coord = self.x as i32;
        let new_coord = coord + distance;
        if new_coord < 0 {
            return false;
        }
        for d in coord..=new_coord {
            self.track.insert((d as usize, self.y));
        }
        self.x = new_coord as usize;

        true
    }

    fn west(&mut self, distance: i32) -> bool {
        let coord = self.x as i32;
        let new_coord = coord - distance;
        if new_coord < 0 {
            return false;
        }
        for d in coord..=new_coord {
            self.track.insert((d as usize, self.y));
        }
        self.x = new_coord as usize;

        true
    }

    fn south(&mut self, distance: i32) -> bool {
        let coord = self.x as i32;
        let new_coord = coord + distance;
        if new_coord < 0 {
            return false;
        }
        for d in coord..=new_coord {
            self.track.insert((self.x, d as usize));
        }
        self.x = new_coord as usize;

        true
    }

    fn north(&mut self, distance: i32) -> bool {
        let coord = self.x as i32;
        let new_coord = coord - distance;
        if new_coord < 0 {
            return false;
        }
        for d in coord..=new_coord {
            self.track.insert((self.x, d as usize));
        }
        self.x = new_coord as usize;

        true
    }

    fn new(coord: (usize, usize)) -> Cursor2D {
        Cursor2D {
            x: coord.0,
            y: coord.1,
            track: HashSet::new(),
        }
    }
}
