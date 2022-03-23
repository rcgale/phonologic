import unittest

from phonologic._error_analysis._levenshtein import ActionStep, edit_distance, Action, LevenshteinCost


class ParseDefinitionTests(unittest.TestCase):
    maxDiff = None

    def test_cost_min(self):
        test_cases = (
            (
                LevenshteinCost(Action.EQ, cost=0),
                LevenshteinCost(Action.DEL, cost=0),
                LevenshteinCost(Action.INS, cost=0),
                0
            ),
            (
                LevenshteinCost(Action.DEL, cost=0),
                LevenshteinCost(Action.EQ, cost=0),
                LevenshteinCost(Action.INS, cost=0),
                1
            ),
            (
                LevenshteinCost(Action.DEL, cost=0),
                LevenshteinCost(Action.INS, cost=0),
                LevenshteinCost(Action.EQ, cost=0),
                2
            ),
            (
                0,
                LevenshteinCost(Action.INS, cost=0),
                LevenshteinCost(Action.EQ, cost=0),
                2
            ),
            (
                LevenshteinCost(Action.EQ, cost=0),
                LevenshteinCost(Action.DEL, cost=0),
                0,
                0
            ),
        )
        for *items, expected_idx in test_cases:
            with self.subTest(items):
                actual = min(*items)
                self.assertEqual(items[expected_idx].action, actual.action)

    def test_levenshtein(self):
        test_cases = [
            ("A", "A", 0),
            ("AA", "AA", 0),
            ("A", "B", 1),
            ("AA", "BB", 2),
            ("AA", "A", 1),
            ("A", "AA", 1),
            ("ABC", "AA", 2),
            ("AA", "ABC", 2),
        ]
        for a, b, expected in test_cases:
            with self.subTest((a, b)):
                actual = edit_distance(a, b)
                self.assertEqual(expected, actual)

    def test_backtrace(self):
        test_cases = [
            ("", "", tuple()),
            ("A", "A", (
                ActionStep(Action.EQ, "A", "A", 0.0),
            )),
            ("AA", "AA", (
                ActionStep(Action.EQ, "A", "A", 0.0),
                ActionStep(Action.EQ, "A", "A", 0.0),
            )),
            ("A", "B", (
                ActionStep(Action.SUB, "A", "B", 1.0),
            )),
            ("AA", "BB", (
                ActionStep(Action.SUB, "A", "B", 1.0),
                ActionStep(Action.SUB, "A", "B", 1.0),
            )),
            ("AB", "A", (
                ActionStep(Action.EQ, "A", "A", 0.0),
                ActionStep(Action.DEL, "B", None, 1.0),
            )),
            ("A", "AB", (
                ActionStep(Action.EQ, "A", "A", 0.0),
                ActionStep(Action.INS, None, "B", 1.0),
            )),
            ("ABC", "BB", (
                ActionStep(Action.SUB, 'A', 'B', 1.0),
                ActionStep(Action.EQ, 'B', 'B', 0.0),
                ActionStep(Action.DEL, 'C', None, 1.0),
            )),
            ("BB", "ABC", (
                ActionStep(Action.SUB, 'B', 'A', 1.0),
                ActionStep(Action.EQ, 'B', 'B', 0.0),
                ActionStep(Action.INS, None, 'C', 1.0),
            )),
            ("AAA", "AA", (
                ActionStep(Action.DEL, 'A', None, 1.0),
                ActionStep(Action.EQ, 'A', 'A', 0.0),
                ActionStep(Action.EQ, 'A', 'A', 0.0),
            )),

            ("AA", "AAA", (
                ActionStep(Action.INS, None, 'A', 1.0),
                ActionStep(Action.EQ, 'A', 'A', 0.0),
                ActionStep(Action.EQ, 'A', 'A', 0.0),
            )),
        ]
        for a, b, expected in test_cases:
            with self.subTest((a, b)):
                diff = edit_distance(a, b)
                actual = diff.trace()
                self.assertEqual(expected, actual)
