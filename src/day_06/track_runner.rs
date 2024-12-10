use std::collections::HashMap;
use crate::utils::grid_cursor::GridCursor;
use crate::utils::ivector2::IVector2;

pub struct TrackRunners {
    by_x: HashMap<i32, Vec<usize>>,
    by_y: HashMap<i32, Vec<usize>>,
    all: Vec<GridCursor>
}

impl TrackRunners {
    pub fn new() -> Self {
        TrackRunners {
            by_y: HashMap::new(),
            by_x: HashMap::new(),
            all: Vec::new()
        }
    }


    fn insert_y(&mut self, y: i32, index: usize) {
        if !self.by_y.contains_key(&y) {
            let mut new_list = Vec::new();
            new_list.push(index);
            self.by_y.insert(y, new_list);
        } else {
            self.by_y.get_mut(&y).unwrap().push(index);
        }
    }
    fn insert_x(&mut self, x: i32, index: usize) {
        if !self.by_x.contains_key(&x) {
            let mut new_list = Vec::<usize>::new();
            new_list.push(index);
            self.by_x.insert(x, new_list);
        } else {
            self.by_x.get_mut(&x).unwrap().push(index);
        }
    }

    pub fn insert(&mut self, runner: GridCursor) {
        let position = runner.current_position();
        self.all.push(runner);
        let index = self.all.len() - 1;
        self.insert_x(position.x, index);
        self.insert_y(position.y, index);
    }


    pub fn get_all_by_y(&self, y: i32) -> Vec<&GridCursor> {
        let indices = self.by_y.get(&y);

        if indices.is_some() {
            indices.unwrap().iter().map(|i| &self.all[*i]).collect::<Vec<&GridCursor>>()
        } else {
            Vec::new()
        }

    }

    pub fn get_all_by_x(&self, x: i32) -> Vec<&GridCursor> {
        let indices = self.by_x.get(&x);

        if indices.is_some() {
            indices.unwrap().iter().map(|i| &self.all[*i]).collect::<Vec<&GridCursor>>()
        } else {
            Vec::new()
        }
    }

    pub fn contains_track_with_velocity(&self, coord: IVector2, velocity: IVector2) -> bool {
        if velocity.y == 0 {
            self.get_all_by_x(coord.x).iter().any(|runner| {
                runner.contains_track_with_velocity(coord, velocity)
            })
        } else {
            self.get_all_by_y(coord.y).iter().any(|runner| {
                runner.contains_track_with_velocity(coord, velocity)
            })
        }
    }

    pub fn len(&self) -> usize {
        self.all.len()
    }
}