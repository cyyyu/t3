use crate::color::Color;
use crate::{ray::Ray, scene::Scene, util};
use glam::Vec3;

static MAX_DIST: f32 = 200.0;

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

    pub fn intersect(&self, r: Ray, scene: &Scene) -> (Color, Vec3) {
        let Sphere {
            origin: sphere_origin,
            radius: sphere_radius,
        } = *self;
        let Ray {
            direction: ray_direction,
            origin: ray_origin,
        } = r;
        let p = ray_origin
            + ray_direction
                * (sphere_origin - ray_origin).dot(ray_direction);
        let len_to_sphere_origin = (p - sphere_origin).length();

        let mut output = Color::new(0.0, 0.0, 0.0);
        if len_to_sphere_origin <= sphere_radius {
            let hit_point = p - ray_direction
                * (sphere_radius.powi(2)
                    - len_to_sphere_origin.powi(2))
                .sqrt();

            // todo: take care of all lights
            let light = scene.lights.first().unwrap();
            let light_pos = light.get_pos();

            // ambient
            output += 0.1;

            // diffuse
            let p_nor = (hit_point - sphere_origin).normalize();
            let mut angle =
                p_nor.dot((light_pos - p_nor).normalize());
            angle = util::clamp(angle, 0.0, 1.0);
            output += util::remap(
                util::clamp(angle, 0.0, 1.0),
                (0.0, 1.0),
                (0.0, 0.2),
            );

            // specular
            let light_reflection =
                util::reflection(hit_point - light_pos, p_nor)
                    .normalize();
            let mut x = (-ray_direction).dot(light_reflection);
            x = util::clamp(x, 0.0, 1.0);
            let strength = 0.4;
            output += strength * x.powi(32);

            return (output, hit_point);
        }

        (output, Vec3::new(MAX_DIST, MAX_DIST, MAX_DIST))
    }
}
