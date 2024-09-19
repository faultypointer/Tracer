use crate::color::{self, Color};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweeknd::{self, INF};
use crate::vector::{Point, Vector};

use std::io::{self, Write};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub max_depth: u32,
    pub vfov: f64,
    pub lookfrom: Point,
    pub lookat: Point,
    pub vup: Vector,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    samples_per_pixel: u32,
    pixel_samples_scale: f64,
    image_height: u32,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Vector,
    pixel_delta_v: Vector,
    // camera frame basis vector
    u: Vector,
    v: Vector,
    w: Vector,
    defocus_disk_u: Vector,
    defocus_disk_v: Vector,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32, vfov: f64) -> Self {
        Camera {
            aspect_ratio,
            image_width,
            max_depth: 10,
            vfov,
            lookfrom: Vector::zero(),
            lookat: Point::new(0.0, 0.0, -1.0),
            vup: Vector::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            samples_per_pixel,
            pixel_samples_scale: 1.0 / 10.0,
            image_height: ((image_width as f64 / aspect_ratio) as u32).max(1),
            center: Point::new(0.0, 0.0, 0.0),
            pixel00_loc: Point::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vector::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vector::new(0.0, 0.0, 0.0),
            u: Vector::zero(),
            v: Vector::zero(),
            w: Vector::zero(),
            defocus_disk_u: Vector::zero(),
            defocus_disk_v: Vector::zero(),
        }
    }
    pub fn initialize(&mut self) {
        self.center = self.lookfrom;

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        let theta = rtweeknd::deg2rad(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // basis vector
        self.w = (self.lookfrom - self.lookat).unit();
        self.u = Vector::cross(self.vup, self.w).unit();
        self.v = Vector::cross(self.w, self.u);

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * (-self.v);

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - self.focus_dist * self.w - viewport_v / 2.0 - viewport_u / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * (rtweeknd::deg2rad(self.defocus_angle / 2.0)).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
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
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        Ray::new(ray_origin, pixel_sample - self.center)
    }
    fn defocus_disk_sample(&self) -> Point {
        let p = Vector::random_in_unit_disk();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
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
            if let Some(mat) = rec.material.clone() {
                // Check if material scatters ray
                if mat.scatter(&r, &rec, &mut attenuation, &mut scattered) {
                    return attenuation * self.ray_color(scattered, depth - 1, world);
                } else {
                    return Color::new(0.0, 0.0, 0.0); // Ray absorbed, return black
                }
            } else {
                // Debugging: If material is missing, log something
                eprintln!("Warning: Object hit but no material found.");
            }
            let direction = rec.normal + Vector::random_unit_vector();
            return 0.7 * self.ray_color(Ray::new(rec.p, direction), depth - 1, world);
        }
        let unit_direction = r.direction().unit();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
