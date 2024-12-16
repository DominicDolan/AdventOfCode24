use crate::utils::ivector2::IVector2;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct GridCursor {
    position: IVector2,
    velocity: IVector2,
    track: HashMap<IVector2, HashSet<IVector2>>,
    corners: Vec<IVector2>,
}

impl GridCursor {

    fn insert_track_location(&mut self, location: IVector2) {
        if self.track.contains_key(&location) {
            self.track.get_mut(&location).unwrap().insert(self.velocity);
        } else {
            let mut set = HashSet::new();
            set.insert(self.velocity);
            self.track.insert(location, set);
        }
    }
    pub fn step(&mut self)  {
        let velocity_magnitude = self.velocity.magnitude();
        assert!(velocity_magnitude >= 0.99 && velocity_magnitude <= 1.001);
        let old_coord = self.position;
        let new_coord = self.position.plus(self.velocity);

        self.insert_track_location(old_coord);
  
        self.position = new_coord;
    }

    pub fn step_until_some<F, R>(&mut self, condition: F) -> R where F: Fn(IVector2) -> Option<R> {
        let mut next = self.position.plus(self.velocity);
        let mut result = condition(next);
        while result.is_none() {
            self.step();

            next = next.plus(self.velocity);
            
            result = condition(next);
        }
        
        result.unwrap()
    }
    
    pub fn set_velocity(&mut self, velocity: IVector2) {
        if velocity != self.velocity { 
            self.corners.push(self.position);
        }
        self.velocity = velocity;
    }
    
    pub fn is_corner(&self, coord: IVector2) -> bool {
        self.corners.contains(&coord)
    }
    
    pub fn has_passed(&self, coord: IVector2) -> bool {
        self.track.contains_key(&coord)
    }

    pub fn track_len(&self) -> usize {
        self.track.len()
    }


    pub fn contains_track(&self, coord: IVector2)-> bool {
        self.track.contains_key(&coord)
    }
    
    pub fn contains_track_with_velocity(&self, coord: IVector2, velocity: IVector2) -> bool {
        self.track.contains_key(&coord) && self.track.get(&coord).unwrap().contains(&velocity)
    }
    
    pub fn current_position(&self) -> IVector2 {
        self.position.clone()
    }
    
    pub fn current_velocity(&self) -> IVector2 {
        self.velocity
    }
    
    pub fn new(starting_coord: IVector2) -> GridCursor {
        let mut corners = Vec::<IVector2>::new();
        corners.push(starting_coord);
        
        GridCursor {
            position: starting_coord,
            velocity: IVector2::default(),
            track: HashMap::new(),
            corners,
        }
    }
}
