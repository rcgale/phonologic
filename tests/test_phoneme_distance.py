import unittest

import phonologic
from phonologic._error_analysis._levenshtein import Action


class TestPhonemeDistance(unittest.TestCase):
    maxDiff = None

    def test_token_diff(self):
        systems = {
            "hayes": phonologic.load("hayes"),
            "hayes-arpabet": phonologic.load("hayes-arpabet"),
        }

        test_cases = [
            ("hayes", "", "", []),
            ("hayes", "f", "v", [
                (1, Action.SUB),
            ]),
            ("hayes", "s", "θ", [
                (1, Action.SUB),
            ]),
            ("hayes", "e͡ɪ", "a͡ɪ", [
                # -low -> +-low      = 1.5/2 = 0.75
                # +-tense -> 0tense  = 0.5/2 = 0.25
                # +front -> -+front  = 1.5/2 = 0.75
                (1, Action.SUB),
            ]),
            ("hayes", "a͡ɪ", "e͡ɪ", [
                (1, Action.SUB),
            ]),
            ("hayes", "kæt", "hæt", [
                (1, Action.SUB),
                (0, Action.EQ),
                (0, Action.EQ),
            ]),
            ("hayes", "kæt", "æt", [
                (1, Action.DEL),
                (0, Action.EQ),
                (0, Action.EQ),
            ]),
            ("hayes", "æt", "kæt", [
                (1, Action.INS),
                (0, Action.EQ),
                (0, Action.EQ),
            ]),
            ("hayes-arpabet", "R", "ER", [
                (1, Action.SUB),
            ]),
            ("hayes-arpabet", "ER", "R", [
                (1, Action.SUB),
            ]),
            ("hayes-arpabet", "ER", "UW", [
                (1, Action.SUB),
            ]),
            ("hayes-arpabet", "UW", "ER", [
                (1, Action.SUB),
            ]),
            ("hayes-arpabet", "K AE T", "K AE T <spn>", [
                (0, Action.EQ),
                (0, Action.EQ),
                (0, Action.EQ),
                (0, Action.INS),
            ]),
            ("hayes-arpabet", "K AE T", "K <spn> T", [
                (0, Action.EQ),
                (0, Action.INS),
                (1, Action.DEL),
                (0, Action.EQ),
            ]),
            ("hayes-arpabet", "S W IH NG Z S Z", "S W IH M IH NG IH SH IH Z", [
                (0, Action.EQ),
                (0, Action.EQ),
                (1, Action.INS),
                (1, Action.INS),
                (0, Action.EQ),
                (0, Action.EQ),
                (1, Action.SUB),
                (1, Action.SUB),
                (1, Action.INS),
                (0, Action.EQ),
            ])
        ]

        for system_name, a, b, expected in test_cases:
            systems[system_name] = systems.get(system_name, phonologic.load(system_name))
            system = systems[system_name]

            with self.subTest((a, b)):
                analysis = system.analyze_phoneme_errors(a, b)
                actual = analysis.steps

            with self.subTest((a, b, "left-tokens came back")):
                expected_left_tokens = system.tokenize(a)
                actual_left_tokens = tuple((step.expected for step in actual if step.expected is not None))
                self.assertEqual(expected_left_tokens, actual_left_tokens)

            with self.subTest((a, b, "right-tokens came back")):
                expected_right_tokens = system.tokenize(b)
                actual_right_tokens = tuple((step.actual for step in actual if step.actual is not None))
                self.assertEqual(expected_right_tokens, actual_right_tokens)

            with self.subTest((a, b, "lengths match")):
                self.assertEqual(len(expected), len(actual))

            for n, (exp_cost, exp_action) in enumerate(expected):
                with self.subTest((a, b, f"Step {n} cost")):
                    self.assertAlmostEqual(exp_cost, actual[n].cost)
                with self.subTest((a, b, f"Step {n} action")):
                    self.assertEqual(exp_action, actual[n].action)

