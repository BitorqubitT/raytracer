use std::f64;

use crate::tuple::Tuple;

// Use this to check if certain numbers are equal up to x decimal points.
// <T> defines a generic type
pub trait ApproxEq<T> {
    fn fuzzy_eq(&self, other: T) -> bool;
}

impl ApproxEq<f64> for f64 {
    fn fuzzy_eq(&self, other: f64) -> bool{
        let epsilon = 1e-1;
        (*self - other).abs() < epsilon
    }
}

// Use & to pass a reference, aka a point to the value in stead of the value
// Then I need to also pass the reference in the tests
// impl ApproxEq<&Tuple> for Tuple{
// fn fuzzy_eq(&self, other: &Tuple) -> bool {

impl ApproxEq<Tuple> for Tuple{
    fn fuzzy_eq(&self, other: Tuple) -> bool {
        let epsilon = 1e-1;
        (self.x - other.x).abs() < epsilon &&
        (self.y - other.y).abs() < epsilon &&
        (self.z - other.z).abs() < epsilon
        
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_approx_equality(){
        
        let b: f64 = 0.21804511278195488;
        let a: f64 = 0.21805;

        assert!(a.fuzzy_eq(b));
        assert!(a.fuzzy_eq(b));
    }
}