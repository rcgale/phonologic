use crate::distance::levenshtein::{ComputeCost, DefaultCalculator, Cost};
use crate::phl::systems::PhonologicalFeatureSystem;
use crate::phl::tokenizer::Symbol;
use crate::errors::PhlDistanceError;

pub struct PhonemeCostCalculator<'a> {
    pub(crate) system: &'a PhonologicalFeatureSystem
}

impl<'a> PhonemeCostCalculator<'a> {
    pub fn new(system: &'a PhonologicalFeatureSystem) -> Self {
        PhonemeCostCalculator { system }
    }

    pub(crate) fn is_zero_cost(&self, s: Option<&Symbol>) -> bool {
        s.is_some() && self.system.is_zero_cost(s.unwrap())
    }
}

impl<'a> ComputeCost<Symbol> for PhonemeCostCalculator<'a> {
    fn cost_sub(&self, expected: Option<&Symbol>, actual: Option<&Symbol>) -> Result<Cost, PhlDistanceError> {
        Ok(
            if self.is_zero_cost(expected) && self.is_zero_cost(actual) {
                Cost::ZERO
            }
            else if self.is_zero_cost(expected) || self.is_zero_cost(actual) {
                Cost::INFINITY
            }
            else if expected.is_none() || actual.is_none() {
                Cost::INFINITY
            }
            else {
                let (expected, actual) = (expected.unwrap(), actual.unwrap());
                Cost(if expected != actual { 1.0 } else { 0.0 })
            }
        )
    }

    fn cost_del(&self, expected: Option<&Symbol>) -> Result<Cost, PhlDistanceError> {
        if expected.is_some() && self.system.is_zero_cost(&expected.unwrap()) {
            Ok(Cost::ZERO)
        }
        else {
            DefaultCalculator { }.cost_del(expected)
        }
    }

    fn cost_ins(&self, actual: Option<&Symbol>) -> Result<Cost, PhlDistanceError> {
        if actual.is_some() && self.system.is_zero_cost(&actual.unwrap()) {
            Ok(Cost::ZERO)
        }
        else {
            DefaultCalculator { }.cost_ins(actual)
        }
    }
}


#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use crate::distance::levenshtein::{Action, Cost};
    use crate::distance::phoneme_tokenizer::PhonemeTokenizer;
    use crate::phl::systems::PhonologicalFeatureSystem;

    #[test]
    fn test_phoneme_distance() {
        let test_cases = vec![
            ("hayes", "", "", vec![]),
            ("hayes", "f", "v", vec![
                (1, Action::SUB),
            ]),
            ("hayes", "s", "θ", vec![
                (1, Action::SUB),
            ]),
            ("hayes", "e͡ɪ", "a͡ɪ", vec![
                // -low -> +-low      = 1.5/2 = 0.75
                // +-tense -> 0tense  = 0.5/2 = 0.25
                // +front -> -+front  = 1.5/2 = 0.75
                (1, Action::SUB),
            ]),
            ("hayes", "a͡ɪ", "e͡ɪ", vec![
                (1, Action::SUB),
            ]),
            ("hayes", "kæt", "hæt", vec![
                (1, Action::SUB),
                (0, Action::EQ),
                (0, Action::EQ),
            ]),
            ("hayes", "kæt", "æt", vec![
                (1, Action::DEL),
                (0, Action::EQ),
                (0, Action::EQ),
            ]),
            ("hayes", "æt", "kæt", vec![
                (1, Action::INS),
                (0, Action::EQ),
                (0, Action::EQ),
            ]),
            ("hayes-arpabet", "R", "ER", vec![
                (1, Action::SUB),
            ]),
            ("hayes-arpabet", "ER", "R", vec![
                (1, Action::SUB),
            ]),
            ("hayes-arpabet", "ER", "UW", vec![
                (1, Action::SUB),
            ]),
            ("hayes-arpabet", "UW", "ER", vec![
                (1, Action::SUB),
            ]),
            ("hayes-arpabet", "K AE T", "K AE T <spn>", vec![
                (0, Action::EQ),
                (0, Action::EQ),
                (0, Action::EQ),
                (0, Action::INS),
            ]),
            ("hayes-arpabet", "K AE T", "K <spn> T", vec![
                (0, Action::EQ),
                (0, Action::INS),
                (1, Action::DEL),
                (0, Action::EQ),
            ]),
            ("hayes-arpabet", "S W IH NG Z S Z", "S W IH M IH NG IH SH IH Z", vec![
                (0, Action::EQ),
                (0, Action::EQ),
                (0, Action::EQ),
                (1, Action::INS),
                (1, Action::INS),
                (0, Action::EQ),
                (1, Action::SUB),
                (1, Action::SUB),
                (1, Action::INS),
                (0, Action::EQ),
            ])
        ];

        let system_names = test_cases.clone().into_iter().map(|(sn, _, _, _)| sn);
        let systems = load_systems(system_names.collect());


        for (system_name, a, b, expected) in test_cases {
            let system = systems.get(&system_name).unwrap();
            let tokenizer = PhonemeTokenizer::build(system);
            let actual = system.phoneme_edit_distance(a, b).unwrap();

            let expected_left_tokens = tokenizer.tokenize(a);
            let expected_right_tokens = tokenizer.tokenize(b);

            let actual_left_tokens: Vec<_> = actual
                .iter()
                .filter_map(|s| s.expected.clone())
                .collect();
            assert_eq!(expected_left_tokens, actual_left_tokens);

            let actual_right_tokens: Vec<_> = actual
                .iter()
                .filter_map(|s| s.actual.clone())
                .collect();
            assert_eq!(expected_right_tokens, actual_right_tokens);

            let expected_steps: Vec<_> = expected
                .into_iter()
                .map(|(cost, action)| (Cost(cost as f64), action))
                .collect();
            let actual_steps: Vec<_> = actual
                .into_iter()
                .map(|s| (s.cost, s.action))
                .collect();
            assert_eq!(expected_steps, actual_steps);
        }
    }

    fn load_systems(system_names: HashSet<&str>) -> HashMap<&str, PhonologicalFeatureSystem>{
        system_names
            .clone()
            .into_iter()
            .map(|system_name| (system_name, PhonologicalFeatureSystem::load(system_name).unwrap()))
            .collect()
    }
}