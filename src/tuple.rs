use std::ops::{Add, Div, Mul, Neg, Sub};

//use crate::fuzzy_eq::*;
//Maybe PartialeEq
// could make pub x:
#[derive(Debug, Copy, Clone)]
pub struct Tuple {
   pub x: f64,
   pub y: f64,
   pub z: f64,
   pub w: f64,
    }

impl Tuple {

    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
            Tuple { x, y, z, w}
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
            Tuple { x, y, z, w: 1.0}
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Self {
            Tuple { x, y, z, w: 0.0}
    }
}

// should i use: Add<Self>  ?????
impl Add for Tuple{
    type Output = Self;

    fn add(self, other: Tuple) -> Self::Output{
        Tuple::new( self.x + other.x,
                    self.y + other.y,
                    self.z + other.z,
                    self.w + other.w)
    }
}
   
impl Sub for Tuple{
    type Output = Self;

    fn sub(self, other: Tuple) -> Self::Output{
        Tuple::new( self.x - other.x,
                    self.y - other.y,
                    self.z - other.z,
                    self.w - other.w)
    }
}

 // Create operator overload, to negate a tuple
impl Neg for Tuple{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {x: -self.x, y: -self.y, z: -self.z, w: -self.w}
    }
}

// What is the difference between mul & MulAssign?
// Do we use MulAssign to change the given tuple?
// And we use MUl if we want to create a new one?
// Is this the solution that I am looking for?
// We call self.x etc here because we alter the input tuple.
// We dont return anything but the tuple.

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Tuple::new(
            self.x * other,
            self.y * other,
            self.z * other,
            self.w * other 
        )
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        Tuple::new(
            self.x / other,
            self.y / other,
            self.z / other,
            self.w / other
        )
    }
}

// Why the & before self
impl Tuple{

    pub fn magnitude(&self) -> f64{
        let magnitude_v = f64::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2));
        return magnitude_v;
    }

    pub fn norm(&self) -> Self {
        *self / self.magnitude()
    }

