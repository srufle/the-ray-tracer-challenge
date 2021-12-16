use crate::trtc::{point, reverse, vector};

pub fn run() {
    let p1 = point(4.3, -4.2, 3.1);
    println!("p1: {:?}", p1);

    let v1 = vector(4.3, -4.2, 3.1);
    println!("v1: {:?}", v1);

    let zyx = reverse(v1);

    println!("wzyx: {:?}", zyx);

    println!("Hello, world!");
    println!();
}
