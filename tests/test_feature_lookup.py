import math
import unittest

import phonologic
from phonologic._file_parsing import parse_features


def entropy_sorted_table(system):
    N = len(system.query("[]"))

    def entropy(kvp):
        key, (present, absent) = kvp
        lengths = [l for l in (len(present), len(absent)) if l > 0]
        return -math.log2(min(lengths) / N)

    table = {}
    for feature in system.features:
        present = system.query(f"+{feature}")
        absent = system.query(f"-{feature}")
        if len(present) and len(absent):
            table[feature] = (present, absent)
    table = dict(sorted(table.items(), key=entropy))
    return table


class ParseDefinitionTests(unittest.TestCase):
    maxDiff = None

    def test_nearest(self):
        print("Nearest")
        system = phonologic.load("hayes-arpabet")
        for a in sorted(system.phoneme_entries):
            distances = []
            for b in system.phoneme_entries:
                if a == b:
                    continue
                analysis = system.analyze_feature_errors(a, b)
                deltas = analysis.steps[0].deltas

                distances.append((deltas, b))
            fewest_features = min(len(d) for d, p in distances)
            nearby = sorted(
                (
                    (p, d)
                    for d, p in distances
                    if len(d) == fewest_features
                ),
                key=lambda pd: len(pd[1])
            )
            print(f"{a}: {nearby}")
        print()

    def test_find(self):
        system = phonologic.load("hayes-arpabet")
        test_cases = [
            ("[+syllabic,  +high,  +front,  +tense,  -round]", {"IY"}),
            ("[+syllabic,  +high,  -front,  +tense,  +round]", {"UW"}),
            ("[+syllabic,  +high,  +front,  -tense,  -round]", {"IH"}),
            ("[+syllabic,  +high,  -front,  -tense,  +round]", {"UH"}),

            ("[+syllabic, -+high,  +front, +-tense,  -round]", {"EY"}),
            ("[+syllabic,  -high,  +front,  -tense,  -round]", {"EH"}),
            ("[+syllabic,  -high,  -front,  -tense,  +round]", {"AO"}),
            ("[+syllabic, -+high, -+front,  -tense, +-round]", {"OY"}),
            ("[+syllabic, -+high,  -front, +-tense,  +round]", {"OW"}),

            ("[+syllabic,  -high,  +front, 0tense,  -round]", {"AE"}),
            ("[+syllabic, -+high,  -front, 0tense, -+round]", {"AW"}),
            ("[+syllabic,  -high,  -front, -tense,  -round]", {"AH"}),
            ("[+syllabic, -+high, -+front, 0tense,  -round]", {"AY"}),
            ("[+syllabic,  -high,  -front, 0tense,  -round]", {"AA"}),

            ("[+voice, -syllabic, -consonantal, +approximant, +sonorant, +continuant]", {"Y", "R", "W"}),
            ("[+voice, -syllabic, -consonantal, +approximant, +sonorant, +continuant, -labial, -coronal]", {"Y"}),
            ("[+voice, -syllabic, -consonantal, +approximant, +sonorant, +continuant, +labial]", {"W"}),
            ("[+voice, -syllabic, -consonantal, +approximant, +sonorant, +continuant, -labial, +coronal]", {"R"}),
            ("[+voice, +syllabic, -consonantal, +approximant, +sonorant, +continuant, -labial, +coronal]", {"ER"}),

            ("[+voice, -syllabic, +consonantal, +approximant, +sonorant, +continuant]", {"L", "DX"}),
            ("[+voice, -syllabic, +consonantal, +approximant, +sonorant, +continuant, +lateral]", {"L"}),
            ("[+voice, -syllabic, +consonantal, +approximant, +sonorant, +continuant, -lateral]", {"DX"}),

            ("[+voice, -syllabic, +consonantal, -approximant, +sonorant, -continuant]", {"M", "N", "NG"}),
            ("[+voice, -syllabic, +consonantal, -approximant, +sonorant, -continuant, +coronal]", {"N"}),
            ("[+voice, -syllabic, +consonantal, -approximant, +sonorant, -continuant, -coronal, +labial]", {"M"}),
            ("[+voice, -syllabic, +consonantal, -approximant, +sonorant, -continuant, -coronal, -labial]", {"NG"}),

            ("[+voice, -syllabic, +consonantal, -approximant, -sonorant, +continuant]", {"V", "ZH", "DH", "Z"}),
            ("[+voice, -syllabic, +consonantal, -approximant, -sonorant, +continuant, -labial, +anterior, +distributed]", {"DH"}),
            ("[+voice, -syllabic, +consonantal, -approximant, -sonorant, +continuant, -labial, +anterior, -distributed]", {"Z"}),
            ("[+voice, -syllabic, +consonantal, -approximant, -sonorant, +continuant, -labial, -anterior]", {"ZH"}),
            ("[+voice, -syllabic, +consonantal, -approximant, -sonorant, +continuant, +labial]", {"V"}),

            ("[-voice, -syllabic, -approximant, -sonorant, +continuant]", {"SH", "TH", "F", "S", "HH"}),
            ("[-voice, -syllabic, +consonantal, -approximant, -sonorant, +continuant, +coronal, +anterior, +distributed]", {"TH"}),
            ("[-voice, -syllabic, +consonantal, -approximant, -sonorant, +continuant, +coronal, +anterior, -distributed]", {"S"}),
            ("[-voice, -syllabic, +consonantal, -approximant, -sonorant, +continuant, +coronal, -anterior]", {"SH"}),
            ("[-voice, -syllabic, +consonantal, -approximant, -sonorant, +continuant, -coronal]", {"F"}),
            ("[-voice, -syllabic, -consonantal, -approximant, -sonorant, +continuant, -coronal]", {"HH"}),

            ("[+voice, -syllabic, +consonantal, -approximant, -sonorant, -continuant]", {"G", "D", "B", "JH"}),
            ("[+voice, -syllabic, +consonantal, -approximant, -sonorant, -continuant, -coronal, +labial]", {"B"}),
            ("[+voice, -syllabic, +consonantal, -approximant, -sonorant, -continuant, -coronal, -labial]", {"G"}),
            ("[+voice, -syllabic, +consonantal, -approximant, -sonorant, -continuant, +coronal, +anterior]", {"D"}),
            ("[+voice, -syllabic, +consonantal, -approximant, -sonorant, -continuant, +coronal, -anterior]", {"JH"}),

            ("[-voice, -syllabic, +consonantal, -approximant, -sonorant, -continuant]", {"P", "T", "K", "CH"}),
            ("[-voice, -syllabic, +consonantal, -approximant, -sonorant, -continuant, -coronal, +labial]", {"P"}),
            ("[-voice, -syllabic, +consonantal, -approximant, -sonorant, -continuant, +coronal, -labial, +anterior]", {"T"}),
            ("[-voice, -syllabic, +consonantal, -approximant, -sonorant, -continuant, -coronal, -labial]", {"K"}),

            ("[-voice, -syllabic, +consonantal, -approximant, -sonorant, -continuant, +coronal, -labial, -anterior]", {"CH"}),
        ]
        symbols = set(s for s in system.entries if s[0] != "<")
        used_features = set()
        for query, expected in test_cases:
            with self.subTest((expected, query)):
                matching = system.query(query)
                actual = set(entry.symbol for entry in matching)
                self.assertSetEqual(expected, actual)
                if len(actual) == 1:
                    symbol = actual.pop()
                    if symbol in symbols:
                        symbols.remove(symbol)
                    features = parse_features(query)
                    used_features.update(set(features))
                    pass

        with self.subTest("coverage"):
            self.assertSetEqual(set(), symbols)
