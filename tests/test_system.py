import unittest

from phonologic._exceptions import MustHaveDefaultError, FileParseError
from phonologic._file_parsing._parser import parse_file


class SystemTests(unittest.TestCase):
    def test_parse_file(self):
        test_cases = [
            (
                [
                    "<default> = [0labial, 0labiodental, 0coronal, 0dorsal, 0lateral, 0sonorant, 0approximant, 0continuant, 0syllabic, 0consonantal, 0delayedrelease, 0voice]",
                    "<labiodental> = [+labial, +labiodental, -coronal, -dorsal, -lateral]",
                    "<fricative> = [-syllabic, +consonantal, -approximant, +sonorant, +continuant, +delayedrelease]",
                    "f = [+voice] <labiodental> <fricative>",
                ],
                ("<labiodental>", "<fricative>", "f")
            ),
        ]
        self.maxDiff = None
        for lines, expected in test_cases:
            with self.subTest(lines):
                actual = parse_file(lines)
                # self.assertEqual(expected, actual)

    def test_default(self):
        test_cases = [
            (
                [
                    "<default> = [+labial, +labiodental, -coronal, -dorsal, -lateral]",
                ],
                None
            ),
            (
                [
                    "<default> = [0labial, 0labiodental, 0coronal, 0dorsal, 0lateral, 0syllabic, 0consonantal, 0approximant, 0sonorant, 0continuant, 0delayedrelease]",
                    "<fricative> = [-syllabic, +consonantal, -approximant, +sonorant, +continuant, +delayedrelease]",
                ],
                None
            ),
            (
                [
                    "<fricative> = [-syllabic, +consonantal, -approximant, +sonorant, +continuant, +delayedrelease]",
                    "<default> = [+labial, +labiodental, -coronal, -dorsal, -lateral]",
                ],
                MustHaveDefaultError
            ),
            (
                [
                    "<fricative> = [-syllabic, +consonantal, -approximant, +sonorant, +continuant, +delayedrelease]",
                ],
                MustHaveDefaultError
            ),
        ]
        self.maxDiff = None
        for lines, expected in test_cases:
            with self.subTest(lines):
                try:
                    result = parse_file(lines)
                    assert expected is None
                except Exception as actual:
                    if isinstance(actual, AssertionError) or expected is None:
                        raise
                    if isinstance(actual, FileParseError):
                        actual = actual.inner_error
                    self.assertIsInstance(actual, expected)


if __name__ == '__main__':
    unittest.main()
