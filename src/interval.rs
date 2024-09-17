use core::f64;

pub struct Interval {
    pub max: f64,
    pub min: f64,
}

impl Interval {
    pub fn empty() -> Self {
        Interval {
            max: -f64::INFINITY,
            min: f64::INFINITY,
        }
    }

    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        x >= self.min && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        x > self.min && x < self.max
    }
}
