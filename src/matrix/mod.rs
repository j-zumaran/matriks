use crate::vector;
use rand::Rng;

#[derive(Debug)]
pub struct Matrix {
    array: Vec<Vec<i32>>,
    pub n_rows: usize,
    pub n_cols: usize,
}

impl Matrix {

    pub fn is_sqr(&self) -> bool {
        self.n_rows == self.n_cols
    }

    pub fn row(&self, index: usize) -> Option<&Vec<i32>> {
        self.array.get(index)
    }

    pub fn rows(&self) -> &Vec<Vec<i32>> {
        &self.array
    }

    pub fn col(&self, index: usize) -> Vec<i32> {
        let mut res = vector::new(self.n_rows);

        for row in self.rows() {
            if let Some(item) = row.get(index) {
                res.push(item.clone());
            }
        }
        res
    }

    pub fn cols(&self) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> =
            Vec::with_capacity(self.n_cols);

        for col in 0..result.capacity() {
            result.push(self.col(col));
        }
        result
    }

    pub fn transpose(&self) -> Matrix {
        Self::from(self.cols())
    }

    pub fn diagonal() -> () {

    }

    pub fn det(&self) -> Result<i32, &'static str> {
        if self.is_sqr() {
            if let Some(vec) = self.array.first() {

            }
        }
        Err("Matrix is not square.")
    }
}

impl Matrix {

    fn empty() -> Matrix {
        Self::from(Self::empty_array(0))
    }

    pub fn from(array: Vec<Vec<i32>>) -> Matrix {
        let (is_empty, n_cols, n_rows) = Self::is_empty(&array);

        if is_empty {
            return Self::empty();
        }
        Matrix {
            array: array,
            n_rows: n_rows,
            n_cols: n_cols,
        }
    }

    pub fn id(sqr_n: usize) -> Matrix {
        let mut array = Self::empty_array(sqr_n);

        for row in 0..sqr_n {
            let mut vec = vector::new(sqr_n);

            for col in 0..sqr_n {
                if col == row {
                    vec.push(1);
                } else {
                    vec.push(0);
                }
            }
            array.push(vec);
        }
        Self::from(array)
    }

    pub fn rand_sqr(size: usize) -> Matrix {
        let mut array = Self::empty_array(size);

        for _row in 0..size {
            let mut vec = vector::new(size);

            for _col in 0..size {
                vec.push(rand::thread_rng().gen_range(-5, 5));
            }
            array.push(vec);
        };
        Self::from(array)
    }
}

impl Matrix {

    pub fn scalar_mult(matrix: &Matrix, scalar: i32) -> Matrix {

        let mut result = Self::empty_array(matrix.n_rows);
        for vec in matrix.rows() {
            result.push(vector::scalar_mult(vec, scalar))
        }
        Self::from(result)
    }

    pub fn sum(m1: &Matrix, m2: &Matrix) -> Result<Matrix, String> {

        if Self::equal_size(m1, m2) {
            let size = m1.n_rows;

            let mut result = Self::empty_array(size);

            for index in 0..size {
                if let Some(vec1) = m1.row(index) {
                    if let Some(vec2) = m2.row(index) {
                        if let Ok(sum) = vector::sum(vec1, vec2) {
                            result.push(sum);
                        }
                    }
                }
            }
            return Ok(Self::from(result));
        }
        Err(format!("Matrices are not the same size:
                        1: {}x{}, 2: {}x{}",
                    m1.n_cols, m1.n_rows,
                    m2.n_cols, m2.n_rows))
    }

    pub fn diff(m1: &Matrix, m2: &Matrix) -> Result<Matrix, String> {
        Self::sum(m1, &Self::scalar_mult(m2, -1))
    }

    pub fn cross_prod() {
        unimplemented!()
    }
}

impl Matrix {

    fn equal_row_col(m1: &Matrix, m2: &Matrix) -> bool {
        m1.n_cols == m2.n_rows
    }

    pub fn equal_size(m1: &Matrix, m2: &Matrix) -> bool {
        m1.n_cols == m2.n_cols
            && m1.n_rows == m2.n_rows
    }

    fn is_empty(array: &Vec<Vec<i32>>) -> (bool, usize, usize) {
        match array.first() {
            Some(vec) => return (vec.is_empty(), array.len(), vec.len()),
            None => return    (true, 0, 0)
        }
    }

    fn empty_array(size: usize) -> Vec<Vec<i32>> {
        Vec::<Vec<i32>>::with_capacity(size)
    }
}
