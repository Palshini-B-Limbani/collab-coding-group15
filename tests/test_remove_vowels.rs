#[cfg(test)]
mod tests {
    use super::super::strings::remove_vowels;

    #[test]
    fn test_remove_vowels() {
        assert_eq!(remove_vowels("hello"), "hll");
        assert_eq!(remove_vowels("AEIOUaeiou"), "");
        assert_eq!(remove_vowels("Python Programming"), "Pythn Prgrmmng");
        assert_eq!(remove_vowels(""), "");
    }
}
