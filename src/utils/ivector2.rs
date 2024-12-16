
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct IVector2 {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl IVector2 {
    pub fn new(x: i32, y: i32) -> IVector2 {
        IVector2 { x, y }
    }

    pub fn new_usize(x: usize, y: usize) -> IVector2 {
        IVector2::new(x as i32, y as i32)
    }

    pub fn default() -> IVector2 {
        IVector2 { x: 0, y: 0 }
    }

    pub fn turn_right(self) -> IVector2 {
        IVector2::new(-self.y, self.x)
    }

    pub fn turn_left(self) -> IVector2 {
        IVector2::new(self.y, -self.x)
    }

    pub fn reverse(self) -> IVector2 {
        IVector2::new(-self.x, -self.y)
    }

    pub fn normal_pc(self) -> IVector2 {
        let x = self.x as f32;
        let y = self.y as f32;

        let magnitude = (x * x + y * y).sqrt();
        let x_i32 = ((x*100f32)/magnitude).round() as i32;
        let y_i32 = ((y*100f32)/magnitude).round() as i32;
        IVector2::new(x_i32, y_i32)
    }

    pub fn plus(self, other: IVector2) -> IVector2 {
        IVector2::new(self.x + other.x, self.y + other.y)
    }
    
    pub fn magnitude(self) -> f32 {
        let x = self.x as f32;
        let y = self.y as f32;

        (x * x + y * y).sqrt()
    }

}

