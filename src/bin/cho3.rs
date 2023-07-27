extern crate raytracer;
use raytracer::matrix::*;

fn main() {
    let matrix_values = vec![
        vec![1, 2, 3],
        vec![1, 2, 3],
        vec![1, 2, 3],
    ];

    let mut new_matrix = Matrix::new(3, 3, matrix_values);

    new_matrix.display();

    new_matrix[0][0] = 99;

    new_matrix.display();



}