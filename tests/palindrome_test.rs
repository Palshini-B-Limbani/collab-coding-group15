use collab_coding_group15::palindrome;

#[test]
fn test_is_palindrome() {
    assert!(palindrome::is_palindrome("Racecar"));
    assert!(palindrome::is_palindrome("Never odd or even"));
    assert!(!palindrome::is_palindrome("hello"));
}