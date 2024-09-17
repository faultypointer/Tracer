mod camera;
mod color;
mod hittable;
mod interval;
mod ray;
mod rtweeknd;
mod vector;

use camera::Camera;
use hittable::{HittableList, Sphere};
use vector::Vector;
fn main() {
    // world
    let mut world = HittableList::new();
    world.add(Sphere::new(Vector::new(0.0, 0.1, -1.0), 0.5));
    world.add(Sphere::new(Vector::new(0.4, 0.3, -1.0), 0.5));
    world.add(Sphere::new(Vector::new(0.3, 0.6, -1.0), 0.5));
    world.add(Sphere::new(Vector::new(0.7, 0.4, -1.0), 0.5));
    world.add(Sphere::new(Vector::new(0.8, 0.2, -1.0), 0.5));
    world.add(Sphere::new(Vector::new(1.0, -1.5, -1.0), 0.5));

    // camera
    let cam = Camera::new(16.0 / 9.0, 400);
    cam.render(&world);
}
