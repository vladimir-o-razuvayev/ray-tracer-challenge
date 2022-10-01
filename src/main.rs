use point::Point;
use std::fmt;
use vector::Vector;

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
    let mut projectile = Projectile::new(
        Point::new(0.0, 1.0, 0.0),
        Vector::new(1.0, 1.0, 0.0).normalize(),
    );
    let environment = Environment::new(Vector::new(0.0, -0.1, 0.0), Vector::new(-0.01, 0.0, 0.0));

    println!("Projectile: {}", projectile);
    let mut height = projectile.position.y;
    while height > 0.0 {
        projectile = tick(&environment, projectile);
        height = projectile.position.y;
    }
}
