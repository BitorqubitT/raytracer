use std::ops::{Index, IndexMut, Mul};
use std::cmp::PartialEq;

use crate::tuple::Tuple;

// Questions:
// Functions need to be public to use them in ch03, but is this the only reason
// Implement Eq for comparing matrices -> what is the difference: ops, cmp
// Implementing matrix comparison can be done by using partialEq -> how does this really work?

// TODO:
// Use my own struct. How to create impl for m * tuple
// Matrix mul speed?

// Change:
// make everything float
// M * Tuple = tuple  (how do i deal with different sizes?)



#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<f64>>,
    }

// Looks like its better to create matrices from native types
impl Matrix {

    // f64 instead of i32???
    pub fn new(width: usize, height: usize, data: Vec<Vec<f64>>) -> Self {
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
    type Output = Vec<f64>;

    fn index(&self, index: usize) -> &Vec<f64> {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Vec<f64> {
        &mut self.data[index]
    }
}

impl Mul for Matrix {

    type Output = Self;

    fn mul(self, other: Matrix) -> Self {
        if self.width != other.height {
            panic!("Matrix dimensions are not compatible.");
        }
        
        // For now we create a matrix to make the impl work.
        let matrix_values: Vec<Vec<f64>> = vec![vec![0.0; other.width]; self.height];
        let mut result = Matrix::new(other.width, self.height, matrix_values);

        for i in 0..self.height {
            for j in 0..other.width {
                
                // Type is i32 for now, don't we need flt64 for matrix?
                let mut sum= 0.0;
                for k in 0..self.width {
                    sum += self.data[i][k] * other.data[k][j];
                }
                result.data[i][j] = sum;
            }
        }
    // dont need the return, whats the diff?
       return result
    }

    // whats a logical implementation?
    fn mul(self, other: Tuple) -> other {
            






    }



}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2d_matrix(){
        let matrix_values = vec![
            vec![-3.0, 5.0],
            vec![1.0, -2.0],
        ];

        let new_matrix = Matrix::new(2, 2, matrix_values);

        assert!(new_matrix[0][0] == -3.0);
        assert!(new_matrix[0][1] == 5.0);
        assert!(new_matrix[1][0] == 1.0);
        assert!(new_matrix[1][1] == -2.0);
    }

    #[test]
    fn test_3d_matrix(){
        
        let matrix_values = vec![
            vec![-3.0, 5.0, 0.0],
            vec![1.0, -2.0, -7.0],
            vec![0.0, 1.0, 1.0],
        ];

        let new_matrix = Matrix::new(3, 3, matrix_values);

        assert!(new_matrix[0][0] == -3.0);
        assert!(new_matrix[1][1] == -2.0);
        assert!(new_matrix[2][2] == 1.0);
        assert!(new_matrix[0][1] == 5.0);
    }

    #[test]
    // Should make it work 100%, does this mean with floating points aswell?
    fn compare_matrices(){

        let matrix_values = vec![
            vec![-3.0, 5.0, 0.0],
            vec![1.0, -2.0, -7.0],
            vec![0.0, 1.0, 1.0],
        ];
    
        let matrix_values_2 = vec![
            vec![-1.0, 5.0, 0.0],
            vec![1.0, -2.0, -7.0],
            vec![0.0, 1.0, 1.0],
        ];

        let matrix_one = Matrix::new(3, 3, matrix_values);
        let matrix_two = Matrix::new(3, 3, matrix_values_2);
        assert!(matrix_one == matrix_one);
        assert!(matrix_one != matrix_two);
    }

    #[test]
    fn multiply_matrices_2by2(){

        let matrix_values_a = vec![
            vec![2.0, 5.0],
            vec![9.0, 51.0],
        ];

        let matrix_values_b = vec![
            vec![6.0, 3.0],
            vec![1.0, 81.0],
        ];

        let matrix_values_c = vec![
            vec![17.0, 411.0],
            vec![105.0, 4158.0],
        ];

        let matrix_a = Matrix::new(2, 2, matrix_values_a);
        let matrix_b = Matrix::new(2, 2, matrix_values_b);
        let matrix_c = Matrix::new(2, 2, matrix_values_c);

        assert!(matrix_a * matrix_b == matrix_c);
    }

    #[test]
    fn multiply_matrices_3_2_by_2_3(){

        let matrix_values_a = vec![
            vec![2.0, 0.0],
            vec![1.0, 11.0],
            vec![955.0, 51.0],
        ];

        let matrix_values_b = vec![
            vec![6.0, 3.0, 1.0],
            vec![1.0, 81.0, 9.0],
        ];

        let matrix_values_c = vec![
            vec![12.0, 6.0, 2.0],
            vec![17.0, 894.0, 100.0],
            vec![5781.0, 6996.0, 1414.0],
        ];

        // check what is the correct input
        // maybe add a check
        let matrix_a = Matrix::new(2, 3, matrix_values_a);
        let matrix_b = Matrix::new(3, 2, matrix_values_b);
        let matrix_c = Matrix::new(3, 3, matrix_values_c);

        assert!(matrix_a * matrix_b == matrix_c);
    }

    #[test]
    // should look at how i implemented multiplication for canvas
    fn multiply_matrices_4by4(){

        let matrix_values_a = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0]
        ];

