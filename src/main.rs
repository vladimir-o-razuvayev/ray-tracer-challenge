use canvas::Canvas;
use color::WHITE;
use point::Point;
use std::fmt;
use vector::Vector;

mod canvas;
mod color;
mod point;
mod tuple;
mod vector;

struct Environment {
    gravity: Vector,
    wind: Vector,
}

impl Environment {
    pub fn new(gravity: Vector, wind: Vector) -> Self {
        Environment { gravity, wind }
    }
}

struct Projectile {
    position: Point,
    velocity: Vector,
}

impl Projectile {
    pub fn new(position: Point, velocity: Vector) -> Self {
        Projectile { position, velocity }
    }
}

impl fmt::Display for Projectile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(pos: {}, vel: {})", self.position, self.velocity)
    }
}

fn tick(environment: &Environment, projectile: Projectile) -> Projectile {
    let new_position = projectile.position + projectile.velocity;
    let new_velocity = projectile.velocity + environment.gravity + environment.wind;
    let new_projectile = Projectile::new(new_position, new_velocity);
    println!("Projectile: {}", new_projectile);
    new_projectile
}

fn main() {
    let mut canvas = Canvas::new(20, 20);
    canvas.write_pixel(10, 10, WHITE);
    canvas.to_ppm().unwrap();
}
