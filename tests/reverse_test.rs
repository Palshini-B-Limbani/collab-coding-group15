use collab_coding_group15::reverse as strings;
#[test]
fn test_reverse_string() {
    assert_eq!(strings::reverse_string("hello"), "olleh");
    assert_eq!(strings::reverse_string(""), "");
}


