
extern crate raytracer;
use raytracer::tuple::*;
use raytracer::canvas::*;

// Do I still need clone and copy?
#[derive(Debug, Copy, Clone)]
pub struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

#[derive(Debug, Copy, Clone)]
pub struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

impl Projectile {
    pub fn new(position: Tuple, velocity: Tuple) -> Self {
        Projectile{position, velocity}
    }
}

impl Environment {
    pub fn new(gravity: Tuple, wind: Tuple) -> Self {
        Environment{gravity, wind}
    }
}

fn tick(environment: &Environment, projectile: &Projectile) -> Projectile {
    Projectile::new(projectile.position + projectile.velocity,
                    projectile.velocity + environment.gravity + environment.wind)
}

fn main() {
    let environment = Environment::new(
Tuple::vector(0.0, -0.1, 0.0),
  Tuple::vector(-0.01, 0.0, 0.0)
    );


    // Normalize the vector * 11.25 
    // Get function from canvas
    let projectile = Projectile::new(Tuple::point(0.0, 1.0, 0.0), Tuple::vector(1.0, 1.8, 0.0));


    let mut current = projectile;
    let mut n = 0;

    while current.position.y > 0.0 {
        println!("Position:, {:?}, Amount of ticks: {}", current, n);
        current = tick(&environment, &current);
        n += 1;
    }
}