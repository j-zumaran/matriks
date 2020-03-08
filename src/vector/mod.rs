
pub fn new(size: usize) -> Vec<i32> {
    Vec::with_capacity(size)
}

pub fn equal_size(vec1: &[i32], vec2: &[i32]) -> Result<(), String> {
    if vec1.len() == vec2.len() {
        return Ok(());
    }
    Err(format!("Vectors are not the same size -> 1: {}, 2: {}",
        vec1.len(), vec2.len()))
}

pub fn sum(vec1: &[i32], vec2: &[i32]) -> Result<Vec<i32>, String> {

    equal_size(vec1, vec2)?;

    let mut iter_2 = vec2.iter();
    Ok(vec1.iter().map(|e| {
        let mut x = 0;
        if let Some(val) = iter_2.next() {
            x = e + val;
        };
        x
    })
    .collect::<Vec<i32>>())
}

pub fn diff
        (min: &[i32], subs: &[i32]) -> Result<Vec<i32>, String> {
    sum(min, &scalar_mult(subs, -1))
}

pub fn scalar_mult
        (vec: &[i32], scalar: i32) -> Vec<i32> {
    vec.iter()
        .map(|e| { let x = e * scalar; x })
        .collect::<Vec<i32>>()
}

pub fn dot_prod
    (vec1: &[i32], vec2: &[i32]) -> Result<i32, String> {

    equal_size(vec1, vec2)?;

    let mut iter_2 = vec2.iter();

    Ok(vec1.iter().map(|e| {
        let mut x = 0;
        if let Some(val) = iter_2.next() {
            x = e * val;
        }
        return x
    })
    .sum())
}

pub mod identity {
    //use crate::vector::new;

    //pub fn j_hat(dim: usize) -> Vec<i32> {
    //    Vec::from(s: &mut [T])
    //}
}
