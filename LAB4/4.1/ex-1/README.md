Conclusion:

In Rust, when you assign a String to another variable, it does not perform a shallow copy.
Instead, it transfers ownership of (moves) the data, leaving the original variable (text1 in this case) invalid.
If you try to use the original variable after the move, the Rust compiler will throw an error, ensuring memory safety.
