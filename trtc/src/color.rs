use crate::EPSILON;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        Color { red, green, blue }
    }

    pub fn equal(self, other: Color) -> bool {
        f32::abs(self.red - other.red) < EPSILON
            && f32::abs(self.green - other.green) < EPSILON
            && f32::abs(self.blue - other.blue) < EPSILON
    }

    pub fn black() -> Color {
        Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }
    pub fn red() -> Color {
        Color {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
        }
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let red = self.red + other.red;
        let green = self.green + other.green;
        let blue = self.blue + other.blue;
        Self { red, green, blue }
    }
}

impl Sub for Color {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let red = self.red - other.red;
        let green = self.green - other.green;
        let blue = self.blue - other.blue;
        Self { red, green, blue }
    }
}

impl Mul<f32> for Color {
    type Output = Self;
    fn mul(self, factor: f32) -> Self {
        let red = self.red * factor;
        let green = self.green * factor;
        let blue = self.blue * factor;
        Self { red, green, blue }
    }
}

impl Mul<Color> for Color {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let red = self.red * other.red;
        let green = self.green * other.green;
        let blue = self.blue * other.blue;
        Self { red, green, blue }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_color() {
        let c1 = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(c1.red, -0.5);
        assert_eq!(c1.green, 0.4);
        assert_eq!(c1.blue, 1.7);
    }

    #[test]
    fn test_color_add() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let actual = c1 + c2;
        let expected = Color::new(1.6, 0.7, 1.0);
        assert!(expected.equal(actual));
    }

    #[test]
    fn test_color_sub() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let actual = c1 - c2;
        let expected = Color::new(0.2, 0.5, 0.5);
        assert!(expected.equal(actual));
    }

    #[test]
    fn test_color_mult_scalar() {
        let c1 = Color::new(0.2, 0.3, 0.4);
        let actual = c1 * 2.0;
        let expected = Color::new(0.4, 0.6, 0.8);
        assert!(expected.equal(actual));
    }

    #[test]
    fn test_color_mult_color() {
        // https://en.wikipedia.org/wiki/Hadamard_product_(matrices)
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        let actual = c1 * c2;
        let expected = Color::new(0.9, 0.2, 0.04);
        assert!(expected.equal(actual));
    }
}
