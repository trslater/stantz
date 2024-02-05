use std::{iter::Sum, ops};

use na::Vector3;

#[derive(Debug)]
pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self {
            red: red.clamp(0.0, 1.0),
            green: green.clamp(0.0, 1.0),
            blue: blue.clamp(0.0, 1.0),
        }
    }

    pub fn new_black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn as_8_bit_array(&self) -> [u8; 3] {
        [
            (self.red * 255.0) as u8,
            (self.green * 255.0) as u8,
            (self.blue * 255.0) as u8,
        ]
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new(
            self.red + rhs.red,
            self.green + rhs.green,
            self.blue + rhs.blue,
        )
    }
}

impl Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|acc, c| acc + c).unwrap_or(Color::new_black())
    }
}

impl ops::Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Color::new(
            self.red * rhs.red,
            self.green * rhs.green,
            self.blue * rhs.blue,
        )
    }
}

impl ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Color::new(self.red * rhs, self.green * rhs, self.blue * rhs)
    }
}

impl Copy for Color {}
impl Clone for Color {
    fn clone(&self) -> Self {
        Color::new(self.red, self.green, self.blue)
    }
}

pub struct Light {
    pub position: Vector3<f32>,
    pub color: Color,
}

impl Light {
    pub fn direction_from(&self, point: &Vector3<f32>) -> Vector3<f32> {
        (self.position - point).normalize()
    }
}
