mod math;
mod strings;
mod validate;
mod convert;
fn main() {
    println!("Collab Project group 15");
    println!("Factorial of 5 = {}", math::factorial(5));
    println!("Is 'racecar' palindrome? {}", strings::is_palindrome("racecar"));
    println!("Valid email 'alice@example.com'? {}", validate::is_valid_email("alice@example.com"));
    println!("Lowercase of 'HELLO' = {}", convert::to_lowercase("HELLO"));
}
