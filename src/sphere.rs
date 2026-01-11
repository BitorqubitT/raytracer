use std::mem::Discriminant;
use std::ptr;
use std::vec;
use crate::ray::*;
use crate::matrix::*;
use crate::tuple::*;
use crate::intersection::*;
use std::ops::{Index, IndexMut, Mul, Sub, };

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere{
 // somethign
    pub transform: Matrix,

}

impl Default for Sphere {
    fn default() -> Self {

        let identity_m = vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ];

        let matrix = Matrix::new(4,4, identity_m);

        Self {
            transform: matrix,

        }
    }
}


impl Sphere{

   pub fn new(transform: Matrix) -> Self {
        Sphere {
            transform,
        }
    }

    //TODO:
    // I sometimes use let dot_computed: f64 = vec_one.dot(vec_two);
    // BUt also dot(vecone, vectwo);
    // Checkl this
    // Should this function be able to return multiple?
    // TODO: Check if tests below are still relevant, or only this one.
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
            // page 65
            // Might need to change this back to *self. instead of clone
            let intersect_1 = Intersection::new(t1, self.clone());
            let intersect_2 = Intersection::new(t2, self.clone());
            return vec![intersect_1, intersect_2];
        }
    }

    //CHECK: Should be mut self not &self
    pub fn set_transform(&self, transform: Matrix) -> Self {
        // Default is identity matrix
        //TODO: https://doc.rust-lang.org/std/default/trait.Default.html   refactor

        //TODO: might need to make widht,height an param
        //let identity_matrix = Matrix::new(4,4, identity_m);
        self.transform = transform;
        self
        //return identity_matrix;
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
        let sphere_a = Sphere::default();
        let ray_a = Ray::new(point_a, vector_a);
        let xs = sphere_a.intersect(ray_a);
        assert!(xs.len() == 2);
        assert!(xs[0].t == 4.0);
        assert!(xs[1].t == 6.0);
    }

    #[test]
    fn ray_intersect_sphere_object_property(){
        // Changed this test for new property
        let point_a = Tuple::point(0.0, 0.0, -5.0);
        let vector_a = Tuple::vector(0.0, 0.0, 1.0);
        let sphere_a = Sphere::default();
        let ray_a = Ray::new(point_a, vector_a);
        let xs = sphere_a.intersect(ray_a);
        // Clean this up.
        assert!(xs.len() == 2);
        // TODO:  implement this with partialEQ
        //assert!(ptr::eq(&xs[0].object, &sphere_a));
        //assert!(ptr::eq(&xs[1].object, &sphere_a));
    }


    #[test]
    fn ray_intersect_sphere_at_tangent(){
        let point_a = Tuple::point(0.0, 1.0, -5.0);
        let vector_a = Tuple::vector(0.0, 0.0, 1.0);
        let sphere_a = Sphere::default();
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
        let sphere_a = Sphere::default();
        let ray_a = Ray::new(point_a, vector_a);
        let xs = sphere_a.intersect(ray_a);
        assert!(xs.len() == 0);
    }

    #[test]
    fn ray_originates_inside_sphere(){
        let point_a = Tuple::point(0.0, 0.0, 0.0);
        let vector_a = Tuple::vector(0.0, 0.0, 1.0);
        let sphere_a = Sphere::default();
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
        let sphere_a = Sphere::default();
        let ray_a = Ray::new(point_a, vector_a);
        let xs = sphere_a.intersect(ray_a);
        assert!(xs.len() == 2);
        assert!(xs[0].t == -6.0);
        assert!(xs[1].t == -4.0);
    }

    //page 69
    #[test]
    fn sphere_default_transform(){
        let s = Sphere::default();
        let identity_m = vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ];
        let identity_matrix = Matrix::new(4,4, identity_m);

        assert!(s.transform == identity_matrix);
    }

    #[test]
    fn change_sphere_transform(){
        let mut s = Sphere::default();
        let translation_matrix = Matrix::translation(2.0, 3.0, 4.0);
        
        let translation_s = s.set_transform(translation_matrix);
        assert!(translation_s.transform == translation_matrix);
    }



}
