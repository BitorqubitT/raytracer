use std::mem::Discriminant;
use crate::ray::*;
use crate::matrix::*;
use crate::tuple::*;
use std::ops::{Index, IndexMut, Mul, Sub, };

pub struct Sphere{
 // somethign

}

impl Sphere{

   pub fn new() -> Self {
        Self {
        }
    }

    //TODO:
    // I sometimes use let dot_computed: f64 = vec_one.dot(vec_two);
    // BUt also dot(vecone, vectwo);
    // Checkl this
    // Might need to move this function to other struct
    pub fn intersect(ray: Ray) {

        let sphere_to_ray = ray.origin - Tuple::point(0.0, 0.0, 0.0);

        //let a = dot(ray.direction, ray.direction);
        let a = ray.direction.dot(ray.direction);
        //let b = 2 * dot(ray.direction, sphere_to_ray);
        //let c = dot(sphere_to_ray, sphere_to_ray) - 1;
        let b = 2.0 * ray.direction.dot(ray.direction);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = (b * b) - 4.0 * a * c;

        if discriminant < 0.0 {
            return ()
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            return (t1, t2);
        }
    }


}




#[cfg(test)]
mod tests {
    
    use crate::tuple::Tuple;

    use super::*;

    #[test]
    fn ray_intersect_sphere_at_tangent(){
        let point_a = Tuple::point(0.0, 1.0, -5.0);
        let vector_a = Tuple::vector(0.0, 0.0, 1.0);
        let sphere_a = Sphere::new();
        let ray_a = Ray::new(point_a, vector_a);
        let xs = Sphere.intersect(ray_a);

        assert(len(xs) == 2.0);
        assert(xs[0] == -6.0);
        assert(xs[1] == -4.0);

    }

 

}