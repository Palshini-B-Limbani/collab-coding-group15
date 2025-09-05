use unicode_normalization::UnicodeNormalization;
/// Normalize a string by:
/// 1. Trimming leading/trailing whitespace
/// 2. Converting to lowercase
/// 3. Applying Unicode NFKC normalization
/// 4. Collapsing multiple spaces into one
/// # Arguments
/// * `s` - A string slice to normalize
/// # Returns
/// * A new string with normalized text
/// # Author
/// Suyash Suryavansh (22BDS058)
pub fn normalize(s: &str) -> String {
    s.trim()                             // Step 1: Trim
     .to_lowercase()                     // Step 2: Lowercase
     .nfkc()                             // Step 3: Unicode NFKC
     .collect::<String>()
     .split_whitespace()                 // Step 4: Collapse spaces
     .collect::<Vec<_>>()
     .join(" ")
}

