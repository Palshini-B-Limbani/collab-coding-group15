#[cfg(test)]
mod tests {
    use collab_coding_group15::word_count::word_count;

    #[test]
    fn test_word_count() {
        assert_eq!(word_count("Rust is awesome"), 3);
        assert_eq!(word_count("   multiple   spaces   "), 2);
        assert_eq!(word_count(""), 0);
    }
}
