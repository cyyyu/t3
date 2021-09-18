use std::ops::{Add, Div, Mul, Sub};

pub fn remap<T>(value: T, domain: (T, T), range: (T, T)) -> T
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    range.0 + (range.1 - range.0) * (value - domain.0) / (domain.1 - domain.0)
}

pub fn clamp<T>(value: T, min: T, max: T) -> T
where
    T: std::cmp::PartialOrd + Copy,
{
    if value > max {
        max
    } else if value < min {
        min
    } else {
        value
    }
}
