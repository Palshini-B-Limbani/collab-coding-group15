/// String utilities module
/// Author: Hitesh

/// Check if a string is numeric
pub fn is_numeric(s: &str)->bool {
    !s.is_empty() &&s.chars().all(|c| c.is_ascii_digit())
}