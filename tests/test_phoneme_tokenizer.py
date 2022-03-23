import unittest

import phonologic


class ParseDefinitionTests(unittest.TestCase):
    maxDiff = None

    def test_tokenize_ipa(self):
        system = phonologic.load("hayes")
        test_cases = [
            ("", tuple()),
            ("asdf", ("a", "s", "d", "f")),
            ("as df", ("a", "s", "d", "f")),
            ("stɛθəskoʊp", ('s', 't', 'ɛ', 'θ', 'ə', 's', 'k', 'oʊ', 'p')),
            ("stɛθəsko͡ʊp", ('s', 't', 'ɛ', 'θ', 'ə', 's', 'k', 'o͡ʊ', 'p')),
            ("/ˈstɛθəsˌkoʊp/", ('s', 't', 'ɛ', 'θ', 'ə', 's', 'k', 'oʊ', 'p')),
        ]
        for s, expected in test_cases:
            with self.subTest(s):
                actual = system.tokenize(s)
                self.assertEqual(expected, actual)

    def test_tokenize_arpabet(self):
        system = phonologic.load("hayes-arpabet")
        test_cases = [
            ("", tuple()),
            ("K AE T", ("K", "AE", "T")),
        ]
        for s, expected in test_cases:
            with self.subTest(s):
                actual = system.tokenize(s)
                self.assertEqual(expected, actual)
