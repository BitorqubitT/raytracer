#[macro_use]
use crate::sphere::Sphere;
use crate::ray::*;
use crate::tuple::*;

#[derive(Debug)]
pub struct Intersection{
    pub t: f64,
    pub object: Sphere,
}

impl Intersection {

    pub fn new(t: f64, object: Sphere) -> Self {
        Self {
            t,
            object,
        }
    }


}

pub struct Intersections {
    pub objects: Vec<Intersection>,

}

impl Intersections {

    pub fn new(objects: Vec<Intersection>) -> Self {
        
        Intersections {
            objects: objects,
        }
    }

    pub fn count(&self) -> usize {

        return self.objects.len();

    }


}

#[cfg(test)]
mod tests {
    
    use super::*;
    
    #[test]
    fn create_intersection(){
        let time_t = 3.5;
        let sphere_a = Sphere::new();
        let new_intersection = Intersection::new(time_t, sphere_a);

        assert!(new_intersection.t == 3.5);
        //TODO: How can I check this?
        println!("{:?}", new_intersection.object);
    }

    #[test]
    fn aggregating_intersections(){
        let sphere_a = Sphere::new();
        let sphere_b = Sphere::new();
        let new_intersection_1 = Intersection::new(1.0, sphere_a);
        let new_intersection_2 = Intersection::new(2.0, sphere_b);
        //TODO: do we write this as implemn
        let xs = Intersections::new(vec![new_intersection_1, new_intersection_2]);
        //assert!(xs.objects.len() == 2);
        assert!(xs.count() == 2);
        assert!(xs.objects[0].t == 1.0);
        assert!(xs.objects[1].t == 2.0);
    }

    #[test]
    fn intersect_object_on_intersection(){
        // page 64
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let sphere_a = Sphere::new();
        let xs = sphere_a.intersect(r);
        assert!(xs.len() == 2);
        assert(xs.object)

    }




}