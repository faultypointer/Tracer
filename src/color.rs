use std::io::{self, Write};

use crate::{interval::Interval, rtweeknd::INTENSITY_INTERVAL, vector::Vector};

pub type Color = Vector;

pub fn write_color<W: Write>(out: &mut W, pixel_color: Color) -> io::Result<()> {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    // Translate the [0,1] component values to the byte range [0,255].
    let rbyte = (256.0 * INTENSITY_INTERVAL.clamp(r)) as i32;
    let gbyte = (256.0 * INTENSITY_INTERVAL.clamp(g)) as i32;
    let bbyte = (256.0 * INTENSITY_INTERVAL.clamp(b)) as i32;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    f64::sqrt(linear_component)
}
