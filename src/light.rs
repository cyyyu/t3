use glam::Vec3;

pub struct Light(Vec3);

impl Light {
    pub fn new(pos: Vec3) -> Self {
        Light(pos)
    }

    pub fn get_pos(&self) -> Vec3 {
        self.0.clone()
    }

    pub fn set_pos(&mut self, pos: Vec3) {
        self.0 = pos;
    }
}
