use std::ops::{Index, IndexMut, Mul};
use std::cmp::{min, PartialEq};

use crate::tuple::Tuple;

// Current:
// Check if minor implementation makes sense, rustwise
// Implement cofactors

// Questions:
// Functions need to be public to use them in ch03, but is this the only reason
// I already have a lib which makes "script public" or just the implementation?
// Implement Eq for comparing matrices -> what is the difference: ops, cmp
// Does the implementation of minor make sense, when should I implement it as part of matrix and when shouldnt I?

// TODO:
// Use my own struct. How to create impl for m * tuple
// Matrix mul speed?
// WHy cant I use the trait COPY in matrix?
// Check indentation of the returns in most functions.

// Change:
// Check when to use &, without you consume the variable.
// When should functions be pub in rust?
// M * T, works, but is this the right implementation for this raytracer, what if we use diff sizes?

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    // CHange this
    // width and height should just check the data
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

   // When transposing, do we want to keep the old one aswell?
   // I loop, but I can also hardcode it per n*m, this would be faster.
   // Check output type.
   pub fn transpose(&self) -> Self {
        let mut transposed_matrix_values = vec![];

        for j in 0..self.width {
            // let mut row = Vec::with_capacity(self.height + 1); // capacity set for better performance
            let mut row = vec![];    
            for i in 0..self.height {
                row.push(self.data[i][j])
            }
        transposed_matrix_values.push(row);
        }
    
    let transposed_matrix = Matrix::new(self.width, self.height, transposed_matrix_values);
    return transposed_matrix
   }

   pub fn determinant(&self) -> f64 {
        
        if self.width == 2 || self.height == 2 {
            let determinant = self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]; 

            return determinant
        }
        else {
           let mut determinant = 0.0;

           for j in 0..self.width {
                determinant = determinant + (self[0][j] * self.cofactor(0, j));

           } 
            return determinant
        }

   }

   pub fn submatrix(&self, row: usize, column: usize) -> Self {

    // Better way to slice?
    // Better way than iterating?
    // is this memory efficient?
    // Do we "destroy" old matrix, otherwise we might keep too many in memory for now reason

        let mut matrix_values = vec![];

        for i in 0..self.height {
            // let mut row = Vec::with_capacity(self.height + 1); // capacity set for better performance
            let mut new_row = vec![];

            for j in 0..self.width {
                // When we see that column or row is equal we go next
                if j == column || i == row {
                    continue
                } else {
                    new_row.push(self.data[i][j])
                }
            }
            if new_row.len() != 0 {
                matrix_values.push(new_row);
            }
        }

    let submatrix = Matrix::new(self.width - 1, self.height -1, matrix_values);
    return submatrix
   }

    // get minor
    // Is this the best way to implement?
    // Should it be a part of matrix?
    pub fn minor(&self, row: usize, column:usize) -> f64 {

        let submatrix = self.submatrix(row, column);
        let determinant_a = submatrix.determinant();

        return determinant_a
   }

   pub fn cofactor(&self, row: usize, column:usize) -> f64 {

        let minor_a = self.minor(row, column);

        // Weird there are no type errors?
        // We can just add usizes? and test devision by 2, without it being an flt.
        if (row + column) % 2 == 0 {
            return minor_a
        } 
        let cofactor_a = minor_a * -1.0;
        return cofactor_a
   }

   pub fn inverse(&self) -> Matrix {
     // First check the determinant
     // if det == 0 then return warning -> cant invert this matrix

    // Change size of matrix
    // change order???

        let mut matrix_values = vec![];

        for i in 0..self.height - 1{
            let mut new_row = vec![];

            for j in 0..self.width - 1{
                let c = self.cofactor(i, j);
                new_row.push(c / self.determinant());
            }
            matrix_values.push(new_row);
        }
        let submatrix = Matrix::new(self.width, self.height, matrix_values);
        return submatrix
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
}

