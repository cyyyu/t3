use crate::geometry::Sphere;
use glam::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}
impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn hit(&self, sphere: &Sphere) -> Option<Vec3> {
        let Sphere {
            origin: sphere_origin,
            radius,
        } = sphere;
        let p = self.origin + self.direction * (*sphere_origin - self.origin).dot(self.direction);
        let len_to_sphere_origin = (p - *sphere_origin).length();
        if len_to_sphere_origin <= *radius {
            Some(p - self.direction * (radius.powi(2) - len_to_sphere_origin.powi(2)).sqrt())
        } else {
            None
        }
    }
}
