use crate::color::Color;
use crate::{ray::Ray, scene::Scene, util};
use glam::Vec3;

static MAX_DIST: f32 = 200.0;

pub struct Plane {
    origin: Vec3,
    normal: Vec3,
    width: f32,
    height: f32,
}

impl Plane {
    pub fn new(
        origin: Vec3,
        normal: Vec3,
        width: f32,
        height: f32,
    ) -> Plane {
        Plane {
            origin,
            normal,
            width,
            height,
        }
    }

    pub fn set_origin(&mut self, origin: Vec3) {
        self.origin = origin;
    }

    pub fn set_normal(&mut self, normal: Vec3) {
        self.normal = normal;
    }

    pub fn intersect(&self, r: Ray, scene: &Scene) -> (Color, Vec3) {
        let Plane {
            origin: _plane_origin,
            normal: plane_normal,
            width: _plane_width,
            height: _plane_height,
        } = *self;
        let Ray {
            direction: ray_direction,
            origin: _ray_origin,
        } = r;
        let dist = self.ray_matching(r);

        let mut output = Color::new(0.0, 0.0, 0.0);

        if dist > MAX_DIST {
            return (output, Vec3::new(MAX_DIST, MAX_DIST, MAX_DIST));
        }

        // todo: take care of all lights
        let light = scene.lights.first().unwrap();
        let light_pos = light.get_pos();

        // ambient
        output += 0.05;

        // diffuse
        let mut angle =
            plane_normal.dot((light_pos - plane_normal).normalize());
        angle = util::clamp(angle, 0.0, 1.0);
        output += util::remap(
            util::clamp(angle, 0.0, 1.0),
            (0.0, 1.0),
            (0.0, 0.3),
        );

        // specular
        let hit_point = r.origin + r.direction * dist;
        let light_reflection =
            util::reflection(hit_point - light_pos, plane_normal)
                .normalize();
        let mut x: f32 = (-ray_direction).dot(light_reflection);
        x = util::clamp(x, 0.0, 1.0);
        let strength = 0.6;
        output += strength * x.powi(32);

        (output, hit_point)
    }

    fn ray_matching(&self, r: Ray) -> f32 {
        let Plane {
            origin: plane_origin,
            normal: plane_normal,
            width: _plane_width,
            height: _plane_height,
        } = *self;
        let Ray {
            direction: ray_direction,
            origin: ray_origin,
        } = r;

        let mut d0 = 0.0;
        let mut p;
        let mut dist;
        let epsilon = 0.5;

        let step = 20_usize;
        for _ in 0..step {
            p = (ray_origin + ray_direction * d0) - plane_origin;

            dist = {
                // distance to plane
                // todo: take care of the case when ray is parallel to plane
                // todo: take care of the case when ray is behind the plane
                let d = p.dot(plane_normal).abs();
                // p - plane_normal * d
                d
            };

            d0 += dist;
            if dist < epsilon || d0 > MAX_DIST {
                break;
            }
        }

        d0
    }
}
