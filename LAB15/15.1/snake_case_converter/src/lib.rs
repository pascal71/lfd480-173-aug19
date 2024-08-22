/// Converts a given string to snake_case.
///
/// # Examples
///
/// ```
/// let text = "HelloWorld";
/// assert_eq!(snake_case_converter::to_snake_case(text), "hello_world");
/// ```
pub fn to_snake_case(s: &str) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            if c.is_uppercase() {
                if i != 0 {
                    format!("_{}", c.to_lowercase())
                } else {
                    c.to_lowercase().to_string()
                }
            } else {
                c.to_string()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_converts_to_snake_case() {
        assert_eq!(to_snake_case("HelloWorld"), "hello_world");
        assert_eq!(to_snake_case("RustIsAwesome"), "rust_is_awesome");
    }
}
