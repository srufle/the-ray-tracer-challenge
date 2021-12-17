pub const EPSILON: f32 = 0.000_01;

pub fn reverse(value: (f32, f32, f32, f32)) -> (f32, f32, f32, f32) {
    let (x, y, z, w) = value;
    (w, z, y, x)
}

pub fn point(x: f32, y: f32, z: f32) -> (f32, f32, f32, f32) {
    (x, y, z, 1.0)
}

pub fn vector(x: f32, y: f32, z: f32) -> (f32, f32, f32, f32) {
    (x, y, z, 0.0)
}
pub fn equal(a: f32, b: f32) -> bool {
    f32::abs(a - b) < EPSILON
}

pub fn add(a: (f32, f32, f32, f32), b: (f32, f32, f32, f32)) -> (f32, f32, f32, f32) {
    #![allow(clippy::many_single_char_names)]
    let x = a.0 + b.0;
    let y = a.1 + b.1;
    let z = a.2 + b.2;
    let w = if equal(a.3, 1.0) && equal(b.3, 1.0) {
        1.0
    } else {
        a.3 + b.3
    };
    (x, y, z, w)
}

pub fn sub(a: (f32, f32, f32, f32), b: (f32, f32, f32, f32)) -> (f32, f32, f32, f32) {
    #![allow(clippy::many_single_char_names)]
    let x = a.0 - b.0;
    let y = a.1 - b.1;
    let z = a.2 - b.2;
    let w = if equal(a.3, 1.0) && equal(b.3, 1.0) {
        0.0
    } else {
        f32::abs(a.3 - b.3)
    };
    (x, y, z, w)
}

pub fn negate(a: (f32, f32, f32, f32)) -> (f32, f32, f32, f32) {
    let x = -a.0;
    let y = -a.1;
    let z = -a.2;
    let w = -a.3;
    (x, y, z, w)
}

pub fn mult(a: (f32, f32, f32, f32), scale: f32) -> (f32, f32, f32, f32) {
    let x = a.0 * scale;
    let y = a.1 * scale;
    let z = a.2 * scale;
    let w = a.3 * scale;
    (x, y, z, w)
}

pub fn div(a: (f32, f32, f32, f32), scale: f32) -> (f32, f32, f32, f32) {
    if scale == 0.0 || scale < EPSILON {
        panic!("scale must be greater than 0.0")
    }
    let x = a.0 / scale;
    let y = a.1 / scale;
    let z = a.2 / scale;
    let w = a.3 / scale;
    (x, y, z, w)
}

pub fn magnitude(a: (f32, f32, f32, f32)) -> f32 {
    let x_sq = f32::powi(a.0, 2);
    let y_sq = f32::powi(a.1, 2);
    let z_sq = f32::powi(a.2, 2);
    let w_sq = f32::powi(a.3, 2);
    let sum_of = x_sq + y_sq + z_sq + w_sq;
    sum_of.sqrt()
}

pub fn normalize(a: (f32, f32, f32, f32)) -> (f32, f32, f32, f32) {
    let mag = magnitude(a);
    div(a, mag)
}

#[allow(clippy::let_and_return)]
pub fn dot(a: (f32, f32, f32, f32), b: (f32, f32, f32, f32)) -> f32 {
    let ans = a.0 * b.0;
    let ans = ans + (a.1 * b.1);
    let ans = ans + a.2 * b.2;
    let ans = ans + a.3 * b.3;
    ans
}

