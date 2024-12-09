use std::cmp::{max, min};
use std::collections::HashSet;

pub struct GridCursor {
    x: usize,
    y: usize,
    pub track: HashSet<(usize, usize)>,
}

impl GridCursor {

    pub fn inc(&mut self, distance: (i32, i32)) -> bool {
        let old_coord = (self.x as i32, self.y as i32);
        let new_coord = (old_coord.0 + distance.0, old_coord.1 + distance.1);

        for location in min(old_coord.0, new_coord.0)..=max(old_coord.0, new_coord.0) {
            self.track.insert((location as usize, old_coord.1 as usize));
        }

        for location in min(old_coord.1, new_coord.1)..=max(old_coord.1, new_coord.1) {
            self.track.insert((new_coord.0 as usize, location as usize));
        }

        self.x = new_coord.0 as usize;
        self.y = new_coord.1 as usize;

        true
    }

    pub fn inc_until<F>(&mut self, increment: (i32, i32), mut condition: F) -> bool where F: FnMut((i32, i32)) -> bool {
        let mut next = (self.x as i32 + increment.0, self.y as i32 + increment.1);
        while condition(next) {
            let success = self.inc(increment);
            if !success {
                return false
            }

            next.0 += increment.0;
            next.1 += increment.1;
        }

        true
    }

    pub fn new(starting_coord: (usize, usize)) -> GridCursor {
        GridCursor {
            x: starting_coord.0,
            y: starting_coord.1,
            track: HashSet::new(),
        }
    }
}
