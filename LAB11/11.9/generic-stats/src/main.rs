use std::convert::Into;
use std::iter::Sum;
use std::ops::{Add, Sub};

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
    let sum: T = sum(vec);
    let count = vec.len();
    sum.into() / count as f64
}

// Generic function to calculate the variance of elements in a vector
fn variance<T>(vec: &[T]) -> f64
where
    T: Add<Output = T> + Sum + Copy + Into<f64> + Sub<Output = T> + Default,
{
    let mean = average(vec);
    let sum_of_squared_diffs: f64 = vec
        .iter()
        .map(|&value| {
            let diff = value.into() - mean;
            diff * diff
        })
        .sum();
    sum_of_squared_diffs / vec.len() as f64
}

// Generic function to calculate the standard deviation of elements in a vector
fn standard_deviation<T>(vec: &[T]) -> f64
where
    T: Add<Output = T> + Sum + Copy + Into<f64> + Sub<Output = T> + Default,
{
    variance(vec).sqrt()
}

fn main() {
    let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let v2 = vec![1.2, 28.1, 1.2, 9.1];

    println!("Sum of v1: {}", sum(&v1));
    println!("Sum of v2: {}", sum(&v2));

    println!("Average of v1: {}", average(&v1));
    println!("Average of v2: {}", average(&v2));

    println!("Variance of v1: {}", variance(&v1));
    println!("Variance of v2: {}", variance(&v2));

    println!("Standard Deviation of v1: {}", standard_deviation(&v1));
    println!("Standard Deviation of v2: {}", standard_deviation(&v2));
}
