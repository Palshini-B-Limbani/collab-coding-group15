/// String utilities module
/// Author: Aashish

/// Reverse a string
pub fn reverse_string(s: &str)->String {
    s.chars().rev().collect()
}
