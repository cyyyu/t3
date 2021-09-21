use glam::Vec3;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}
impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }
}
