use std::ops::{Sub, Mul, Div, Add};

pub(crate) struct Point {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, point: &Point) -> Point {
        Point {
            x: self.x - point.x,
            y: self.y - point.y,
        }
    }
}

impl Mul<f32> for &Point {
    type Output = Point;

    fn mul(self, number: f32) -> Point {
        Point {
            x: self.x * number,
            y: self.y * number,
        }
    }
}

impl Div<f32> for Point {
    type Output = Point;

    fn div(self, number: f32) -> Point {
        Point {
            x: self.x / number,
            y: self.y / number,
        }
    }
}


impl Add<Point> for &Point {
    type Output = Point;

    fn add(self, point: Point) -> Point {
        Point {
            x: self.x + point.x,
            y: self.y + point.y,
        }
    }
}