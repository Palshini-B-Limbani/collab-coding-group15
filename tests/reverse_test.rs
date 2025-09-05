use collab-coding-group15::reverse;
#[test]
fn test_reverse_string() {
    assert_eq!(strings::reverse_string("hello"), "olleh");
    assert_eq!(strings::reverse_string(""), "");
}
