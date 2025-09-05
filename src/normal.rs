use unicode_normalization::UnicodeNormalization;

/// Normalize a string by:
/// 1. Trimming leading/trailing whitespace
/// 2. Converting to lowercase
/// 3. Applying Unicode NFKC normalization
/// 4. Collapsing multiple spaces into one
///
/// # Arguments
/// * `s` - A string slice to normalize
///
/// # Returns
/// * A new string with normalized text
///
/// # Author
/// Suyash Suryavansh (22BDS058)


pub fn normalize(s: &str) -> String {
    // Step 1: Trim leading and trailing whitespace
    let trimmed = s.trim();

    // Step 2: Convert to lowercase
    let lowercased = trimmed.to_lowercase();

    // Step 3: Normalize Unicode (NFKC form = compatibility + canonical)
    let unicode_normalized: String = lowercased.nfkc().collect();

    // Step 4: Collapse multiple spaces into one
    let collapsed = unicode_normalized
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    collapsed
}
