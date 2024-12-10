
fn turn_right_test() {
    assert_eq!(turn_right((1, 0)), (0, 1));
    assert_eq!(turn_right((0, 1)), (-1, 0));
    assert_eq!(turn_right((-1, 0)), (0, -1));
    assert_eq!(turn_right((0, -1)), (1, 0));
}

fn turn_left_test() {
    assert_eq!(turn_left((1, 0)), (0, -1));
    assert_eq!(turn_left((0, -1)), (-1, 0));
    assert_eq!(turn_left((-1, 0)), (0, 1));
}

pub fn turn_right(direction: (i32, i32)) -> (i32, i32) {
    (-direction.1, direction.0)
}
pub fn turn_left(direction: (i32, i32)) -> (i32, i32) {
    (direction.1, -direction.0)
}