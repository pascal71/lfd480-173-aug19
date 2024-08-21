pub fn assert_positive(n: i32) {
    if n < 0 {
        panic!("Number is negative: {}", n);
    } else if n == 0 {
        panic!("Number is zero.");
    }
    // No action needed for positive numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assert_positive_no_panic() {
        // These cases should not panic
        assert_positive(1);
        assert_positive(100);
        assert_positive(999);
    }

    #[test]
    #[should_panic(expected = "Number is negative: -1")]
    fn test_assert_positive_panic_negative() {
        assert_positive(-1);
    }

    #[test]
    #[should_panic(expected = "Number is zero.")]
    fn test_assert_positive_panic_zero() {
        assert_positive(0);
    }

    #[test]
    fn test_assert_positive_table_driven() {
        let test_cases = vec![
            (1, false),   // positive, should not panic
            (50, false),  // positive, should not panic
            (-1, true),   // negative, should panic
            (-100, true), // negative, should panic
            (0, true),    // zero, should panic
        ];

        for (input, should_panic) in test_cases {
            let result = std::panic::catch_unwind(|| {
                assert_positive(input);
            });

            if should_panic {
                assert!(result.is_err(), "Expected panic for input: {}", input);
            } else {
                assert!(result.is_ok(), "Unexpected panic for input: {}", input);
            }
        }
    }
}
