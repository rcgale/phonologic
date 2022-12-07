use regex::Regex;
use std::collections::HashSet;
use string_join::Join;
use crate::phl::systems::PhonologicalFeatureSystem;
use crate::phl::tokenizer::Symbol;
use itertools::Itertools;

pub struct PhonemeTokenizer {
    pattern: Regex,
    separators: HashSet<Symbol>,
    ignore_symbols: HashSet<Symbol>,
}

impl PhonemeTokenizer {
    pub fn build(system: &PhonologicalFeatureSystem) -> Self {
        let pattern_string = Self::build_pattern_string(system);
        let pattern = Regex::new(pattern_string.as_str()).unwrap();
        let separators = system.separators.clone();
        let ignore_symbols = system.ignore_symbols.clone();
        Self { pattern, separators, ignore_symbols }
    }

    pub fn tokenize(&self, s: &str) -> Vec<Symbol> {
        let tokens = self.split_tokens(s)
            .into_iter()
            .map(|t| Symbol(t.to_string()))
            .filter(|t| !self.separators.contains(t) && !self.ignore_symbols.contains(t))
            .collect();
        tokens
    }

    fn split_tokens(&self, s: &str) -> Vec<String> {
        let mut s = s;
        let mut tokens = vec![];
        while s.len() > 0 {
            let (token, remainder) = match self.pattern.find(s) {
                None => (&s[..1], &s[1..]), // todo: error handling for this
                Some(m) => (&s[m.start()..m.end()], &s[m.end()..])
            };
            tokens.push(token.to_string());
            s = remainder;
        }
        tokens
    }

    fn build_pattern_string(system: &PhonologicalFeatureSystem) -> String {
        r"|".join(
            Self::sort_and_group_symbols(system)
                .into_iter()
                .map(|(_, by_len)| "|".join(by_len))
        )
    }

    fn sort_and_group_symbols(system: &PhonologicalFeatureSystem) -> Vec<(usize, Vec<String>)> {
        let mut symbol_strings: Vec<_> = system.entries
            .clone()
            .into_iter()
            .map(|entry| entry.symbol)
            .filter(|symbol| !symbol.is_class())
            .map(|symbol| symbol.to_string())
            .collect();
        symbol_strings.extend(system.separators.iter().map(|s| s.to_string()));
        symbol_strings.extend(system.zero_cost_symbols.iter().map(|s| s.to_string()));
        symbol_strings.sort_by_key(|s| -(s.len() as i64));
        symbol_strings.push(r".".to_string());  // A single character as a last resort
        symbol_strings
            .iter()
            .map(|token| format!(r"^{token}"))
            .group_by(|s| s.len())
            .into_iter()
            .map(|(key, items)| (key, items.into_iter().collect()))
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use crate::distance::phoneme_tokenizer::PhonemeTokenizer;
    use crate::phl::systems::PhonologicalFeatureSystem;
    use crate::phl::tokenizer::Symbol;

    #[test]
    fn test_phoneme_tokenizer() {
        let system = PhonologicalFeatureSystem::load("hayes-ipa-arpabet").unwrap();
        let tokenizer = PhonemeTokenizer::build(&system);

        let test_cases = vec![
            ("", vec![]),
            ("asdf", vec!["a", "s", "d", "f"]),
            ("as df", vec!["a", "s", "d", "f"]),
            ("stɛθəskoʊp", vec!["s", "t", "ɛ", "θ", "ə", "s", "k", "oʊ", "p"]),
            ("stɛθəsko͡ʊp", vec!["s", "t", "ɛ", "θ", "ə", "s", "k", "o͡ʊ", "p"]),
            ("good", vec!["g", "o", "o", "d"]),
            ("/ˈstɛθəsˌkoʊp/", vec!["s", "t", "ɛ", "θ", "ə", "s", "k", "oʊ", "p"]),
        ];

        for (s, expected) in test_cases {
            let expected: Vec<_> = expected.iter().map(|t| Symbol(t.to_string())).collect();
            let actual = tokenizer.tokenize(s);
            assert_eq!(expected, actual);
            // self.assertEqual(expected, actual)        let output = system.analyze_phoneme_errors("koko", "nono");
        }
    }

    #[test]
    fn test_arpabet_tokenizer() {
        let system = PhonologicalFeatureSystem::load("hayes-ipa-arpabet").unwrap();
        let tokenizer = PhonemeTokenizer::build(&system);

        let test_cases = vec![
            ("K AE T <spn>", vec!["K", "AE", "T", "<spn>"]),
        ];

        for (s, expected) in test_cases {
            let expected: Vec<_> = expected.iter().map(|t| Symbol(t.to_string())).collect();
            let actual = tokenizer.tokenize(s);
            assert_eq!(expected, actual);
            // self.assertEqual(expected, actual)        let output = system.analyze_phoneme_errors("koko", "nono");
        }

    }
}
