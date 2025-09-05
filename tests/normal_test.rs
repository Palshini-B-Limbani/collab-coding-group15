// tests/normal_test.rs
use collab_coding_group15::normal::normalize;

#[test]
fn test_normalize() {
    assert_eq!(normalize("  HELLO "), "hello");
    assert_eq!(normalize(" Rust "), "rust");
}
