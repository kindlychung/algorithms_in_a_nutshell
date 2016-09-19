use std::ops::{Add, Sub, Mul};

use super::definite_num::DefinitelyANumber;


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub x: DefinitelyANumber,
    pub y: DefinitelyANumber,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point {
            x: DefinitelyANumber::new(x).expect("X coordinate cannot be NaN!"),
            y: DefinitelyANumber::new(y).expect("Y coordinate cannot be NaN!"),
        }
    }

    // Euclidean distance
    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).to_f64().powi(2) + (self.y - other.y).to_f64().powi(2)).sqrt()
    }

    // Draw a horizontal line through this point, connect this point with the other, and measure the angle between these two lines.
    pub fn angle(&self, other: &Point) -> f64 {
        if self == other {
            0.0
        } else {
            (other.y - self.y).to_f64().atan2((other.x - self.x).to_f64())
        }
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
// dot product
impl Mul for Point {
    type Output = f64;
    fn mul(self, rhs: Point) -> f64 {
        (self.x * rhs.x + self.y * rhs.y).to_f64()
    }
}

