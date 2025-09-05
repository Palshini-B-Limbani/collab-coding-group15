use collab_coding_group15::strings;

#[test]
fn test_is_palindrome() {
    assert!(strings::is_palindrome("Racecar"));
    assert!(strings::is_palindrome("Never odd or even"));
    assert!(!strings::is_palindrome("hello"));
}

#[test]
fn test_is_numeric() {
    assert!(strings::is_numeric("12345"));
    assert!(!strings::is_numeric("12a45"));
    assert!(!strings::is_numeric(""));
}

#[test]
fn test_reverse_string() {
    assert_eq!(strings::reverse_string("hello"), "olleh");
    assert_eq!(strings::reverse_string(""), "");
}

#[test]
fn test_normalize() {
    assert_eq!(strings::normalize("  HELLO "), "hello");
    assert_eq!(strings::normalize(" Rust "), "rust");
}
