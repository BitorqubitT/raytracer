#[macro_use]
use crate::sphere::Sphere;
use crate::ray::*;
use crate::tuple::*;
use std::cmp::Ordering;


#[derive(Debug, PartialEq, Copy, Clone)]
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

    pub fn new(mut objects: Vec<Intersection>) -> Self {
        objects.sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Ordering::Equal));
        // SORT THE INTERSECTIONS HERE ALREADY
        // Does this really work?
        
        Intersections {
            objects: objects,
        }
    }

    pub fn count(&self) -> usize {

        return self.objects.len();

    }

    // FIXHACK: Ugly code. Tests have to use Some. Any better options?
    // When to use reference?
    pub fn hit(&self) -> Option<&Intersection>{

        //TODO: Check diff between self.object.iter() and &self.object
        for i in self.objects.iter() {
            if i.t >= 0.0 {
                return Some(i);
            }
        }
        return None

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
    fn hit_with_all_positive_intersections(){
        let sphere_a = Sphere::new();
        let intersection_1 = Intersection::new(1.0, sphere_a);
        let intersection_2 = Intersection::new(2.0, sphere_a);
        let xs = Intersections::new(vec![intersection_2, intersection_1]);
        let i = xs.hit();
        //assert!(i.t == 1.0);
        assert!(i == Some(&intersection_1));
    }

    #[test]
    fn hit_with_some_negative_intersections(){
        let sphere_a = Sphere::new();
        let intersection_1 = Intersection::new(-1.0, sphere_a);
        let intersection_2 = Intersection::new(1.0, sphere_a);
        let xs = Intersections::new(vec![intersection_2, intersection_1]);
        let i = xs.hit();
        assert!(i == Some(&intersection_2));
    }

    #[test]
    fn hit_with_all_negative_intersections(){
        let sphere_a = Sphere::new();
        let intersection_1 = Intersection::new(-2.0, sphere_a);
        let intersection_2 = Intersection::new(-1.0, sphere_a);
        let xs = Intersections::new(vec![intersection_2, intersection_1]);
        let i = xs.hit();
        assert!(i == None);
    }

    #[test]
    fn hit_is_always_lowest_non_negative_intersections(){
        let sphere_a = Sphere::new();
        let intersection_1 = Intersection::new(5.0, sphere_a);
        let intersection_2 = Intersection::new(7.0, sphere_a);
        let intersection_3 = Intersection::new(-3.0, sphere_a);
        let intersection_4 = Intersection::new(2.0, sphere_a);
        let xs = Intersections::new(vec![intersection_1, intersection_2, intersection_3, intersection_4]);
        let i = xs.hit();
        assert!(i == Some(&intersection_4));

    }

}