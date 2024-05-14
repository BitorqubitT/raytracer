use crate::EPSILON;
use std::f64;

// Use this to check if certain numbers are equal up to x decimal points.
//const EPSILON: f64 = 1e-6;

trait ApproxEq {
    fn fuzzy_eq(&self, other: f64) -> bool;
}

impl ApproxEq for f64 {
    fn fuzzy_eq(&self, other: f64) -> bool{
        (*self - other).abs() < EPSILON 
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_approx_equality(){
        
        let a: f64 = 0.3334;
        let b: f64 = 0.3334;

        assert(a.fuzzy_eq(b));
    }


}