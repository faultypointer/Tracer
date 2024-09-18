use std::rc::Rc;

use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::{Point, Vector};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vector,
    pub material: Option<Rc<dyn Material>>,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            p: Point::zero(),
            normal: Vector::zero(),
            material: None,
            t: 0.0,
            front_face: false,
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vector) {
        // outward_normal is a unit Vector
        let front_face = Vector::dot(r.direction(), outward_normal) < 0.0;
        if front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    center: Point,
    radius: f64,
    pub material: Option<Rc<dyn Material>>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Option<Rc<dyn Material>>) -> Self {
        // todo material
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = Vector::dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let discriminant_sqrt = f64::sqrt(discriminant);
        let root = (h - discriminant_sqrt) / a;
        if !ray_t.surrounds(root) {
            let root = (h + discriminant_sqrt) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.material = self.material.clone();
        return true;
    }
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add<T: Hittable + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut tmp_record = HitRecord::new();
        let mut hit_anything = false;
        let mut closest = ray_t.max;

        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest), &mut tmp_record) {
                hit_anything = true;
                closest = tmp_record.t;
                *rec = tmp_record.clone();
            }
        }

        hit_anything
    }
}
