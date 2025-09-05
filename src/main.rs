
use collab_coding_group15::{ normal, numeric, palindrome, reverse, remove_vowels, word_count};

fn main() {
    println!("--- Collab Project Driver ---");

    // Strings
    //palindrome
    println!("Is 'Never odd or even' palindrome? {}", palindrome::is_palindrome("Never odd or even"));
    //reverse
    println!("Reverse of 'Rust' = {}", reverse::reverse_string("Rust"));
    //numeric
    println!("Is '12345' numeric? {}", numeric::is_numeric("12345"));
    //normal
    println!("Normalized '  HELLO  ' = {}", normal::normalize("  HELLO  "));
    //remove_vowels
    println!("Remove vowels from 'Hello World' = {}", remove_vowels::remove_vowels("Hello World"));
    //count words
     println!("Word count of 'Rust is fast and safe' = {}", word_count::word_count("Rust is fast and safe"));
} 

