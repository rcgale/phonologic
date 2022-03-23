import unittest

from phonologic._systems import FeatureValue, Feature, FeatureCollection, Definition, Symbol, DefinitionItems
from phonologic._file_parsing._parser import parse_features, parse_statement
from phonologic._file_parsing._tokenizer import tokenize


class ParseDefinitionTests(unittest.TestCase):
    maxDiff = None

    def test_feature_parse(self):
        test_cases = [
            ("[]", FeatureCollection()),
            ("[-syl]", FeatureCollection([
                Feature(FeatureValue(-1), "syl")
            ])),
            ("[-syl, +appr, 0son]", FeatureCollection([
                Feature(FeatureValue(-1), "syl"),
                Feature(FeatureValue(+1), "appr"),
                Feature(FeatureValue(0), "son"),
            ])),
        ]
        for s, expected in test_cases:
            with self.subTest(s):
                actual = parse_features(s)
                self.assertEqual(expected, actual)

    def test_tokenize(self):
        test_cases = [
            (
                "<glide> = [-syl, -cons, +appr, +son]",
                ("<glide>", "=", "[-syl, -cons, +appr, +son]")
            ),
            (
                "t̪͡θ = <voiceless> <dental> <affricate> [-strid, -lat]",
                ("t̪͡θ", "=", "<voiceless>", "<dental>", "<affricate>", "[-strid, -lat]")
            ),
            (
                "<a>=<b>",
                ("<a>", "=", "<b>")
            ),
            (
                "<a> =<b>",
                ("<a>", "=", "<b>")
            ),
            (
                "<a>= <b>",
                ("<a>", "=", "<b>")
            ),
            (
                "a͡ʊ = a [-+high, -+low, 0-tense, -+back, -+round]",
                ("a͡ʊ", "=", "a", "[-+high, -+low, 0-tense, -+back, -+round]"),
            ),
            (
                "# This is a comment.",
                tuple(),
            ),
            (
                "<a> = <b>  # This is a comment.",
                ("<a>", "=", "<b>"),
            ),
            (
                "# Diphthongs use special symbols: `+-` means the feature goes from present to absent, `-+` is the reverse.",
                tuple()
            )

        ]
        for line, expected in test_cases:
            with self.subTest(line):
                actual = tokenize(line)
                self.assertEqual(expected, actual)

    def test_parser(self):
        test_cases = [
            (
                "<glide> = [-syl, +appr, 0son]",
                Definition(Symbol("<glide>"), DefinitionItems([
                    FeatureCollection([
                        Feature(FeatureValue(-1), "syl"),
                        Feature(FeatureValue(+1), "appr"),
                        Feature(FeatureValue(0), "son"),
                    ])
                ]))
            ),
            (
                "t̪͡θ = <voiceless> <dental> <affricate> [-strid, -lat]",
                Definition(Symbol("t̪͡θ"), DefinitionItems([
                    Symbol("<voiceless>"),
                    Symbol("<dental>"),
                    Symbol("<affricate>"),
                    FeatureCollection([
                        Feature(FeatureValue(-1), "strid"),
                        Feature(FeatureValue(-1), "lat"),
                    ])
                ]))
            ),
        ]

        for line, expected in test_cases:
            with self.subTest(line):
                actual = parse_statement(line)
                self.assertEqual(expected, actual)


if __name__ == '__main__':
    unittest.main()