        let matrix_values_b = vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0]
        ];

        let matrix_values_c = vec![
            vec![20.0, 22.0, 50.0, 48.0],
            vec![44.0, 54.0, 114.0, 108.0],
            vec![40.0, 58.0, 110.0, 102.0],
            vec![16.0, 26.0, 46.0, 42.0]
        ];

        let matrix_a = Matrix::new(4, 4, matrix_values_a);
        let matrix_b = Matrix::new(4, 4, matrix_values_b);
        let matrix_c = Matrix::new(4, 4, matrix_values_c);

        assert!(matrix_a * matrix_b == matrix_c);
    }

    #[test]
    fn multiply_matrices_6by6(){

        let matrix_values_a = vec![
            vec![1.0, 2.0, 3.0, 4.0, 9.0, 1.0],
            vec![5.0, 6.0, 7.0, 8.0, 1.0, 2.0],
            vec![9.0, 8.0, 7.0, 6.0, 3.0, 5.0],
            vec![5.0, 4.0, 3.0, 2.0, 6.0, 7.0],
            vec![1.0, 82.0, 7.0, 1.0, 3.0, 5.0],
            vec![9.0, 8.0, 7.0, 5.0, 3.0, 5.0],
        ];

        let matrix_values_b = vec![
            vec![-2.0, 1.0, 2.0, 3.0, 9.0, 1.0],
            vec![3.0, 2.0, 1.0, -1.0, 44.0, 2.0],
            vec![4.0, 3.0, 6.0, 5.0, 15.0, 1.0],
            vec![1.0, 2.0, 1.0, 8.0, 1.0, 4.0],
            vec![1.0, 1.0, 6.0, 82.0, 45.0, 4.0],
            vec![13.0, 6.0, 7.0, 1.0, 5.0, 4.0],
            ];

        let matrix_values_c = vec![
            vec![42.0, 37.0, 87.0, 787.0, 556.0, 64.0],
            vec![71.0, 67.0, 86.0, 192.0, 477.0, 68.0],
            vec![108.0, 91.0, 127.0, 353.0, 704.0, 88.0],
            vec![113.0, 74.0, 119.0, 541.0, 573.0, 76.0],
            vec![341.0, 221.0, 180.0, 215.0, 3883.0, 208.0],
            vec![107.0, 89.0, 126.0, 345.0, 703.0, 84.0],
        ];


        let matrix_a = Matrix::new(6, 6, matrix_values_a);
        let matrix_b = Matrix::new(6, 6, matrix_values_b);
        let matrix_c = Matrix::new(6, 6, matrix_values_c);

        assert!(matrix_a * matrix_b == matrix_c);
    }


    #[test]
    fn multiply_matrix_by_tuple(){

        let matrix_values_a = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ];

        let mut tuple_a = Tuple::new(2.0, 3.0, -1.0, 6.0);
        let matrix_a = Matrix::new(4, 4, matrix_values_a);
        let tuple_b = Tuple::new(29.0, 24.0, 36.0, 6.0);

        assert!(matrix_a * tuple_a == tuple_b);

    }



}
