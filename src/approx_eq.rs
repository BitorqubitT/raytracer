use std::f64;

// Use this to check if certain numbers are equal up to x decimal points.
pub trait ApproxEq {
    fn fuzzy_eq(&self, other: f64) -> bool;
}

impl ApproxEq for f64 {
    fn fuzzy_eq(&self, other: f64) -> bool{
        let epsilon = 1e-1;
        (*self - other).abs() < epsilon
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