pub fn cross(a: (f32, f32, f32, f32), b: (f32, f32, f32, f32)) -> (f32, f32, f32, f32) {
    let ax = a.0;
    let ay = a.1;
    let az = a.2;

    let bx = b.0;
    let by = b.1;
    let bz = b.2;

    let x = (ay * bz) - (az * by);
    let y = (az * bx) - (ax * bz);
    let z = (ax * by) - (ay * bx);
    vector(x, y, z)
}

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

    #[test]
    fn test_point() {
        let actual = point(4.3, -4.2, 3.1);
        let expected = (4.3, -4.2, 3.1, 1.0);
        assert_eq!(expected, actual);
        assert_eq!(actual.3, 1.0, "Is a point");
        assert_ne!(actual.3, 0.0, "Is NOT a vector");
    }

    #[test]
    fn test_vector() {
        let actual = vector(4.3, -4.2, 3.1);
        let expected = (4.3, -4.2, 3.1, 0.0);
        assert_eq!(expected, actual);
        assert_ne!(actual.3, 1.0, "Is NOT a point");
        assert_eq!(actual.3, 0.0, "Is a vector");
    }

    #[test]
    fn test_reverse() {
        let v1 = vector(4.3, -4.2, 3.1);
        let actual = reverse(v1);
        let expected = (0.0, 3.1, -4.2, 4.3);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_eq_true() {
        assert!(equal(1.00001, 1.00001));
        assert!(equal(1.000013, 1.000012));
        assert!(equal(-1.000013, -1.000012));
        assert!(equal(1.00001, 1.000011));
    }

    #[test]
    fn test_eq_false() {
        assert!(!equal(1.00001, 1.00002));
    }

    #[test]
    fn test_add_point_and_point() {
        let p1 = point(3.0, -2.0, 5.0);
        let p2 = point(-2.0, 3.0, 1.0);
        let actual = add(p1, p2);
        let expected = (1.0, 1.0, 6.0, 1.0);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_add_point_and_vector() {
        let p1 = point(3.0, -2.0, 5.0);
        let v1 = vector(-2.0, 3.0, 1.0);
        let actual = add(p1, v1);
        let expected = (1.0, 1.0, 6.0, 1.0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_add_vector_and_vector() {
        let v1 = vector(3.0, -2.0, 5.0);
        let v2 = vector(-2.0, 3.0, 1.0);
        let actual = add(v1, v2);
        let expected = (1.0, 1.0, 6.0, 0.0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sub_point_and_point() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);
        let actual = sub(p1, p2);
        let expected = vector(-2.0, -4.0, -6.0);
        assert_eq!(expected, actual);

        let actual = sub(p2, p1);
        let expected = vector(2.0, 4.0, 6.0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sub_point_and_vector() {
        let p1 = point(3.0, 2.0, 1.0);
        let v1 = vector(5.0, 6.0, 7.0);
        let actual = sub(p1, v1);
        let expected = point(-2.0, -4.0, -6.0);
        assert_eq!(expected, actual);

        let actual = sub(v1, p1);
        let expected = point(2.0, 4.0, 6.0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sub_vector_and_vector() {
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);
        let actual = sub(v1, v2);
        let expected = vector(-2.0, -4.0, -6.0);
        assert_eq!(expected, actual);

        let actual = sub(v2, v1);
        let expected = vector(2.0, 4.0, 6.0);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_negate_tuple() {
        let t1 = (1.0, -2.0, 3.0, 0.0);
        let actual = negate(t1);
        let expected = (-1.0, 2.0, -3.0, 0.0);
        assert_eq!(expected, actual);

        let t1 = (1.0, -2.0, 3.0, 1.0);
        let actual = negate(t1);
        let expected = (-1.0, 2.0, -3.0, -1.0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_scalar_mult() {
        let t1 = (1.0, -2.0, 3.0, -4.0);
        let actual = mult(t1, 3.5);
        let expected = (3.5, -7.0, 10.5, -14.0);
        assert_eq!(expected, actual);

        let t1 = (1.0, -2.0, 3.0, -4.0);
        let actual = mult(t1, 0.5);
        let expected = (0.5, -1.0, 1.5, -2.0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_scalar_div() {
        let t1 = (1.0, -2.0, 3.0, -4.0);
        let actual = div(t1, 2.0);
        let expected = (0.5, -1.0, 1.5, -2.0);
        assert_eq!(expected, actual);
    }

    #[test]
    #[should_panic]
    fn test_scalar_div_0() {
        let t1 = (1.0, -2.0, 3.0, -4.0);
        let _ = div(t1, 0.0);
    }

    #[test]
    #[should_panic]
    fn test_scalar_div_smaller_than_epsilon() {
        let t1 = (1.0, -2.0, 3.0, -4.0);
        let _ = div(t1, EPSILON / 10.0);
    }

    #[test]
    fn test_scalar_div_epsilon() {
        let t1 = (1.0, -2.0, 3.0, -4.0);
        let actual = div(t1, EPSILON);
        let expected = (100000.0, -200000.0, 300000.0, -400000.0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_magnitude() {
        let v1 = vector(1.0, 0.0, 0.0);
        let actual = magnitude(v1);
        let expected = 1.0;
        assert_eq!(expected, actual);

        let v1 = vector(0.0, 1.0, 0.0);
        let actual = magnitude(v1);
        let expected = 1.0;
        assert_eq!(expected, actual);

        let v1 = vector(0.0, 0.0, 1.0);
        let actual = magnitude(v1);
        let expected = 1.0;
        assert_eq!(expected, actual);

        let v1 = vector(1.0, 2.0, 3.0);
        let actual = magnitude(v1);
        let expected = 3.741_657_5;
        assert!(equal(actual, expected));

        let v1 = negate(vector(1.0, 2.0, 3.0));
        let actual = magnitude(v1);
        let expected = 3.741_657_5;
        println!("actual magnitude: {}", actual);
        assert!(equal(actual, expected));
    }

    #[test]
    fn test_normalize() {
        let v1 = vector(4.0, 0.0, 0.0);
        let actual = normalize(v1);
        let expected = vector(1.0, 0.0, 0.0);
        assert_eq!(actual, expected);

        let v1 = vector(1.0, 2.0, 3.0);
        let actual = normalize(v1);
        let sqrt_14 = f32::powf(14.0, 0.5);
        let expected = div(v1, sqrt_14);
        assert_eq!(actual, expected);

        let actual = magnitude(actual);
        let expected = 1.0;
        assert!(equal(actual, expected));
    }

    #[test]
    fn test_dot() {
        let v1 = vector(1.0, 2.0, 3.0);
        let v2 = vector(2.0, 3.0, 4.0);
        let actual = dot(v1, v2);
        let expected = 20.0;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_cross() {
        let v1 = vector(1.0, 2.0, 3.0);
        let v2 = vector(2.0, 3.0, 4.0);
        let actual = cross(v1, v2);
        let expected = vector(-1.0, 2.0, -1.0);
        assert_eq!(actual, expected);

        let actual = cross(v2, v1);
        let expected = vector(1.0, -2.0, 1.0);
        assert_eq!(actual, expected);
    }
}
// Run tests with output
// cargo test -- --nocapture
