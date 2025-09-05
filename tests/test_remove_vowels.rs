
    use collab_coding_group15::remove_vowels::remove_vowels;

    #[test]
    fn test_remove_vowels() {
        assert_eq!(remove_vowels("hello"), "hll");
        assert_eq!(remove_vowels("AEIOUaeiou"), "");
        assert_eq!(remove_vowels("Python Programming"), "Pythn Prgrmmng");
        assert_eq!(remove_vowels(""), "");
    }

