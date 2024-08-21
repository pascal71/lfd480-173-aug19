pub mod string_utils {
    // Reverses the input string
    pub fn reverse(s: &str) -> String {
        s.chars().rev().collect()
    }

    // Converts the input string to uppercase
    pub fn to_uppercase(s: &str) -> String {
        s.to_uppercase()
    }

    // Concatenates two strings
    pub fn concatenate(s1: &str, s2: &str) -> String {
        format!("{}{}", s1, s2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_basic() {
        assert_eq!(string_utils::reverse("hello"), "olleh");
        assert_eq!(string_utils::reverse("rust"), "tsur");
    }

    #[test]
    fn test_to_uppercase_basic() {
        assert_eq!(string_utils::to_uppercase("hello"), "HELLO");
        assert_eq!(string_utils::to_uppercase("Rust"), "RUST");
    }

    #[test]
    fn test_concatenate_basic() {
        assert_eq!(string_utils::concatenate("hello", "world"), "helloworld");
        assert_eq!(string_utils::concatenate("Rust", "Lang"), "RustLang");
    }

    #[test]
    fn test_reverse_table_driven() {
        let test_cases = vec![
            ("", ""),
            ("a", "a"),
            ("hello", "olleh"),
            ("rustlang", "gnaltsur"),
            ("12345", "54321"),
            ("!@#$%", "%$#@!"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(string_utils::reverse(input), expected);
        }
    }

    #[test]
    fn test_to_uppercase_table_driven() {
        let test_cases = vec![
            ("", ""),
            ("a", "A"),
            ("hello", "HELLO"),
            ("rustlang", "RUSTLANG"),
            ("12345", "12345"),
            ("!@#$%", "!@#$%"),
            ("Hello, World!", "HELLO, WORLD!"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(string_utils::to_uppercase(input), expected);
        }
    }

    #[test]
    fn test_concatenate_edge_cases() {
        assert_eq!(string_utils::concatenate("", "world"), "world");
        assert_eq!(string_utils::concatenate("hello", ""), "hello");
        assert_eq!(string_utils::concatenate("", ""), "");
    }
}
