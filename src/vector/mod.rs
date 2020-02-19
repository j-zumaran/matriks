
pub fn new(size: usize) -> Vec<i32> {
    Vec::with_capacity(size)
}

pub fn equal_size(vec1: &Vec<i32>, vec2: &Vec<i32>) -> bool {
    vec1.len() == vec2.len()
}

pub fn sum(vec1: &Vec<i32>, vec2: &Vec<i32>)
        -> Result<Vec<i32>, String> {

    if equal_size(vec1, vec2) {
        let mut result = new(vec1.len());

        for i in 0..result.capacity() {
            let sum = vec1.get(i).unwrap()
                    + vec2.get(i).unwrap();
            result.push(sum);
        }
        return Ok(result)
    }

    Err(format!("Vectors are not the same size
            -> 1: {}, 2: {}",
        vec1.len(), vec2.len()))
}

pub fn diff(min: &Vec<i32>,subs: &Vec<i32>)
        -> Result<Vec<i32>, String> {

    sum(min, &scalar_mult(subs, -1))
}

pub fn scalar_mult(vec: &[i32], scalar: i32) -> Vec<i32> {
    let mut res = new(vec.len());
    for val in vec {
        res.push(val * scalar)
    }
    res
}

pub mod identity {
    //use crate::vector::new;

    //pub fn j_hat(dim: usize) -> Vec<i32> {
    //    Vec::from(s: &mut [T])
    //}
}
