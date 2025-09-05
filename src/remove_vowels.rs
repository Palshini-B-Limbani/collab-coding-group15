/// Removes all vowels from the input string.
/// Author: Teena
pub fn remove_vowels(s: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    s.chars()
        .filter(|c| !vowels.contains(c))
        .collect()
}
