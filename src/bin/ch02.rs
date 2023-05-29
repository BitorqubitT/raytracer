extern crate raytracer;
use raytracer::tuple::*;
use raytracer::canvas::*;

// Calculate the trajectory of a projectile.
// Write this trajectory to a vector.
// Then write this to ppm to view it as an image.

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

    // Maybe give to ppm an extra var -> file_name 
    let mut new_canvas = Canvas::new(100, 100);
    let rainbow = Color::new(1.0, 0.0, 0.0);

    while current.position.y > 0.0 {
        current = tick(&environment, &current);

        // Tuple gives back a vector, catch these in a variable and set to usize.
        // Converting type doesn't seem ideal, however the point to a pos in matrix.
        // Need to use usize? Could int be used?
        // Why shouldn't these be mut?
        let current_x = current.position.x as usize;
        let current_y = 99 - current.position.y as usize;

        new_canvas.write_pixel(current_x, current_y, rainbow);
    }
    let _canvas = new_canvas.canvas_to_ppm();

}




