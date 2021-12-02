pub const EPSILON: f32 = 0.00001;

fn reverse(value: (f32, f32, f32, f32)) -> (f32, f32, f32, f32) {
    let (x, y, z, w) = value;
    (w, z, y, x)
}

fn point(x: f32, y: f32, z: f32) -> (f32, f32, f32, f32) {
    (x, y, z, 1.0)
}

fn vector(x: f32, y: f32, z: f32) -> (f32, f32, f32, f32) {
    (x, y, z, 0.0)
}
fn equal(a: f32, b: f32) -> bool {
    f32::abs(a - b) < EPSILON
}

fn add(a: (f32, f32, f32, f32), b: (f32, f32, f32, f32)) -> (f32, f32, f32, f32) {
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

fn sub(a: (f32, f32, f32, f32), b: (f32, f32, f32, f32)) -> (f32, f32, f32, f32) {
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

fn main() {
    let p1 = point(4.3, -4.2, 3.1);
    println!("p1: {:?}", p1);

    let v1 = vector(4.3, -4.2, 3.1);
    println!("v1: {:?}", v1);

    let zyx = reverse(v1);

    println!("wzyx: {:?}", zyx);

    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

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
}
