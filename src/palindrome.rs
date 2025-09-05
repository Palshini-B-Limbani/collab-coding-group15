pub fn is_palindrome(s: &str)->bool {
    let cleaned: String = s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    cleaned==cleaned.chars().rev().collect::<String>()
}