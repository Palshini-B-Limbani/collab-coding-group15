use collab_coding_group15::reverse;
#[test]
fn test_reverse_string() {
    assert_eq!(reverse::reverse_string("hello"), "olleh");
    assert_eq!(reverse::reverse_string(""), "");
}



