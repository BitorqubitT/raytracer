use std::ops::{Index, IndexMut, Mul};
use std::cmp::PartialEq;

use crate::tuple::Tuple;
crate::tuple::Tuple::{Mul}; 

// Questions:
// Functions need to be public to use them in ch03, but is this the only reason
// Implement Eq for comparing matrices -> what is the difference: ops, cmp
// Implementing matrix comparison can be done by using partialEq -> how does this really work?

// TODO:
// Test M mul for different dimensions
// Implement M * tuple. use my own struct. How to creae impl for m * tuple
// With my current implementation of matrix I have to supply data
// Can also put the values in vec<vec>> and the convert to matrix
// Check types, might need to change 
// Matrix mul speed?


#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<i32>>,
    }

// Looks like its better to create matrices from native types
impl Matrix {

    // f64 instead of i32???
    pub fn new(width: usize, height: usize, data: Vec<Vec<i32>>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }

    pub fn display(&self) {
        for row in &self.data {
            println!("{:?}", row);
        }
   } 

}

impl Index<usize> for Matrix {
    type Output = Vec<i32>;

    fn index(&self, index: usize) -> &Vec<i32> {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Vec<i32> {
        &mut self.data[index]
    }
}

impl Mul for Matrix {

    type Output = Self;

    fn mul(self, other: Matrix) -> Self {

        // Check if matrix dimensins allow multiplication.
        // Is this the right way of handling errors?
        if self.width != other.height {
            panic!("Matrix dimnesions are not compatible.");
        }
        
        // For now we create a matrix to make the impl work.
        // Should find better solution see TDDO
        let matrix_values: Vec<Vec<i32>> = vec![vec![0; self.width]; other.height];
        let mut result = Matrix::new(self.width, other.height, matrix_values);

        for i in 0..self.height {
            for j in 0..self.width {
                
                // Type is i32 for now, don't we need flt64 for matrix?
                let mut sum= 0;
               // Matrix multiplication, cant this be a seperate function?
                for k in 0..self.height {

                    // Something with more speed? 
                    sum += self.data[i][k] * other.data[k][j];
                }
                // y, x
                result.data[i][j] = sum;
            }
        }
    // dont need the return, whats the diff?
       return result
    }

    // No idea what to do here.
    // Can use tuple or just 1d matrix.
    // What is the play here.
    // Can we mul m by tuple
    // Should I use the tuple struct?
    // whats a logical implementation?

    fn mul(self, other: Tuple) -> Self {





    }



}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2d_matrix(){
        let matrix_values = vec![
            vec![-3, 5],
            vec![1, -2],
        ];

        let new_matrix = Matrix::new(2, 2, matrix_values);

        assert!(new_matrix[0][0] == -3);
        assert!(new_matrix[0][1] == 5);
        assert!(new_matrix[1][0] == 1);
        assert!(new_matrix[1][1] == -2);
    }

    #[test]
    fn test_3d_matrix(){
        
        let matrix_values = vec![
            vec![-3, 5, 0],
            vec![1, -2, -7],
            vec![0, 1, 1],
        ];

        let new_matrix = Matrix::new(3, 3, matrix_values);

        assert!(new_matrix[0][0] == -3);
        assert!(new_matrix[1][1] == -2);
        assert!(new_matrix[2][2] == 1);
    }

    #[test]
    // Should make it work 100%, does this mean with floating points aswell?
    fn compare_matrices(){

        let matrix_values = vec![
            vec![-3, 5, 0],
            vec![1, -2, -7],
            vec![0, 1, 1],
        ];
    
        let matrix_values_2 = vec![
            vec![-1, 5, 0],
            vec![1, -2, -7],
            vec![0, 1, 1],
        ];

        let matrix_one = Matrix::new(3, 3, matrix_values);
        let matrix_two = Matrix::new(3, 3, matrix_values_2);

        assert!(matrix_one == matrix_one);
        assert!(matrix_one != matrix_two);
    }

    #[test]
    // should look at how i implemented multiplication for canvas
    fn multiply_matrices(){

        let matrix_values_a = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 8, 7, 6],
            vec![5, 4, 3, 2]
        ];

        let matrix_values_b = vec![
            vec![-2, 1, 2, 3],
            vec![3, 2, 1, -1],
            vec![4, 3, 6, 5],
            vec![1, 2, 7, 8]
        ];

        let matrix_values_c = vec![
            vec![20, 22, 50, 48],
            vec![44, 54, 114, 108],
            vec![40, 58, 110, 102],
            vec![16, 26, 46, 42]
        ];

        let matrix_a = Matrix::new(4, 4, matrix_values_a);
        let matrix_b = Matrix::new(4, 4, matrix_values_b);
        let matrix_c = Matrix::new(4, 4, matrix_values_c);

        assert!(matrix_a * matrix_b == matrix_c);
    }


    #[test]
    fn multiply_matrix_by_tuple(){

        let matrix_values_a = vec![
            vec![1, 2, 3, 4],
            vec![2, 4, 4, 2],
            vec![8, 6, 4, 1],
            vec![0, 0, 0, 1]
        ];


        let mut tuple_a = Tuple::new(2.0, 3.0, -1.0, 6.0);


    }



}
