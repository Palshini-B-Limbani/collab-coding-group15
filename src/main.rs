use collab_coding_group15::{math, strings, convert, validate};

fn main() {
    println!("--- Collab Project Driver ---");

    // Math
    println!("Factorial of 5 = {}", math::factorial(5));
    println!("Sum of 3 + 7 = {}", math::add(3, 7));

    // Strings
    println!("Is 'Never odd or even' palindrome? {}", strings::is_palindrome("Never odd or even"));
    println!("Reverse of 'Rust' = {}", strings::reverse_string("Rust"));
    println!("Is '12345' numeric? {}", strings::is_numeric("12345"));
    println!("Normalized '  HELLO  ' = {}", strings::normalize("  HELLO  "));

    // Convert
    println!("Lowercase of 'HELLO' = {}", convert::to_lowercase("HELLO"));
    println!("Uppercase of 'rust' = {}", convert::to_uppercase("rust"));

    // Validate
    println!("Valid email 'alice@example.com'? {}", validate::is_valid_email("alice@example.com"));
    println!("Is 'P@ssw0rd123' strong? {}", validate::is_strong_password("P@ssw0rd123"));
}
