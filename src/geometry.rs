use glam::Vec3;

use crate::ray::Ray;

pub trait Geometry {
    fn new() -> Self;
}

pub struct Sphere {
    pub radius: f32,
    pub origin: Vec3,
}

impl Sphere {
    pub fn new(radius: f32, origin: Vec3) -> Sphere {
        Sphere { radius, origin }
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

    pub fn set_origin(&mut self, origin: Vec3) {
        self.origin = origin;
    }

    pub fn hit(&self, r: Ray) -> Option<Vec3> {
        let Sphere {
            origin: sphere_origin,
            radius: sphere_radius,
        } = *self;
        let Ray {
            direction: ray_direction,
            origin: ray_origin,
        } = r;
        let p = ray_origin + ray_direction * (sphere_origin - ray_origin).dot(ray_direction);
        let len_to_sphere_origin = (p - sphere_origin).length();
        if len_to_sphere_origin <= sphere_radius {
            Some(p - ray_direction * (sphere_radius.powi(2) - len_to_sphere_origin.powi(2)).sqrt())
        } else {
            None
        }
    }
}
