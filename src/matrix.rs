use std::ops::{Index, IndexMut};
use std::cmp::Eq;


#[macro_use]

// Questions:
// Functions need to be public to use them in ch03, but is this the only reason
// SHould I use i32?
// Any other way to do this?
// Seems pretty clean, but maybe i got more options
// Easy way to create, n*m 0 matrix maybe?
// Implement Eq for comparing matrices -> what is the difference: ops, cmp



#[derive(Debug, Clone)]
pub struct Matrix {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<i32>>,
    }

// Looks like its better to create matrices from native types
impl Matrix {

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

impl Eq for Matrix {

    fn Eq(self, Eq: bool) -> 


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
        let matrix_three = Matrix::new(3, 3, matrix_values);

        assert!(matrix_one != matrix_one);
        assert!((matrix_one == matrix_three));

    }


    // should look at how i implemented multiplication for canvas
    fn multiply_matrices(){



    }


}
