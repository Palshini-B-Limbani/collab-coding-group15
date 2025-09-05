import unittest
from remove_vowels import remove_vowels

class TestRemoveVowels(unittest.TestCase):
    def test_remove_vowels(self):
        self.assertEqual(remove_vowels("hello"), "hll")
        self.assertEqual(remove_vowels("AEIOUaeiou"), "")
        self.assertEqual(remove_vowels("Python Programming"), "Pythn Prgrmmng")
        self.assertEqual(remove_vowels(""), "")

if __name__ == "__main__":
    unittest.main()
