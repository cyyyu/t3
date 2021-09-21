use crate::{ray::Ray, scene::Scene, util};
use glam::Vec3;

static MAX_DIST: f32 = 200.0;

pub struct Cube {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub origin: Vec3,
}

impl Cube {
    pub fn new(
        width: f32,
        height: f32,
        depth: f32,
        origin: Vec3,
    ) -> Self {
        Cube {
            width,
            height,
            depth,
            origin,
        }
    }

    pub fn get_pos(&self) -> Vec3 {
        self.origin
    }

    pub fn set_pos(&mut self, pos: Vec3) {
        self.origin = pos;
    }

    pub fn hit(&self, r: Ray, scene: &Scene) -> f32 {
        let dist = self.ray_matching(r);

        let mut contribution = 0.0;

        if dist > MAX_DIST {
            return contribution;
        }

        // ambient
        contribution += 0.05;

        // diffuse
        let hit_point = r.origin + r.direction * dist;
        let x = Vec3::new(1.0, 0.0, 0.0);
        let y = Vec3::new(0.0, 1.0, 0.0);
        let z = Vec3::new(0.0, 0.0, 1.0);
        let light = scene.lights.first().unwrap(); // todo: take care of all lights
        let light_pos = light.get_pos();
        let dx = (hit_point - self.origin).normalize().dot(x);
        let dy = (hit_point - self.origin).normalize().dot(y);
        let dz = (hit_point - self.origin).normalize().dot(z);
        let nor = {
            if dz > dy && dz > dx {
                z
            } else if dy > dx {
                y
            } else {
                x
            }
        };
        let mut angle = nor.dot((light_pos - hit_point).normalize());
        angle = util::clamp(angle, 0.0, 1.0);
        contribution += util::remap(
            util::clamp(angle, 0.0, 1.0),
            (0.0, 1.0),
            (0.0, 0.6),
        );

        // specular
        let light_reflection =
            util::reflection(hit_point - light_pos, nor).normalize();
        let mut x = (-r.direction).dot(light_reflection);
        x = util::clamp(x, 0.0, 1.0);
        let strength = 0.4;
        contribution += strength * x.powi(32);

        contribution
    }

    fn ray_matching(&self, r: Ray) -> f32 {
        let Cube {
            width,
            height,
            depth,
            origin: cube_origin,
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
            p = (ray_origin + ray_direction * d0) - cube_origin;
            dist = {
                let mut d = p.abs()
                    - Vec3::new(
                        width / 2.0,
                        height / 2.0,
                        depth / 2.0,
                    );
                d.x = util::max(d.x, 0.0);
                d.y = util::max(d.y, 0.0);
                d.z = util::max(d.z, 0.0);
                d.length()
            };
            d0 += dist;
            if dist < epsilon || d0 > MAX_DIST {
                break;
            }
        }

        d0
    }
}