impl Mul<Tuple> for Matrix {
    type Output =  Tuple;
    // For now we go with M * T -> T
    // And I use my own tuple impl
    // Might have to change this later.
    // This impl only works for 4,4 * 1,4
    fn mul(self, other: Tuple) -> Tuple {

        let mut new_tuple = Tuple::new(0.0, 0.0, 0.0, 0.0);

        for i in 0..self.height {
            let mut sum = 0.0;
            for j in 0..self.width {

                match j {
                0 => sum += self.data[i][j] * other.x,
                1 => sum += self.data[i][j] * other.y,
                2 => sum += self.data[i][j] * other.z,
                3 => sum += self.data[i][j] * other.w,
                _ => panic!("Matrix dimensions are not compatible")
                }

            match i {
            0 => new_tuple.x = sum,
            1 => new_tuple.y = sum,
            2 => new_tuple.z = sum,
            3 => new_tuple.w = sum,
            _ => panic!("Matrix dimensions are not compatible")
            }
            }
        }
    return new_tuple
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
    // Test works, check if this is still the right implementation in the future
    fn multiply_matrix_by_tuple(){
       
        let matrix_values_a = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ];

        let tuple_a = Tuple::new(2.0, 3.0, -1.0, 6.0);
        let matrix_a = Matrix::new(4, 4, matrix_values_a);
        let tuple_b = Tuple::new(29.0, 24.0, 36.0, 6.0);
        assert!(matrix_a * tuple_a == tuple_b);
    }

    #[test]
    // Should find a way to clone or copy matrix_values_a or matrix_a
    // We dont consume the struct so should be fine
    fn multiply_matrix_by_identitymatrix(){

        let matrix_values_a = vec![
            vec![0.0, 1.0, 2.0, 4.0],
            vec![1.0, 2.0, 4.0, 8.0],
            vec![2.0, 4.0, 8.0, 16.0],
            vec![4.0, 8.0, 16.0, 32.0],
        ];

        let matrix_values_b = vec![
            vec![0.0, 1.0, 2.0, 4.0],
            vec![1.0, 2.0, 4.0, 8.0],
            vec![2.0, 4.0, 8.0, 16.0],
            vec![4.0, 8.0, 16.0, 32.0],
        ];

        let matrix_values_i = vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ];

        let matrix_a = Matrix::new(4, 4, matrix_values_a);
        let matrix_b = Matrix::new(4, 4, matrix_values_b);
        let matrix_i = Matrix::new(4, 4, matrix_values_i);

