pub mod math_operations {
    pub fn add(a: i32, b: i32) -> Result<i32, String> {
        match a.checked_add(b) {
            Some(result) => Ok(result),
            None => Err("Overflow occurred during addition".to_string()),
        }
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    pub fn multiply(a: i32, b: i32) -> Result<i32, String> {
        match a.checked_mul(b) {
            Some(result) => Ok(result),
            None => Err("Overflow occurred during multiplication".to_string()),
        }
    }

    pub fn divide(a: i32, b: i32) -> Option<i32> {
        if b == 0 {
            None
        } else {
            Some(a / b)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_no_overflow() {
        assert_eq!(math_operations::add(2, 3), Ok(5));
        assert_eq!(math_operations::add(-2, -3), Ok(-5));
    }

    #[test]
    fn test_add_overflow() {
        assert_eq!(
            math_operations::add(i32::MAX, 1),
            Err("Overflow occurred during addition".to_string())
        );
    }

    #[test]
    fn test_subtract() {
        assert_eq!(math_operations::subtract(5, 3), 2);
        assert_eq!(math_operations::subtract(-5, -3), -2);
    }

    #[test]
    fn test_multiply_no_overflow() {
        assert_eq!(math_operations::multiply(2, 3), Ok(6));
        assert_eq!(math_operations::multiply(-2, -3), Ok(6));
    }

    #[test]
    fn test_multiply_overflow() {
        assert_eq!(
            math_operations::multiply(i32::MAX, 2),
            Err("Overflow occurred during multiplication".to_string())
        );
    }

    #[test]
    fn test_divide() {
        assert_eq!(math_operations::divide(6, 3), Some(2));
        assert_eq!(math_operations::divide(-6, -3), Some(2));
        assert_eq!(math_operations::divide(1, 0), None); // division by zero
    }

    #[test]
    fn table_driven_tests() {
        let test_cases = vec![
            // (a, b, expected_result)
            ("add", 1, 1, Ok(2)),
            ("add", -1, -1, Ok(-2)),
            ("subtract", 5, 3, Ok(2)),
            ("subtract", -5, -3, Ok(-2)),
            ("multiply", 2, 3, Ok(6)),
            ("multiply", -2, -3, Ok(6)),
            ("divide", 6, 3, Ok(2)),
            ("divide", -6, -3, Ok(2)),
        ];

        for (operation, a, b, expected) in test_cases {
            match operation {
                "add" => assert_eq!(math_operations::add(a, b), expected),
                "subtract" => assert_eq!(
                    math_operations::subtract(a, b),
                    expected.unwrap_or_default()
                ),
                "multiply" => assert_eq!(math_operations::multiply(a, b), expected),
                "divide" => assert_eq!(
                    math_operations::divide(a, b)
                        .map(|v| Ok(v))
                        .unwrap_or(Err("Division by zero".to_string())),
                    expected
                ),
                _ => panic!("Unknown operation"),
            }
        }
    }
}
