
pub fn normal(increment: (i32, i32)) -> (f32, f32) {
    let x = increment.0 as f32;
    let y = increment.1 as f32;
    let magnitude = (x * x + y * y).sqrt();
    (x/magnitude, y/magnitude)
}

pub fn rounded_normal(increment: (i32, i32)) -> (i32, i32) {
    let normal = normal(increment);

    (normal.0.round() as i32, normal.1.round() as i32)
}
