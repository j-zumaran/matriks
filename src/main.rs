mod matrix;
mod vector;

use matrix::Matrix;

fn main() {
    let mat1 = Matrix::rand_sqr(3);

    println!("matrix 1: {:?}", mat1);

    let mat2 = Matrix::rand_sqr(2);

    println!("matrix 2: {:?}", mat2);

    let result = Matrix::sum(&mat1, &mat2);

    println!("sum: {:?}", result);

    println!("diff: {:?}", Matrix::diff(&mat1, &mat2));

    println!("id: {:?}", Matrix::id(3));
}
