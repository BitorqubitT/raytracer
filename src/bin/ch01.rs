/*
Also what does * do?
Need to create bin or lib section, or put everything in main
Otherwise rust cant find other files???
Check Rust FMT

questions:
Why do we need lib ?
cho1 cant find raytracer without?
maybe test this again

TODO;
Cant assert equality between tuples -> solve this
Fix cross product test.
*/

extern crate raytracer;
use raytracer::tuple::*;

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
  Tuple::vector(-0.001, 0.0, 0.0)
    );

    let projectile = Projectile::new(Tuple::point(0.0, 1.0, 0.0), Tuple::vector(0.02, 0.0, 0.0));

    println!("{:?}", environment);

    let mut current = projectile;
    let mut n = 0;

    while current.position.y > 0.0 {
        println!("Position:, {:?}, Amount of ticks: {}", current, n);
        current = tick(&environment, &current);
        n += 1;
    }
}