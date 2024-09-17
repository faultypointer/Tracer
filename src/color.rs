use std::io::{self, Write};

use crate::vector::Vector;

pub type Color = Vector;

pub fn write_color<W: Write>(out: &mut W, pixel_color: Color) -> io::Result<()> {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Translate the [0,1] component values to the byte range [0,255].
    let rbyte = (255.999 * r) as i32;
    let gbyte = (255.999 * g) as i32;
    let bbyte = (255.999 * b) as i32;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)
}
