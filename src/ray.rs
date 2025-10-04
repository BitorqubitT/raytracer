use crate::tuple::Tuple;
use crate::matrix::Matrix;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {

    pub fn new(origin: Tuple, direction: Tuple) -> Self {

        Self {
            origin,
            direction,
        }
    }

    pub fn position(&self, t: f64) -> Tuple {
        // t * self.direction doesnt work need to implement lhs
        let new_position = self.origin + (self.direction * t);
        return new_position

    }

    pub fn transform(&self, translation: Matrix) -> Matrix {

        let new_
        // The difference lies in what we receive. 
        // Detect this and then act on it
        // return a new ray
        // page 89 or 69 in book
        return transformed_ray
    }

}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn check_ray_implementation(){
        let origin = Tuple::point(1.0, 2.0, 3.0);
        let direction = Tuple::vector(4.0, 5.0, 6.0);

        let new_ray = Ray::new(origin, direction);
        assert!(new_ray.origin == origin);
        assert!(new_ray.direction == direction);
    }

    #[test]
    fn compute_point_from_distance(){
        let origin = Tuple::point(2.0, 3.0, 4.0);
        let direction = Tuple::vector(1.0, 0.0, 0.0);

        let new_ray = Ray::new(origin, direction);

        assert!(new_ray.position(0.0) == Tuple::point(2.0, 3.0, 4.0));
        assert!(new_ray.position(1.0) == Tuple::point(3.0, 3.0, 4.0));
        assert!(new_ray.position(-1.0) == Tuple::point(1.0, 3.0, 4.0));
        assert!(new_ray.position(2.5) == Tuple::point(4.5, 3.0, 4.0));

    }

    #[test]
    fn translate_ray(){
        let origin = Tuple::point(1.0, 2.0, 3.0);
        let direction = Tuple::vector(0.0, 1.0, 0.0);
        let ray = Ray::new(origin, direction);

        let translation = Matrix::translation(3.0, 4.0, 5.0);

        // What makes if we do type.function() or function(lala, lala)?
        let transformed_ray = ray.tranform(translation);
        assert!(tranformed_ray.origin == Tuple::point(4.0, 6.0, 8.0));
        assert!(tranformed_ray.direction == Tuple::vector(0.0, 1.0, 0.0));

    }

    #[test]
    fn scale_ray(){
        let origin = Tuple::point(1.0, 2.0, 3.0);
        let direction = Tuple::vector(0.0, 1.0, 0.0);
        let ray = Ray::new(origin, direction);
        // Should this be a vector
        let scaling = Matrix::scaling(2.0, 3.0, 4.0);

        // Apply scaling
        let transformed_ray = transform(ray, scaling);
        let tranformed_ray.origin = Tuple::point(4.0, 6.0, 8.0);
        let tranformed_ray.direction = Tuple::vector(0.0, 1.0, 0.0);
        assert!(tranformed_ray.origin == Tuple::point(4.0, 6.0, 8.0));
        assert!(tranformed_ray.direction == Tuple::vector(0.0, 1.0, 0.0));
    }




}