        assert!(matrix_i * matrix_a == matrix_b)
    }

    #[test]
    fn multiply_identitymatrix_by_tuple(){

        let matrix_values_i = vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ];

        let tuple_a = Tuple::new(1.0, 2.0, 3.0, 4.0);
        let tuple_b = Tuple::new(1.0, 2.0, 3.0, 4.0);
        let matrix_i = Matrix::new(4, 4, matrix_values_i);

        assert!(matrix_i * tuple_a == tuple_b);
    }

    #[test]
    fn transpose_matrix(){

        let matrix_values_a = vec![
            vec![0.0, 9.0, 3.0, 0.0],
            vec![9.0, 8.0, 0.0, 8.0],
            vec![1.0, 8.0, 5.0, 3.0],
            vec![0.0, 0.0, 5.0, 8.0],
        ];

        let matrix_values_b = vec![
            vec![0.0, 9.0, 1.0, 0.0],
            vec![9.0, 8.0, 8.0, 0.0],
            vec![3.0, 0.0, 5.0, 5.0],
            vec![0.0, 8.0, 3.0, 8.0],
        ];

        let matrix_a = Matrix::new(4, 4, matrix_values_a);
        let matrix_b = Matrix::new(4, 4, matrix_values_b);
        let transposed_matrix = matrix_a.transpose();
        assert!(transposed_matrix == matrix_b);
    }

    #[test]
    fn transpose_identitymatrix(){

        let matrix_values_i = vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ];

        let matrix_i = Matrix::new(4, 4, matrix_values_i);
        let transposed_matrix = matrix_i.transpose();
        assert!(transposed_matrix == matrix_i);
    }

    #[test]
    fn calculate_determinant(){

        let matrix_values_a = vec![
            vec![1.0, 5.0],
            vec![-3.0, 2.0],
        ];

        let matrix_a = Matrix::new(2, 2, matrix_values_a);
        let determinant = 17.0;
     
        assert!(matrix_a.determinant() == determinant);
    }

    // submatrux(M, removerow, reyymovecolumn)
    // maybe implement this for all sizes?
    #[test]
    fn get_submatrix(){

       let matrix_values_a = vec![
            vec![1.0, 5.0, 0.0],
            vec![-3.0, 2.0, 7.0],
            vec![0.0, 6.0, -3.0],
        ];

       let sub_matrix_values_a = vec![
            vec![-3.0, 2.0],
            vec![0.0, 6.0],
        ];

       let matrix_values_b = vec![
            vec![-6.0, 1.0, 1.0, 6.0],
            vec![-8.0, 5.0, 8.0, 6.0],
            vec![-1.0, 0.0, 8.0, 2.0],
            vec![-7.0, 1.0, -1.0, 1.0],
        ];

       let sub_matrix_values_b = vec![
            vec![-6.0, 1.0, 6.0],
            vec![-8.0, 8.0, 6.0],
            vec![-7.0, -1.0, 1.0],
        ];

        let matrix_a = Matrix::new(3, 3, matrix_values_a);
        let sub_matrix_a = Matrix::new(2, 2, sub_matrix_values_a);
        let matrix_b = Matrix::new(4, 4, matrix_values_b);
        let sub_matrix_b = Matrix::new(3, 3, sub_matrix_values_b);

        assert!(sub_matrix_a == matrix_a.submatrix(0, 2));
        assert!(sub_matrix_b == matrix_b.submatrix(2, 1));
    
    }

    // We calculate the minors    
    #[test]
    fn get_minor(){

       let matrix_values_a = vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ];

        let matrix_a = Matrix::new(3, 3, matrix_values_a);

        assert!(25.0 == matrix_a.minor(1, 0));
    }

    #[test]
    fn get_cofacts(){
       
       let matrix_values_a = vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ];

        let matrix_a = Matrix::new(3, 3, matrix_values_a);

        assert!(-12.0 == matrix_a.cofactor(0, 0));
        assert!(-25.0 == matrix_a.cofactor(1, 0));

    }

    #[test]
    fn calculate_determinant_of_3x3(){
    
       let matrix_values_a = vec![
            vec![1.0, 2.0, 6.0],
            vec![-5.0, 8.0, -4.0],
            vec![2.0, 6.0, 4.0],
        ];

        let matrix_a = Matrix::new(3, 3, matrix_values_a);
       
        assert!(56.0 == matrix_a.cofactor(0, 0));
        assert!(12.0 == matrix_a.cofactor(0, 1));
        assert!(-46.0 == matrix_a.cofactor(0, 2));
        assert!(-196.0 == matrix_a.determinant());

    }

    #[test]
    fn calculate_determinant_of_4x4(){
       let matrix_values_a = vec![
            vec![-2.0, -8.0, 3.0, 5.0],
            vec![-3.0, 1.0, 7.0, 3.0],
            vec![1.0, 2.0, -9.0, 6.0],
            vec![-6.0, 7.0, 7.0,-9.0],
        ];
        
        let matrix_a = Matrix::new(4, 4, matrix_values_a);
       
        assert!(690.0 == matrix_a.cofactor(0, 0));
        assert!(447.0 == matrix_a.cofactor(0, 1));
        assert!(210.0 == matrix_a.cofactor(0, 2));
        assert!(51.0 == matrix_a.cofactor(0, 3));
        assert!(-4071.0 == matrix_a.determinant());

    }

    #[test]
    fn calculate_inverse(){
       let matrix_values_a = vec![
            vec![-5.0, 2.0, 6.0, -8.0],
            vec![1.0, -5.0, 1.0, 8.0],
            vec![7.0, 7.0, -6.0, -7.0],
            vec![1.0, -3.0, 7.0, 4.0],
        ];

       let matrix_values_b = vec![
            vec![0.21805, 0.45113, 0.24060, -0.04511],
            vec![-0.80827, -1.45677, -0.44361, 0.52068],
            vec![-0.07895, -0.22368, -0.05263, 0.19737],
            vec![-0.52256, -0.81391, 0.30075, 0.30639],
        ];

        let matrix_a = Matrix::new(4, 4, matrix_values_a);
        let matrix_b = Matrix::new(4, 4, matrix_values_b);


        println!("{:?}", matrix_a.inverse());

        assert!(matrix_b == matrix_a.inverse());
        assert!(532.0 == matrix_a.determinant());
        assert!(-160.0 == matrix_a.cofactor(2, 3));
        assert!(-160.0 / 532.0 == matrix_b[3][2]);
        assert!(105.0 == matrix_a.cofactor(3, 2));
        assert!(105.0 / 532.0 == matrix_b[2][3]);
    }




}
