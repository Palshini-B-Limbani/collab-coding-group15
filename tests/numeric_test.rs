use collab_coding_group15::numeric;

#[test]
fn test_is_numeric() {
    assert!(numeric::is_numeric("12345"));
    assert!(!numeric::is_numeric("12a45"));
    assert!(!numeric::is_numeric(""));
}




