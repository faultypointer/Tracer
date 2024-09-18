use crate::color::{self, Color};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweeknd::{self, INF};
use crate::vector::{Point, Vector};

use std::fmt::Pointer;
use std::io::{self, Write};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub max_depth: u32,
    samples_per_pixel: u32,
    pixel_samples_scale: f64,
    image_height: u32,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Vector,
    pixel_delta_v: Vector,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32) -> Self {
        let mut temp = Camera {
            aspect_ratio,
            image_width,
            max_depth: 10,
            samples_per_pixel,
            pixel_samples_scale: 1.0 / 10.0,
            image_height: ((image_width as f64 / aspect_ratio) as u32).max(1),
            center: Point::new(0.0, 0.0, 0.0),
            pixel00_loc: Point::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vector::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vector::new(0.0, 0.0, 0.0),
        };
        temp.pixel_samples_scale = 1.0 / temp.samples_per_pixel as f64;
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
                let mut color_pixel = Color::zero();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    color_pixel += self.ray_color(r, self.max_depth, world);
                }
                color::write_color(&mut io::stdout(), color_pixel * self.pixel_samples_scale)
                    .unwrap();
            }
        }
        eprintln!("\nDone!")
    }
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);
        Ray::new(self.center, pixel_sample - self.center)
    }
    fn sample_square(&self) -> Vector {
        Vector::new(rtweeknd::random() - 0.5, rtweeknd::random() - 0.5, 0.0)
    }
    fn ray_color<T: Hittable>(&self, r: Ray, depth: u32, world: &T) -> Color {
        if depth <= 0 {
            return Color::zero();
        }
        let mut rec = HitRecord::new();
        if world.hit(&r, Interval::new(0.001, INF), &mut rec) {
            let mut scattered = Ray::new(Point::zero(), Vector::zero());
            let mut attenuation = Color::zero();
            match rec.material.clone() {
                Some(mat) => {
                    if mat.scatter(&r, &rec, &mut attenuation, &mut scattered) {
                        return attenuation * self.ray_color(scattered, depth - 1, world);
                    }
                    return Color::zero();
                }
                None => {}
            }
            let direction = rec.normal + Vector::random_unit_vector();
            return 0.1 * self.ray_color(Ray::new(rec.p, direction), depth - 1, world);
        }
        let unit_direction = r.direction().unit();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
