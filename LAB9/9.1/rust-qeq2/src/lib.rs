pub mod qeq_lib {
    use std::f64;

    pub fn solve(a: f64, b: f64, c: f64) -> Result<(f64, f64), &'static str> {
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return Err("No real solutions");
        }

        let d = discriminant.sqrt();
        let solution1 = (-b + d) / (2.0 * a);
        let solution2 = (-b - d) / (2.0 * a);

        Ok((solution1, solution2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_solutions() {
        match qeq_lib::solve(1.0, -3.0, 2.0) {
            Ok((solution1, solution2)) => {
                assert!(
                    (solution1 == 2.0 && solution2 == 1.0)
                        || (solution1 == 1.0 && solution2 == 2.0)
                );
            }
            Err(_) => panic!("Expected real solutions, but got an error"),
        }
    }

    #[test]
    fn test_no_real_solutions() {
        match qeq_lib::solve(1.0, 1.0, 1.0) {
            Ok(_) => panic!("Expected no real solutions, but got Ok"),
            Err(e) => assert_eq!(e, "No real solutions"),
        }
    }

    #[test]
    fn table_driven_tests() {
        let test_cases = vec![
            (1.0, -3.0, 2.0, Ok((2.0, 1.0))), // x^2 - 3x + 2 = 0 -> roots are 1, 2
            (1.0, -2.0, 1.0, Ok((1.0, 1.0))), // x^2 - 2x + 1 = 0 -> double root 1
            (1.0, 0.0, -4.0, Ok((2.0, -2.0))), // x^2 - 4 = 0 -> roots are 2, -2
            (1.0, 1.0, 1.0, Err("No real solutions")), // x^2 + x + 1 = 0 -> no real roots
            (2.0, 5.0, 3.0, Ok((-1.0, -1.5))), // 2x^2 + 5x + 3 = 0 -> roots are -1, -1.5
        ];

        for (a, b, c, expected) in test_cases {
            match (qeq_lib::solve(a, b, c), expected) {
                (Ok((solution1, solution2)), Ok((exp1, exp2))) => {
                    assert!(
                        (solution1 == exp1 && solution2 == exp2)
                            || (solution1 == exp2 && solution2 == exp1)
                    );
                }
                (Err(e), Err(exp_err)) => assert_eq!(e, exp_err),
                _ => panic!("Test case failed"),
            }
        }
    }
}

