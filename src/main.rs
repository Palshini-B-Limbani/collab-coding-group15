use collab_utils::{math, strings, validate, convert,remove_vowels};
fn main(){
    println!("collab project");
    println!("Factorial of 5: {}", math::factorial(5));
    println!("Is 'Never odd or even' palindrome? {}", strings::is_palindrome("Never odd or even"));
    println!("Valid email 'alice@example.com'? {}", validate::is_valid_email("alice@example.com"));
    println!("Lowercase of 'Å' = {}", convert::to_lowercase_char('Å'));
    println!("'Hello, World!' without vowels: {}", strings::remove_vowels("Hello, World!"));
}
