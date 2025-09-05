use collab-coding-group15::numeric

#[test]
fn test_is_numeric() {
    assert!(strings::is_numeric("12345"));
    assert!(!strings::is_numeric("12a45"));
    assert!(!strings::is_numeric(""));
}
