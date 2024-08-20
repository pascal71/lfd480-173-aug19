fn main() {
    // Test cases
    let n = 10; // Change this value to calculate a different Fibonacci number

    // Call the recursive Fibonacci function and print the result
    let result = fibonacci(n);
    println!("Fib({}) = {}", n, result);
}

// Recursive function to calculate the n-th Fibonacci number
fn fibonacci(n: u32) -> u32 {
    if n == 1 || n == 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
