// Struct to store a point
pub struct Point {
    x: u8,
    y: u8,
}

impl Point {
    pub fn new(x: u8, y: u8) -> Point {
        Point { x, y }
    }

    pub fn y(&self)  -> u8 {
        self.y.clone()
    }

    pub fn x(&self) -> u8 {
        self.x.clone()
    }
}
