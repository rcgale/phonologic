import csv
import json
import unittest

import phonologic
from phonologic._file_parsing._parser import parse_features
from tests import test_data


class SystemHayesTests(unittest.TestCase):
    maxDiff = None

    def _load_spreadsheet(self):
        mapping = {
            "delayed release": "delayedrelease",
            "spread gl": "spreadglottis",
            "constr gl": "constrictedglottis",
            "LABIAL": "labial",
            "CORONAL": "coronal",
            "DORSAL": "dorsal",
            "stress": None,
            "long": None,
        }
        data = {}
        with open(test_data.path("hayes-from-course-website.tsv")) as f:
            reader = csv.reader(f, dialect=csv.excel_tab)
            _, *header = (mapping.get(h, h) for h in next(reader))
            for symbol, *values in reader:
                values_str = ", ".join(f"{v}{f}" for v, f in zip(values, header) if f is not None)
                data[symbol] = f"[{values_str}]"
        return data

    def test_system_hayes(self):
        system = phonologic.load("hayes")
        data = self._load_spreadsheet()
        for symbol, value_str in data.items():
            with self.subTest(symbol):
                try:
                    actual = system[symbol].features
                except KeyError:
                    # 23 symbols not yet covered, most of which could/should be handled with a diactritic system.
                    # [ŋ+, ŋ˗, ʟ̠, ɣ+, x+, k+, ɡ+, k+͡x+, ɡ+͡ɣ+, ɉ, ɣ̠ , x̠, k̠, ɡ̠, d̠͡ɮ̠, t̠͡ɬ̠, c͡ç, ɉ͡ʝ, k̠͡x̠, ɡ̠̠͡ɣ̠, g͡b, ɰ̠]
                    continue

                if symbol in (
                        "ʕ",  # Book says +delayedrelease, spreadsheet says -delayedrelease
                ):
                    continue

                expected = parse_features(value_str)
                self.assertEqual(set(expected.values()), set(actual.values()), symbol)

    def test_arpabet_hayes(self):
        with open(test_data.path("ipa-to-arpabet.json")) as f:
            mapping = {value: key for key, value in json.load(f).items()}

        with open(test_data.path("arpabet-to-ipa.json"), "w") as f:
            json.dump(mapping, f, ensure_ascii=False)

        system_ipa = phonologic.load("hayes")
        system_arpa = phonologic.load("hayes-arpabet")
        for symbol_arpabet, entry_arpabet in system_arpa.phoneme_entries.items():
            symbol_ipa = mapping[symbol_arpabet]
            entry_ipa = system_ipa[symbol_ipa]
            with self.subTest(f"{symbol_arpabet} /{symbol_ipa}/"):
                ipa_features = set(
                    v
                    for v in entry_ipa.features.values()
                    if v.name not in ("trill", "constrictedglottis")
                )
                arpabet_features = set(entry_arpabet.features.values())
                self.assertSetEqual(ipa_features, arpabet_features)
