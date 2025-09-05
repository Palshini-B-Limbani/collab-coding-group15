/// Counts the number of words in a string.
/// Author: Ganesh Bhabad
pub fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}
