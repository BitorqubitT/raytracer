use std::mem::Discriminant;
use crate::ray::*;
use crate::matrix::*;
use crate::tuple::*;
use crate::intersection::*;
use std::ops::{Index, IndexMut, Mul, Sub, };

#[derive(Debug, Clone, Copy)]
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
    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {

        let sphere_to_ray = ray.origin - Tuple::point(0.0, 0.0, 0.0);

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = (b * b) - 4.0 * a * c;
        if discriminant < 0.0 {
            return Vec::new()
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            // Pretty sure we want to return an intersection here
            // page 65
            let intersect_1 = Intersection::new(t1, *self);
            let intersect_2 = Intersection::new(t2, *self);
            return vec![intersect_1, intersect_2];
        }
    }
}

#[cfg(test)]
mod tests {
    
    use crate::tuple::Tuple;
    use super::*;

    #[test]
    fn ray_intersect_sphere_at_two_points(){
        let point_a = Tuple::point(0.0, 0.0, -5.0);
        let vector_a = Tuple::vector(0.0, 0.0, 1.0);
        let sphere_a = Sphere::new();
        let ray_a = Ray::new(point_a, vector_a);
        let xs = sphere_a.intersect(ray_a);
        // Clean this up.
        assert!(xs.len() == 2);
        println!("{:?}", xs[0]);
        assert!(xs[0].t == 4.0);
        assert!(xs[1].t == 6.0);
    }

    #[test]
    fn ray_intersect_sphere_object_property(){
        let point_a = Tuple::point(0.0, 0.0, -5.0);
        let vector_a = Tuple::vector(0.0, 0.0, 1.0);
        let sphere_a = Sphere::new();
        let ray_a = Ray::new(point_a, vector_a);
        let xs = sphere_a.intersect(ray_a);
        // Clean this up.
        assert!(xs.len() == 2);
        println!("{:?}", xs[0]);
        // TODO:  implement this with partialEQ
        assert!(xs[0].object == sphere_a);
        assert!(xs[1].object == sphere_a);
    }


    #[test]
    fn ray_intersect_sphere_at_tangent(){
        let point_a = Tuple::point(0.0, 1.0, -5.0);
        let vector_a = Tuple::vector(0.0, 0.0, 1.0);
        let sphere_a = Sphere::new();
        let ray_a = Ray::new(point_a, vector_a);
        let xs = sphere_a.intersect(ray_a);
        // Clean this up.
        assert!(xs.len() == 2);
        assert!(xs[0].t == 5.0);
        assert!(xs[1].t == 5.0);
    }

    #[test]
    fn ray_misses_a_sphere(){
        let point_a = Tuple::point(0.0, 2.0, -5.0);
        let vector_a = Tuple::vector(0.0, 0.0, 1.0);
        let sphere_a = Sphere::new();
        let ray_a = Ray::new(point_a, vector_a);
        let xs = sphere_a.intersect(ray_a);
        assert!(xs.len() == 0);
    }

    #[test]
    fn ray_originates_inside_sphere(){
        let point_a = Tuple::point(0.0, 0.0, 0.0);
        let vector_a = Tuple::vector(0.0, 0.0, 1.0);
        let sphere_a = Sphere::new();
        let ray_a = Ray::new(point_a, vector_a);
        let xs = sphere_a.intersect(ray_a);
        assert!(xs.len() == 2);
        assert!(xs[0].t == -1.0);
        assert!(xs[1].t == 1.0);
    }

    #[test]
    fn sphere_is_behind_ray(){
        let point_a = Tuple::point(0.0, 0.0, 5.0);
        let vector_a = Tuple::vector(0.0, 0.0, 1.0);
        let sphere_a = Sphere::new();
        let ray_a = Ray::new(point_a, vector_a);
        let xs = sphere_a.intersect(ray_a);
        assert!(xs.len() == 2);
        assert!(xs[0].t == -6.0);
        assert!(xs[1].t == -4.0);
    }

}




