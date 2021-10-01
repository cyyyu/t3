use crate::color::Color;
use crate::geometry::{Cube, Plane, Sphere};
use crate::light::Light;
use crate::ray::Ray;

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub cubes: Vec<Cube>,
    pub lights: Vec<Light>,
    pub planes: Vec<Plane>,
}
impl Scene {
    pub fn new() -> Scene {
        Scene {
            spheres: vec![],
            cubes: vec![],
            lights: vec![],
            planes: vec![],
        }
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }

    pub fn add_cube(&mut self, b: Cube) {
        self.cubes.push(b);
    }

    pub fn add_plane(&mut self, plane: Plane) {
        self.planes.push(plane);
    }

    pub fn intersect(&self, r: &Ray, color: &mut Color) {
        let mut dist = f32::MAX;
        self.spheres.iter().for_each(|sphere| {
            let (c, p) = sphere.intersect(*r, &self);
            let d = r.origin.distance(p);
            if d < dist {
                *color = c;
                dist = d;
            }
        });
        self.cubes.iter().for_each(|cube| {
            let (c, p) = cube.intersect(*r, &self);
            let d = r.origin.distance(p);
            if d < dist {
                *color = c;
                dist = d;
            }
        });
        self.planes.iter().for_each(|plane| {
            let (c, p) = plane.intersect(*r, &self);
            let d = r.origin.distance(p);
            if d < dist {
                *color = c;
                dist = d;
            }
        });
    }
}
