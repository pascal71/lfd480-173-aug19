pub mod math_operations {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
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
    fn test_add() {
        assert_eq!(math_operations::add(2, 3), 5);
        assert_eq!(math_operations::add(-2, -3), -5);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(math_operations::subtract(5, 3), 2);
        assert_eq!(math_operations::subtract(-5, -3), -2);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(math_operations::multiply(2, 3), 6);
        assert_eq!(math_operations::multiply(-2, -3), 6);
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
            ("add", 1, 1, 2),
            ("add", -1, -1, -2),
            ("subtract", 5, 3, 2),
            ("subtract", -5, -3, -2),
            ("multiply", 2, 3, 6),
            ("multiply", -2, -3, 6),
            ("divide", 6, 3, 2),
            ("divide", -6, -3, 2),
        ];

        for (operation, a, b, expected) in test_cases {
            match operation {
                "add" => assert_eq!(math_operations::add(a, b), expected),
                "subtract" => assert_eq!(math_operations::subtract(a, b), expected),
                "multiply" => assert_eq!(math_operations::multiply(a, b), expected),
                "divide" => assert_eq!(math_operations::divide(a, b).unwrap_or(0), expected),
                _ => panic!("Unknown operation"),
            }
        }
    }
}
