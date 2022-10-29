// Struct to store a point
pub struct Point {
    x: u8,
    y: u8,
}

impl Point {
    pub fn new(x: u8, y: u8) -> Point {
        Point { x, y }
    }
}

pub enum Direction { Horizontal, Vertical }

pub struct Battleship {
    points: Vec<Point>
}

impl Battleship {
    pub fn new(x: u8, y: u8, length: u8, direction: Direction) -> Battleship {
        // Creating the vector to store the points the battleship occupies
        let mut points = Vec::new();

        // Adding the points the battleship occupies
        let mut x = x;
        let mut y = y;

        for _i in 0..length {
            points.push(Point::new(x, y));

            match direction {
                Direction::Horizontal => x += 1,
                Direction::Vertical => y += 1,
            }
        }

        Battleship { points } 
    }

    pub fn occupies(&self, x: u8, y: u8) -> bool {
        for point in &self.points {
            if point.x == x && point.y == y {
                return true;
            }
        }
        false
    }
}

