use glam::Vec3;

use crate::{ray::Ray, scene::Scene, util};

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

    pub fn hit(&self, r: Ray, scene: &Scene) -> f32 {
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

        let mut contribution = 0.0;
        if len_to_sphere_origin <= sphere_radius {
            let hit_point =
                p - ray_direction * (sphere_radius.powi(2) - len_to_sphere_origin.powi(2)).sqrt();

            // ambient
            contribution += 0.1;

            // diffuse
            let light = scene.lights.first().unwrap();
            let light_pos = light.get_pos();
            let p_nor = (hit_point - self.origin).normalize();
            let angle = p_nor.dot((light_pos - p_nor).normalize());
            contribution += if angle >= 0.0 {
                util::remap(util::clamp(angle, 0.0, 1.0), (0.0, 1.0), (0.0, 0.6))
            } else {
                0.0
            };
            // todo: specular
        }

        contribution
    }
}
