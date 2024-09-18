use rand::Rng;

use crate::interval::Interval;

pub const INF: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

pub const INTENSITY_INTERVAL: Interval = Interval::intensity_interval();

pub fn deg2rad(degree: f64) -> f64 {
    degree * PI / 180.0
}

pub fn random() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_in_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
