use collab_coding_group15::palindrome as strings;
#[test]
fn test_is_palindrome() {
    assert!(strings::is_palindrome("Racecar"));
    assert!(strings::is_palindrome("Never odd or even"));
    assert!(!strings::is_palindrome("hello"));
}