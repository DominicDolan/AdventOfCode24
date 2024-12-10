use crate::utils::ivector2::IVector2;

pub struct CharGrid {
    grid: Vec<Vec<char>>,
}

impl CharGrid {
    pub fn find_coord(&self, character: char) -> Option<IVector2> {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if character == *c {
                    return Some(IVector2::new_usize(x, y));
                }
            }
        }

        None
    }

    pub fn get(&self, coord: IVector2) -> Option<char> {
        if !self.contains(coord) {
            return None
        }
        Some(self.grid[coord.y as usize][coord.x as usize])
    }

    pub fn contains(&self, coord: IVector2) -> bool {
        let dimensions = self.dimensions();
        let contained_horizontal = coord.x >= 0 && coord.x < dimensions.x as i32;
        let contained_vertical = coord.y >= 0 && coord.y < dimensions.y as i32;

        contained_horizontal && contained_vertical
    }

    pub fn dimensions(&self) -> IVector2 {
        IVector2::new_usize(self.grid[0].len(), self.grid.len())
    }

    pub fn as_transformed_string<F>(&self, transform_char: F) -> String where F: Fn(char, IVector2) -> char {
        let string = self.grid.iter().enumerate().map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, c)| { transform_char(*c, IVector2::new(j as i32, i as i32)) })
                .collect::<String>()
        }).collect::<Vec<String>>();

        string.join("\n")
    }
    
    pub fn as_string(&self) -> String {
        self.as_transformed_string(|c, _| c)
    }
    
    pub fn from(string: &str) -> CharGrid {
        let rows = string.lines().collect::<Vec<&str>>();
        
        let row_length = rows[0].len();
        assert!(rows.iter().all(|row| row.len() == row_length), "All rows should be the same size to create a grid");
        
        let grid = rows.iter().map(|row| { row.chars().collect() }).collect();
        
        CharGrid { grid }
    }
}