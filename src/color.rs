use std::{ops::Add, ops::AddAssign};

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn get_char(&self) -> char {
        if self.r >= 0.9 {
            '█'
        } else if self.r >= 0.75 {
            '▓'
        } else if self.r >= 0.55 {
            '▒'
        } else if self.r >= 0.35 {
            '░'
        } else if self.r >= 0.25 {
            '▄'
        } else if self.r >= 0.1 {
            '='
        } else {
            '-'
        }
    }
}

impl AddAssign<f32> for Color {
    fn add_assign(&mut self, rhs: f32) {
        self.r += rhs;
        self.g += rhs;
        self.b += rhs;
    }
}

impl Add<f32> for Color {
    type Output = Color;
    fn add(self, n: f32) -> Color {
        Color {
            r: self.r + n,
            g: self.g + n,
            b: self.b + n,
        }
    }
}
