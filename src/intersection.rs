use crate::sphere::Sphere;

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

}

impl Intersections {



}

#[cfg(test)]
mod tests {
    
    use crate::sphere;

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

    fn aggregating_intersections(){
        let sphere_a = Sphere::new();
        let new_intersection_1 = Intersection::new(1.0, sphere_a);
        let new_intersection_2 = Intersection::new(2.0, sphere_a);
        //TODO: do we write this as implemn
        let xs = intersections(new_intersection_1, new_intersection_2);
        assert!(xs.count == 2);
        assert!(xs[0].t == 1);
        assert!(xs[1].t == 2);
    }






}