// A projectile has a position (a point) and a velocity (a vector).
// An environment has gravity (a vector) and wind (a vector).
// Then, add a tick(environment, projectile) function which returns a new projectile,
//  representing the given projectile after one unit of time has passed.
//  (The actual units here don’t really matter—maybe they’re seconds, or milliseconds.
//  Whatever. We’ll just call them “ticks.”)

use trtc::{add, normalize, point, vector};

#[derive(Debug, Clone, Copy)]
struct Projectile {
    position: (f32, f32, f32, f32),
    velocity: (f32, f32, f32, f32),
}

#[derive(Debug, Clone, Copy)]
struct Environment {
    gravity: (f32, f32, f32, f32),
    wind: (f32, f32, f32, f32),
}

fn tick(env: Environment, proj: Projectile) -> Projectile {
    let position = add(proj.position, proj.velocity);
    let velocity = add(proj.velocity, env.gravity);
    let velocity = add(velocity, env.wind);
    Projectile { position, velocity }
}
fn main() {
    println!();
    println!("Running ticker");
    println!();

    let mut projectile = Projectile {
        position: point(0.0, 1.0, 0.0),
        velocity: normalize(vector(1.0, 1.0, 0.0)),
    };

    let environment = Environment {
        gravity: vector(0.0, -0.1, 0.0),
        wind: vector(-0.01, 0.0, 0.0),
    };

    println!("Initial State:");
    println!(" projectile:  {:?}", &projectile);
    println!(" environment: {:?}", &environment);
    println!();
    while projectile.position.1 > 0.0 {
        projectile = tick(environment, projectile);
        // println!(" projectile:  {:?}", &projectile);
        println!(
            "{:?}, {:?}, {:?}",
            &projectile.position.0, &projectile.position.1, &projectile.position.2
        );
    }
}
