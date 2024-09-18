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
use material::{Lambertian, Metal};
use vector::Vector;
fn main() {
    //materials
    let ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    // world
    let mut world = HittableList::new();
    world.add(Sphere::new(
        Vector::new(0.0, -100.5, -1.0),
        100.0,
        Some(ground.clone()),
    ));
    world.add(Sphere::new(
        Vector::new(0.0, 0.0, -1.2),
        0.5,
        Some(material_center.clone()),
    ));
    world.add(Sphere::new(
        Vector::new(-1.0, 0.0, -1.0),
        0.5,
        Some(material_left.clone()),
    ));
    world.add(Sphere::new(
        Vector::new(1.0, 0.0, -1.0),
        0.5,
        Some(material_right.clone()),
    ));

    // let mut rng = rand::thread_rng();
    // for _ in 0..100 {
    //     let random_x = rng.gen_range(-10.0..10.0);
    //     let random_y = rng.gen_range(1.0..10.0);
    //     let random_z = rng.gen_range(-10.0..-5.0);
    //     let random_radius = rng.gen_range(0.1..1.0);
    //
    //     world.add(Sphere::new(
    //         Vector::new(random_x, random_y, random_z),
    //         random_radius,
    //     ));
    // }
    // camera
    let mut cam = Camera::new(16.0 / 9.0, 800, 100);
    cam.max_depth = 50;
    cam.render(&world);
}
