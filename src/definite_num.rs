use std::cmp::Ordering;
use std::ops::{Add, Sub, Mul};

#[derive(Debug, Copy, Clone)]
pub struct DefinitelyANumber(f64);

impl DefinitelyANumber {
    // TODO: Make this a real error
    pub fn new(v: f64) -> Result<Self, ()> {
        if v.is_nan() {
            Err(())
        } else {
            Ok(DefinitelyANumber(v))
        }
    }

    pub fn to_f64(&self) -> f64 {
        self.0
    }
}

impl Add for DefinitelyANumber {
    type Output = DefinitelyANumber;
    fn add(self, rhs: DefinitelyANumber) -> DefinitelyANumber {
        DefinitelyANumber(self.0 + rhs.0)
    }
}

impl Sub for DefinitelyANumber {
    type Output = DefinitelyANumber;
    fn sub(self, rhs: DefinitelyANumber) -> DefinitelyANumber {
        DefinitelyANumber(self.0 - rhs.0)
    }
}

impl Mul for DefinitelyANumber {
    type Output = DefinitelyANumber;
    fn mul(self, rhs: DefinitelyANumber) -> DefinitelyANumber {
        DefinitelyANumber(self.0 * rhs.0)
    }
}

impl PartialEq for DefinitelyANumber {
    fn eq(&self, other: &DefinitelyANumber) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<f64> for DefinitelyANumber {
    fn eq(&self, other: &f64) -> bool {
        if other.is_nan() {
            return false;
        }
        self.0 == *other
    }
}

impl Eq for DefinitelyANumber {}

impl PartialOrd for DefinitelyANumber {
    fn partial_cmp(&self, other: &DefinitelyANumber) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DefinitelyANumber {
    fn cmp(&self, other: &DefinitelyANumber) -> Ordering {
        self.0.partial_cmp(&other.0).expect("A number that can't be NaN was NaN")
    }
}
