use std::{
    fmt::Display,
    ops::{Add, Mul},
};

fn horner_eval<T>(x: T, coefficent_sequence: &[T]) -> T
where
    T: From<usize> + Add + Add<Output = T> + Mul + Mul<Output = T> + Copy + Display,
{
    coefficent_sequence
        .iter()
        .rfold(0.into(), |higher_terms, &this_coeff| {
            higher_terms * x + this_coeff
        })
}

fn main() {
    println!("{}", horner_eval(2, &[1, 3, 0, 5, 0, 1]));
}
#[test]
fn test() {
    main();
}
