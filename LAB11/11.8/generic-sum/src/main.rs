use std::convert::Into;
use std::iter::Sum;
use std::ops::Add;

// Generic function to calculate the sum of elements in a vector
fn sum<T>(vec: &[T]) -> T
where
    T: Add<Output = T> + Sum + Copy + Default,
{
    vec.iter().copied().sum()
}

// Generic function to calculate the average of elements in a vector
fn average<T>(vec: &[T]) -> f64
where
    T: Add<Output = T> + Sum + Copy + Into<f64> + Default,
{
    let sum: T = vec.iter().copied().sum();
    let count = vec.len();
    sum.into() / count as f64
}

fn main() {
    let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let v2 = vec![1.2, 28.1, 1.2, 9.1];

    println!("Sum of v1: {}", sum(&v1));
    println!("Sum of v2: {}", sum(&v2));

    println!("Average of v1: {}", average(&v1));
    println!("Average of v2: {}", average(&v2));
}

