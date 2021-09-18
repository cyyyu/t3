use glam::Vec3;

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
}
