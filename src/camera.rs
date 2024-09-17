use crate::color::{self, Color};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweeknd::INF;
use crate::vector::{Point, Vector};

use std::io::{self, Write};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    image_height: u32,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Vector,
    pixel_delta_v: Vector,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32) -> Self {
        let mut temp = Camera {
            aspect_ratio,
            image_width,
            image_height: ((image_width as f64 / aspect_ratio) as u32).max(1),
            center: Point::new(0.0, 0.0, 0.0),
            pixel00_loc: Point::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vector::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vector::new(0.0, 0.0, 0.0),
        };
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / temp.image_height as f64);

        let viewport_u = Vector::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector::new(0.0, -viewport_height, 0.0);

        temp.pixel_delta_u = viewport_u / image_width as f64;
        temp.pixel_delta_v = viewport_v / temp.image_height as f64;

        let viewport_upper_left =
            temp.center - Vector::new(0.0, 0.0, focal_length) - viewport_v / 2.0 - viewport_u / 2.0;
        temp.pixel00_loc = viewport_upper_left + 0.5 * (temp.pixel_delta_u + temp.pixel_delta_v);
        return temp;
    }
    pub fn render<T: Hittable>(&self, world: &T) {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);
            io::stderr().flush().unwrap();
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (i as f64 * self.pixel_delta_u)
                    + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);
                let pixel_color = self.ray_color(r, world);
                color::write_color(&mut io::stdout(), pixel_color).unwrap();
            }
        }
        eprintln!("\nDone!")
    }
    fn ray_color<T: Hittable>(&self, r: Ray, world: &T) -> Color {
        let mut rec = HitRecord::new();
        if world.hit(&r, Interval::new(0.0, INF), &mut rec) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }
        let unit_direction = r.direction().unit();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
