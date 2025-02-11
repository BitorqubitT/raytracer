
pub struct Sphere(){
 // somethign

}

impl Sphere{

   pub fn new() -> Self {
        Self {
        }
    }


    // Might need to move this function to other struct
    pub fn intersect(){


    }



}




#[cfg(test)]
mod tests {
    
    use crate::tuple::Tuple;

    use super::*;

    #[test]
    fn ray_intersect_sphere_at_tangent(){
        let point_a = Tuple::point_a(0.0, 1.0, -5.0);
        let vector_a = Tuple::vector(0.0, 0.0, 1.0);
        let sphere_a = Sphere::new();
        let ray_a = Ray::new(point_a, vector_a);
        let xs = Sphere.intersect(ray_a);

        assert(len(xs) == 2);
        assert(xs[0] == -6.0);
        assert(xs[1] == -4.0);

    }

 

}