// Could put everything in one line, dont need the variable. Best practice?
    pub fn dot(&self, other: Tuple) -> f64{
        let dot_answer: f64 = (self.x * other.x) +
                              (self.y * other.y) +
                              (self.z * other.z) +
                              (self.w * other.w);
        return dot_answer; 
    }

    pub fn cross(&self, other: Tuple) -> Tuple{
        Tuple::vector ( (self.y * other.z) - (self.z * other.y),
                        (self.z * other.x) - (self.x * other.z),
                        (self.x * other.y) - (self.y * other.x),
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_tuple() {
        let tuple_point = Tuple::point(2.1, -2.3, 3.0);
        assert!(tuple_point.x == 2.1);
        assert!(tuple_point.y == -2.3);
        assert!(tuple_point.z == 3.0);
    }

    #[test]
    fn test_point() {
        let test_point = Tuple::point(2.0, 3.0, 4.0);
        assert!(test_point.w == 1.0);
        //assert!(test_point.point_check());
    }

    #[test]
    fn test_vector() {
        let test_vector = Tuple::vector(3.0, 9.0, -3.0);
        assert!(test_vector.x == 3.0);
        assert!(test_vector.y == 9.0);
        assert!(test_vector.z == -3.0);
        assert!(test_vector.w == 0.0);
        //assert!(test_vector.vector_check());
    }

    #[test]
    fn test_add_tuples() {
        let tuple_a = Tuple::new(4.0, 3.0, 1.0, 2.0);
        let tuple_b= Tuple::new(2.0, 3.0, 9.0, 1.0);
        let tuple_ab = tuple_a + tuple_b;

        assert!(tuple_ab.x == 6.0);
        assert!(tuple_ab.y == 6.0);
        assert!(tuple_ab.z == 10.0);
        assert!(tuple_ab.w == 3.0);
    }

    #[test]
    fn test_sub_tuples() {
        let tuple_a = Tuple::new(3.0, 2.0, 4.0, 2.0);
        let tuple_b= Tuple::new(1.0, 1.0, 1.0, 1.0);
        let tuple_ab = tuple_a - tuple_b;

        assert!(tuple_ab.x == 2.0);
        assert!(tuple_ab.y == 1.0);
        assert!(tuple_ab.z == 3.0);
        assert!(tuple_ab.w == 1.0);
    }

        // Test operation overload negate.
        // Here we copy the value of -tuple_a into a new tuple, is this the way to do it?
        // Seems smart so we use the same adress, otherwise we keep making copies while never using the old stuff.
    #[test]
    fn test_negating_tuples() {
        let tuple_a = Tuple::new(3.0, -2.0, 5.0, -7.0);
        let tuple_a_neg = -tuple_a;

        assert!(tuple_a_neg.x == -3.0);
        assert!(tuple_a_neg.y ==  2.0);
        assert!(tuple_a_neg.z == -5.0);
        assert!(tuple_a_neg.w ==  7.0);
    }

    #[test]
    fn test_scalar_tuples(){
        let mut tuple_a = Tuple::new(2.0, 3.0, -1.0, 6.0);
        tuple_a = tuple_a * 3.0;

        assert!(tuple_a.x == 3.0 *  2.0);
        assert!(tuple_a.y == 3.0 *  3.0);
        assert!(tuple_a.z == 3.0 * -1.0);
        assert!(tuple_a.w == 3.0 *  6.0);
    }

    #[test]
    fn test_div_tuples(){
        let mut tuple_a = Tuple::new(3.0, 9.0, 12.0, -21.0);
        tuple_a = tuple_a / 3.0;

        assert!(tuple_a.x ==  3.0 / 3.0);
        assert!(tuple_a.y ==  9.0 / 3.0);
        assert!(tuple_a.z ==  12.0 / 3.0);
        assert!(tuple_a.w == -21.0 / 3.0);
    }

    #[test]
    fn test_mag_vector(){
        let vector_v = Tuple::vector(0.0, 1.0, 0.0);
        let x = f64::powi(0.0, 2);
        let y = f64::powi(1.0, 2);
        let z = f64::powi(0.0, 2);
        let magnitude_v = f64::sqrt(x+y+z);
        let vector_w = vector_v.magnitude();

        assert!(vector_w == magnitude_v);
    }
        
    #[test]
    fn test_norm_vector(){
        let tuple_one = Tuple::vector(4.0, 0.0, 0.0);
        let norm_vec = tuple_one.norm();
        let norm_x: f64 = 4.0 / tuple_one.magnitude();
        let norm_y: f64 = 0.0 / tuple_one.magnitude();
        let norm_z: f64 = 0.0 / tuple_one.magnitude();
        
        assert!(norm_x == norm_vec.x);
        assert!(norm_y == norm_vec.y);
        assert!(norm_z == norm_vec.z);
    }

    #[test]
    fn test_dot_product(){
        let vec_one = Tuple::vector(1.0, 2.0, 3.0);
        let vec_two = Tuple::vector(2.0, 3.0, 4.0);
            
        let dot_answer: f64 = 20.0;
        let dot_computed: f64 = vec_one.dot(vec_two);
        assert!(dot_answer == dot_computed);
    }

    #[test]
    fn test_cross_product(){
        let vec_one = Tuple::vector(1.0, 2.0, 3.0);
        let vec_two = Tuple::vector(2.0, 3.0, 4.0);
       
        let real_result = Tuple::vector(-1.0, 2.0, -1.0);
        
        let cross_computed = vec_one.cross(vec_two);
        println!("{:?}", real_result);
        println!("{:?}", cross_computed); 
        assert!(real_result.x == cross_computed.x);
        assert!(real_result.y == cross_computed.y);
        assert!(real_result.z == cross_computed.z);
    }
}