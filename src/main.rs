mod matrix;
mod vector;

use matrix::Matrix;

fn main() {
    println!("matrix id: {:?}\n", Matrix::id(3));

    println!("det id: {:?}\n", Matrix::id(3).det());

    let mat1 = Matrix::rand_sqr(3);

    println!("matrix 1: {:?}\n", mat1);

    println!("det 1: {:?}\n", mat1.det());

    //println!("transposed 1: {:?}\n", mat1.transpose());

    let mat2 = Matrix::rand_sqr(3);

    println!("matrix 2: {:?}\n", mat2);

    println!("det 2: {:?}\n", mat2.det());

    //println!("transposed 1: {:?}\n", mat2.transpose());

    println!("sum: {:?}\n", Matrix::sum(&mat1, &mat2));

    println!("diff: {:?}\n", Matrix::diff(&mat1, &mat2));

    println!("dot: {:?}\n", Matrix::dot_prod(&mat1, &mat2));

    for _i in  0..5 {
        println!("rand: {:?}\n", Matrix::rand());
    }

}
