pub fn parse_number(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => Err(format!("Failed to parse '{}' as an integer.", s)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number_success() {
        assert_eq!(parse_number("42"), Ok(42));
        assert_eq!(parse_number("-10"), Ok(-10));
    }

    #[test]
    fn test_parse_number_failure() {
        assert_eq!(
            parse_number("abc"),
            Err("Failed to parse 'abc' as an integer.".to_string())
        );
        assert_eq!(
            parse_number(""),
            Err("Failed to parse '' as an integer.".to_string())
        );
        assert_eq!(
            parse_number("42.5"),
            Err("Failed to parse '42.5' as an integer.".to_string())
        );
    }

    #[test]
    fn test_parse_number_table_driven() {
        let test_cases = vec![
            ("123", Ok(123)),
            ("-456", Ok(-456)),
            ("", Err("Failed to parse '' as an integer.".to_string())),
            (
                "hello",
                Err("Failed to parse 'hello' as an integer.".to_string()),
            ),
            (
                "10000000000",
                Err("Failed to parse '10000000000' as an integer.".to_string()),
            ), // Out of i32 range
            (
                "42.0",
                Err("Failed to parse '42.0' as an integer.".to_string()),
            ), // Floating point
        ];

        for (input, expected) in test_cases {
            assert_eq!(parse_number(input), expected);
        }
    }
}
