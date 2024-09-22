mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod rtweeknd;
mod vector;

use std::rc::Rc;

use camera::Camera;
use color::Color;
use hittable::{HittableList, Sphere};
use material::{Dielectric, Lambertian, Material, Metal};
use vector::{Point, Vector};
fn main() {
    let mut world = HittableList::new();

    let ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    world.add(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Some(ground.clone()),
    ));

    for j in -11..11 {
        for i in -11..11 {
            let choose_mat = rtweeknd::random();
            let center = Point::new(
                j as f64 + 0.9 * rtweeknd::random(),
                0.2,
                i as f64 + 0.9 * rtweeknd::random(),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_mat: Rc<dyn Material> = if choose_mat < 0.8 {
                    Rc::new(Lambertian::new(Color::random() * Color::random()))
                } else if choose_mat < 0.95 {
                    Rc::new(Metal::new(
                        Color::random_in_range(0.5, 1.0),
                        rtweeknd::random_in_range(0.0, 0.5),
                    ))
                } else {
                    Rc::new(Dielectric::new(1.5))
                };
                world.add(Sphere::new(center, 0.2, Some(sphere_mat)));
            }
        }
    }
    let mat1 = Rc::new(Dielectric::new(1.5));
    world.add(Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, Some(mat1)));

    let mat2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, Some(mat2)));

    let mat3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, Some(mat3)));

    let mut cam = Camera::new(16.0 / 9.0, 1200, 100, 20.0);
    cam.max_depth = 50;
    cam.lookfrom = Point::new(13.0, 2.0, 3.0);
    cam.lookat = Point::new(0.0, 0.0, 0.0);
    cam.vup = Vector::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 3.0;
    cam.initialize();

    cam.render(&world);
}
