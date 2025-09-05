/// String utilities module
/// Author: Hitesh

/// Check if a string is a palindrome
pub fn is_palindrome(s: &str)->bool {
    let cleaned: String = s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    cleaned==cleaned.chars().rev().collect::<String>()
}

/// Check if a string is numeric
pub fn is_numeric(s: &str)->bool {
    !s.is_empty() &&s.chars().all(|c| c.is_ascii_digit())
}

/// Reverse a string
pub fn reverse_string(s: &str)->String {
    s.chars().rev().collect()
}

/// Normalize: lowercase + trim spaces
pub fn normalize(s: &str)->String {
    s.trim().to_ascii_lowercase()
}
