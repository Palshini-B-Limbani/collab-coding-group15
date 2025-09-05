def remove_vowels(s: str) -> str:
    """
    Removes all vowels from the input string.

    Author: Your Name

    Args:
        s (str): Input string.

    Returns:
        str: String with all vowels removed.
    """
    vowels = "aeiouAEIOU"
    return "".join(char for char in s if char not in vowels)
