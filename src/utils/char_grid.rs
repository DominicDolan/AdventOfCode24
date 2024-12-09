
pub struct CharGrid {
    grid: Vec<Vec<char>>,
}

impl CharGrid {
    pub fn find_coord(&self, character: char) -> Option<(usize, usize)> {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if character == *c {
                    return Some((x, y));
                }
            }
        }

        None
    }

    pub fn get(&self, coord: (i32, i32)) -> Option<char> {
        if !self.contains(coord) {
            return None
        }
        Some(self.grid[coord.1 as usize][coord.0 as usize])
    }

    pub fn contains(&self, coord: (i32, i32)) -> bool {
        let dimensions = self.dimensions();
        let contained_horizontal = coord.0 >= 0 && coord.0 < dimensions.0 as i32;
        let contained_vertical = coord.1 >= 0 && coord.1 < dimensions.1 as i32;

        contained_horizontal && contained_vertical
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.grid[0].len(), self.grid.len())
    }

    pub fn as_string<F>(&self, transform_char: F) -> String where F: Fn(char, (i32, i32)) -> char {
        let string = self.grid.iter().enumerate().map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, c)| { transform_char(*c, (j as i32, i as i32)) })
                .collect::<String>()
        }).collect::<Vec<String>>();

        string.join("\n")
    }
    
    pub fn from(string: &str) -> CharGrid {
        let rows = string.lines().collect::<Vec<&str>>();
        
        let row_length = rows[0].len();
        assert!(rows.iter().all(|row| row.len() == row_length), "All rows should be the same size to create a grid");
        
        let grid = rows.iter().map(|row| { row.chars().collect() }).collect();
        
        CharGrid { grid }
    }
}