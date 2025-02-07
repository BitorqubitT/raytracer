use crate::tuple::Tuple;

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


}