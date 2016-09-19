
use super::point::Point;

#[derive(PartialEq, Eq, Debug)]
pub struct Triangle {
    p0: Point,
    p1: Point,
    p2: Point,
}

impl Triangle {
    pub fn new(p0: Point, p1: Point, p2: Point) -> Triangle {
        // Sort by x-coordinate to make sure the first point is the leftmost and lowest.
        let mut v = [p0, p1, p2];
        v.sort();
        Triangle {
            p0: v[0],
            p1: v[1],
            p2: v[2],
        }
    }

    pub fn range_x(&self) -> (f64, f64) {
        (self.p0.x.to_f64(), self.p2.x.to_f64())
    }

    pub fn range_y(&self) -> (f64, f64) {
        let mut v = [self.p0, self.p1, self.p2];
        v.sort_by_key(|v| v.y);
        (v[0].y.to_f64(), v[2].y.to_f64())
    }

    // Barycentric Technique, check whether point is in triangle, see http://blackpawn.com/texts/pointinpoly/
    pub fn contains(&self, p: Point) -> bool {
        let v0 = self.p2 - self.p0;
        let v1 = self.p1 - self.p0;
        let v2 = p - self.p0;
        let dot00 = v0 * v0;
        let dot01 = v0 * v1;
        let dot02 = v0 * v2;
        let dot11 = v1 * v1;
        let dot12 = v1 * v2;
        let inv_denom = (dot00 * dot11 - dot01 * dot01).recip();
        let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
        let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

        (u > 0.0) && (v > 0.0) && (u + v < 1.0)
    }
}

