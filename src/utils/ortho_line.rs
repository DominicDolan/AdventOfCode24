
#[derive(Debug)]
pub struct OrthoLine {
    pub direction: (i32, i32),
    pub coord: i32,
}

impl OrthoLine {
    
    pub fn coincides(&self, point: (i32, i32)) -> bool {
        if self.vertical() {
            point.0 == self.coord
        } else {
            point.1 == self.coord
        }
    }
    
    pub fn vertical(&self) -> bool {
        self.direction.0 == 0
    }
    
    pub fn from(point1: (i32, i32), point2: (i32, i32)) -> Self {
        let vertical = if point1.0 == point2.0 { 
            true 
        } else if point1.1 == point2.1 {
            false
        } else { 
            panic!("Cannot create an ortho line with non orthogonal points, point1: {point1:?}, point2: {point2:?}");
        };
        
        let coord = if vertical { point1.0  } else { point1.1 };
        let direction = ((point2.0 - point1.0).signum(), (point2.1 - point1.1).signum());
        OrthoLine { direction, coord }
    }
}