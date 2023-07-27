use std::ops::{Index, IndexMut};

#[macro_use]
// Looks like its better to create matrices from native types
// Can use nalgebra, but this will prob make it slower
// Should also look at canvas implementation
// Check if I want to use matrices this way

// Functions need to be public to use them in ch03, but is this the only reason

// SHould I use i32?
// Any other way to do this?
// Seems pretty clean, but maybe i got more options
// Easy way to create, n*m 0 matrix maybe?
#[derive(Debug, Clone)]
pub struct Matrix {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<i32>>,
    }

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



}
