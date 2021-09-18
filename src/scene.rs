use crate::geometry::Sphere;
use crate::light::Light;

pub struct Scene {
    pub geometries: Vec<Sphere>,
    pub lights: Vec<Light>,
}
impl Scene {
    pub fn new() -> Scene {
        Scene {
            geometries: vec![],
            lights: vec![],
        }
    }

    pub fn add_geometry(&mut self, geo: Sphere) {
        self.geometries.push(geo);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }
}
