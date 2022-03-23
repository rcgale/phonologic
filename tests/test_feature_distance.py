import unittest

import phonologic
from phonologic._error_analysis._levenshtein import Action
from phonologic._file_parsing._parser import parse_features


class ParseDefinitionTests(unittest.TestCase):
    maxDiff = None

    def test_vector_diff_ipa(self):
        systems = {
            "hayes": phonologic.load("hayes"),
            "hayes-arpabet": phonologic.load("hayes-arpabet"),
        }

        test_cases = [
            ("hayes", "f", "v", [
                (1, Action.SUB, parse_features("[+voice]")),
            ]),
            ("hayes", "s", "θ", [
                (2, Action.SUB, parse_features("[+distributed, -strident]")),
            ]),
            ("hayes-arpabet", "EY", "AY", [
                # -low -> +-low      = 1.5/2 = 0.75
                # +-tense -> 0tense  = 0.5/2 = 0.25
                # +front -> -+front  = 1.5/2 = 0.75
                (1.75, Action.SUB,  parse_features("[+-low, 0tense, -+front]")),
            ]),
            ("hayes-arpabet", "AY", "EY", [
                (1.75, Action.SUB,  parse_features("[-low, +-tense, +front]")),
            ]),
            ("hayes", "kæt", "hæt", [
                (6.00, Action.SUB,  parse_features("[-consonantal, +continuant, +delayedrelease, -dorsal, 0high, 0low, +spreadglottis]")),
                (0.00, Action.EQ,  parse_features("[]")),
                (0.00, Action.EQ,  parse_features("[]")),
            ]),
            ("hayes", "kæt", "æt", [
                (23.00, Action.DEL, systems["hayes"]["k"].features),
                (0.00, Action.EQ,  parse_features("[]")),
                (0.00, Action.EQ,  parse_features("[]")),
            ]),
            ("hayes", "æt", "kæt", [
                (23, Action.INS,  systems["hayes"]["k"].features),
                (0, Action.EQ,  parse_features("[]")),
                (0, Action.EQ,  parse_features("[]")),
            ]),
            ("hayes", "kɔl", "ko ʊl", [
                (0.00, Action.EQ,  parse_features("[]")),
                (1.00, Action.SUB,  parse_features("[+tense]")),
                (24.00, Action.INS,  systems["hayes"]["ʊ"].features),
                (0.00, Action.EQ,  parse_features("[]")),
            ]),
            ("hayes", "kɔl", "ko͡ʊl", [
                (0.00, Action.EQ,  parse_features("[]")),
                (1.00, Action.SUB,  parse_features("[+tense, +-tense, -+high]")),
                (0.00, Action.EQ,  parse_features("[]")),
            ]),
            ("hayes-arpabet", "R", "ER", [
                (1, Action.SUB,  parse_features("[+syllabic]")),
            ]),
            ("hayes-arpabet", "ER", "R", [
                (1, Action.SUB,  parse_features("[-syllabic]")),
            ]),
            ("hayes-arpabet", "ER", "UW", [
                (8, Action.SUB,  parse_features("[-coronal, +dorsal, +labial, 0anterior, 0distributed, +high, -low, 0strident, +round, +tense, -front, +back]")),
            ]),
            ("hayes-arpabet", "UW", "ER", [
                (8, Action.SUB,  parse_features("[+coronal, -dorsal, -labial, -anterior, +distributed, 0high, 0low, -strident, -round, 0tense, 0front, 0back]")),
            ]),
            ("hayes-arpabet", "K <spn> T", "K <spn> T", [
                (0.00, Action.EQ, parse_features("[]")),
                (0.00, Action.EQ, parse_features("[]")),
                (0.00, Action.EQ, parse_features("[]")),
            ]),
            ("hayes-arpabet", "K <spn> T", "K <sil> T", [
                (0.00, Action.EQ, parse_features("[]")),
                (0.00, Action.INS, parse_features("[]")),
                (0.00, Action.DEL, parse_features("[]")),
                (0.00, Action.EQ, parse_features("[]")),
            ]),
            ("hayes-arpabet", "OW <spn>", "OW M <spn> <spn>", [
                (0.00, Action.EQ, parse_features("[]")),
                (19.5, Action.INS, systems["hayes-arpabet"]["M"].features),
                (0.00, Action.INS, parse_features("[]")),
                (0.00, Action.EQ, parse_features("[]")),
            ]),
            ("hayes-arpabet", "K AE T", "K AE T <spn>", [
                (0.00, Action.EQ,  parse_features("[]")),
                (0.00, Action.EQ,  parse_features("[]")),
                (0.00, Action.EQ,  parse_features("[]")),
                (0.00, Action.INS,  parse_features("[]")),
            ]),
            ("hayes-arpabet", "K AE T", "K <spn> T", [
                (0.00, Action.EQ,  parse_features("[]")),
                (0.00, Action.INS,  parse_features("[]")),
                (21.5, Action.DEL,  systems["hayes-arpabet"]["AE"].features),
                (0.00, Action.EQ,  parse_features("[]")),
            ]),
            ("hayes-arpabet", "S W IH NG Z S Z", "S W IH M IH NG IH SH IH Z", [
                (0, Action.EQ, parse_features("[]")),
                (0, Action.EQ, parse_features("[]")),
                (22, Action.INS, systems["hayes-arpabet"]["IH"].features),
                (19.5, Action.INS, systems["hayes-arpabet"]["M"].features),
                (0, Action.EQ, parse_features("[]")),
                (0, Action.EQ, parse_features("[]")),
                (10.5, Action.SUB, parse_features("[+syllabic, 0delayedrelease, +front, +dorsal, 0strident, -tense, -back, 0anterior, +high, -coronal, -consonantal, -low, +approximant, 0distributed, +sonorant]")),
                (2, Action.SUB, parse_features("[-anterior, +distributed]")),
                (22, Action.INS, systems["hayes-arpabet"]["IH"].features),
                (0, Action.EQ, parse_features("[]")),
            ])
        ]

        for system_name, a, b, expected in test_cases:
            systems[system_name] = systems.get(system_name, phonologic.load(system_name))
            system = systems[system_name]

            with self.subTest((a, b)):
                analysis = system.analyze_feature_errors(a, b)
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

            for n, (exp_cost, exp_action, exp_features) in enumerate(expected):
                with self.subTest((a, b, f"Step {n} cost")):
                    self.assertAlmostEqual(exp_cost, actual[n].cost)
                with self.subTest((a, b, f"Step {n} action")):
                    self.assertEqual(exp_action, actual[n].action)
                with self.subTest((a, b, f"Step {n} features")):
                    if len(actual[n].deltas):
                        left, right = zip(*actual[n].deltas)
                        actual_deltas = left if exp_action == Action.DEL else right
                    else:
                        actual_deltas = []
                    self.assertSetEqual(set(exp_features.values()), set(actual_deltas))
