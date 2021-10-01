use std::{ops::Add, ops::AddAssign};

/* [0.0, 1.0] range color */
#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }

    pub fn grayscale(&self) -> Self {
        let avg = (self.r + self.g + self.b) / 3.0;
        Color {
            r: avg,
            g: avg,
            b: avg,
        }
    }

    // return 0-255 rgb color
    pub fn rgb(&self) -> Self {
        Color {
            r: self.r * 255.0,
            g: self.g * 255.0,
            b: self.b * 255.0,
        }
    }

    pub fn get_char(&self) -> char {
        let p = self.r * 0.299 + self.g * 0.587 + self.b * 0.114;
        if p >= 0.9 {
            '█'
        } else if p >= 0.75 {
            '▓'
        } else if p >= 0.55 {
            '▒'
        } else if p >= 0.35 {
            '¤'
        } else if p >= 0.25 {
            '░'
        } else if p >= 0.1 {
            '°'
        } else if p > 0.0 {
            '.'
        } else {
            ' '
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

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Add<Color> for Color {
    type Output = Color;
    fn add(self, n: Color) -> Color {
        Color {
            r: self.r + n.r,
            g: self.g + n.g,
            b: self.b + n.b,
        }
    }
}
