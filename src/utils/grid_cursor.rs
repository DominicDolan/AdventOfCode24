use std::cmp::{max, min};
use std::collections::HashSet;
use crate::utils::directions::turn_right;
use crate::utils::ortho_line::OrthoLine;
use crate::utils::vector_math::{normal, rounded_normal};

#[derive(Debug)]
pub struct GridCursor {
    x: usize,
    y: usize,
    track: HashSet<(usize, usize)>,
    previous_inc: (i32, i32),
    corners: Vec<(i32, i32)>,
    lines: Vec<OrthoLine>
}


impl GridCursor {

    pub fn inc(&mut self, increment: (i32, i32)) -> bool {
        let old_coord = (self.x as i32, self.y as i32);
        let new_coord = (old_coord.0 + increment.0, old_coord.1 + increment.1);

        for location in min(old_coord.0, new_coord.0)..=max(old_coord.0, new_coord.0) {
            self.track.insert((location as usize, old_coord.1 as usize));
        }

        for location in min(old_coord.1, new_coord.1)..=max(old_coord.1, new_coord.1) {
            self.track.insert((new_coord.0 as usize, location as usize));
        }

        self.x = new_coord.0 as usize;
        self.y = new_coord.1 as usize;
        
        if rounded_normal(self.previous_inc) != rounded_normal(increment) { 
            self.corners.push(old_coord);
            let old_old_corner = self.get_corner(1);
            if old_old_corner.is_some() { 
                let new_line = OrthoLine::from(old_old_corner.unwrap(), old_coord);
                self.lines.push(new_line)
            }
        }
        
        self.previous_inc = increment;
        

        true
    }

    pub fn inc_until<F>(mut self, increment: (i32, i32), mut condition: F) -> Self where F: FnMut((i32, i32), &Self) -> bool {
        let mut next = (self.x as i32 + increment.0, self.y as i32 + increment.1);
        while condition(next, &self) {
            let success = self.inc(increment);
            if !success {
                return self
            }

            next.0 += increment.0;
            next.1 += increment.1;
        }

        self
    }
    
    pub fn is_corner(&self, coord: (i32, i32)) -> bool {
        self.corners.contains(&coord)
    }
    
    pub fn has_passed(&self, coord: (i32, i32)) -> bool {
        self.track.contains(&(coord.0 as usize, coord.1 as usize))
    }

    pub fn track_len(&self) -> usize {
        self.track.len()
    }
    
    pub fn location(&self) -> (i32, i32) {
        (self.x as i32, self.y as i32)
    }
   
    pub fn coincides_vertical_line(&self, coord: (i32, i32)) -> bool {
        self.lines.iter()
            .filter(|line| { line.vertical() })
            .any(|line| { line.coincides(coord) })
    }

    pub fn coincides_horizontal_line(&self, coord: (i32, i32)) -> bool {
        self.lines.iter()
            .filter(|line| { !line.vertical() })
            .any(|line| { line.coincides(coord) })
    }
    
    pub fn coincides_right_moving_line(&self, coord: (i32, i32), direction: (i32, i32)) -> bool {
        let right_direction = turn_right(direction);
        
        self.lines.iter()
            .filter(|line| { line.direction == right_direction })
            .any(|line| { line.coincides(coord) })
    }
    
    pub fn get_corner(&self, i: usize) -> Option<(i32, i32)> {
        let index = self.corners.len() as i32 - i as i32 - 1;
        if index < 0 || index >= self.corners.len() as i32 { 
            return None
        }
        self.corners.get(self.corners.len() - i - 1).cloned()
    }
    
    pub fn new(starting_coord: (usize, usize)) -> GridCursor {
        let mut corners = Vec::<(i32, i32)>::new();
        corners.push((starting_coord.0 as i32, starting_coord.1 as i32));
        
        GridCursor {
            x: starting_coord.0,
            y: starting_coord.1,
            track: HashSet::new(),
            previous_inc: (0, 0),
            corners,
            lines: Vec::new()
        }
    }
}
