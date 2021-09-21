use crate::geometry::{Cube, Sphere};
use crate::light::Light;

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub cubes: Vec<Cube>,
    pub lights: Vec<Light>,
}
impl Scene {
    pub fn new() -> Scene {
        Scene {
            spheres: vec![],
            cubes: vec![],
            lights: vec![],
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
}
