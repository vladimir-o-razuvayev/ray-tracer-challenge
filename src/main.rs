use canvas::Canvas;
use color::WHITE;
use point::Point;
use std::fmt;
use vector::Vector;

mod canvas;
mod color;
mod matrix;
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
    new_projectile
}

fn main() {
    let canvas_height: usize = 550;
    let canvas_width: usize = 900;
    let mut canvas = Canvas::new(canvas_width, canvas_height);
    let mut projectile = Projectile::new(
        Point::new(0.0, 1.0, 0.0),
        Vector::new(1.0, 1.8, 0.0).normalize() * 11.25,
    );
    let environment = Environment::new(Vector::new(0.0, -0.1, 0.0), Vector::new(-0.01, 0.0, 0.0));

    let mut height = projectile.position.y;
    canvas.write_pixel(
        projectile.position.x as usize,
        canvas_height - height as usize,
        WHITE,
    );

    while height > 0.0 {
        projectile = tick(&environment, projectile);
        height = projectile.position.y;
        canvas.write_pixel(
            projectile.position.x as usize,
            canvas_height - height as usize,
            WHITE,
        );
    }

    canvas.to_ppm().unwrap();
}
