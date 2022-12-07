use crate::distance::levenshtein::{Cost, ComputeCost};
use crate::phl::systems::PhonologicalFeatureSystem;
use crate::phl::tokenizer::{FeatureValue, Symbol};
use crate::errors::PhlDistanceError;
use crate::errors::PhlDistanceError::PhonemeNotFoundError;

#[derive()]
pub struct FeatureCostCalculator<'a> {
    pub(crate) system: &'a PhonologicalFeatureSystem
}

impl<'a> FeatureCostCalculator<'a> {
    pub fn new(system: &'a PhonologicalFeatureSystem) -> Self {
        FeatureCostCalculator{ system }
    }

    pub(crate) fn is_zero_cost(&self, s: Option<&Symbol>) -> bool {
        s.is_some() && self.system.is_zero_cost(s.unwrap())
    }

    fn err_if_not_found(&self, symbols: Vec<&Symbol>) -> Result<(), PhlDistanceError> {
        let not_found: Vec<_> = symbols
            .into_iter()
            .filter(|s| self.system.get_for_symbol(*s).is_none())
            .collect();

        if not_found.len() == 0 { Ok(()) } else {
            Err(PhonemeNotFoundError(format!("{not_found:?}")))
        }
    }
}

impl<'a> ComputeCost<Symbol> for FeatureCostCalculator<'a> {

    fn cost_sub(&self, expected: Option<&Symbol>, actual: Option<&Symbol>) -> Result<Cost, PhlDistanceError> {
        if expected.is_none() || actual.is_none() {
            return Ok(Cost::INFINITY);
        }
        if self.is_zero_cost(expected) && self.is_zero_cost(actual) {
            Ok(Cost::ZERO)
        }
        else if self.is_zero_cost(expected) || self.is_zero_cost(actual) {
            Ok(Cost::INFINITY)
        }
        else {
            let (expected, actual) = (expected.unwrap(), actual.unwrap());
            self.err_if_not_found(vec![expected, actual])?;
            let expected_features = &self.system.get_for_symbol(expected).unwrap().features;
            let actual_features = &self.system.get_for_symbol(actual).unwrap().features;
            let sum = expected_features
                .iter()
                .zip(actual_features.iter())
                .map(|(a, b)| feature_cost(Some(&a.value), Some(&b.value)))
                .sum::<Cost>();
            Ok(sum)
        }
    }

    fn cost_del(&self, a: Option<&Symbol>) -> Result<Cost, PhlDistanceError> {
        if self.is_zero_cost(a) {
            Ok(Cost::ZERO)
        }
        else if a.is_none() {
            Ok(Cost::INFINITY)
        }
        else {
            let a = a.unwrap();
            self.err_if_not_found(vec![a])?;
            let a_features = &self.system.get_for_symbol(a).unwrap().features;
            let sum = a_features
                .iter()
                .map(|a| feature_cost(Some(&a.value), None))
                .sum::<Cost>();
            Ok(sum)
        }
    }

    fn cost_ins(&self, b: Option<&Symbol>) -> Result<Cost, PhlDistanceError> {
        if self.is_zero_cost(b) {
            Ok(Cost::ZERO)
        }
        else if b.is_none() {
            Ok(Cost::INFINITY)
        }
        else {
            let b = b.unwrap();
            self.err_if_not_found(vec![b])?;
            let a_features = &self.system.get_for_symbol(b).unwrap().features;
            let sum = a_features
                .iter()
                .map(|a| feature_cost(Some(&a.value), None).0)
                .sum::<f64>();
            Ok(Cost(sum))
        }
    }
}

fn feature_cost(left: Option<&FeatureValue>, right: Option<&FeatureValue>) -> Cost {
    let left_value = match left { None => 0.0, Some(v) => v.value };
    let right_value = match right { None => 0.0, Some(v) => v.value};
    if left.is_some() && right.is_some() {
        Cost(f64::abs(left_value - right_value) / 2.0)
    }
    else if left.is_some() {
        if left_value == 0.0 { Cost(0.5) } else { Cost(1.0) }
    }
    else if right.is_some() {
        if right_value == 0.0 { Cost(0.5) } else { Cost(1.0) }
    }
    else {
        // Shouldn't ever get here!
        panic!("{}", "Trying to take a feature cost of two empty values!")
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use crate::distance::feature_distance::FeatureCostCalculator;
    use crate::distance::levenshtein::Action::{DEL, EQ, INS, SUB};
    use crate::distance::levenshtein::{ComputeCost, Cost};
    use crate::distance::phoneme_tokenizer::PhonemeTokenizer;
    use crate::phl::systems::PhonologicalFeatureSystem;

    #[test]
    fn test_feature_distance() {
        let test_cases = vec![
            ("hayes", "", "", vec![]),
            ("hayes", "f", "v", vec![
                (1.0, SUB),
            ]),
            ("hayes", "s", "θ", vec![
                (2.0, SUB),
            ]),
            ("hayes", "e͡ɪ", "a͡ɪ", vec![
                // -low -> +-low      = 1.5/2 = 0.75
                // +-tense -> 0tense  = 0.5/2 = 0.25
                // +front -> -+front  = 1.5/2 = 0.75
                (1.75, SUB),
            ]),
            ("hayes", "a͡ɪ", "e͡ɪ", vec![
                (1.75, SUB),
            ]),
            ("hayes", "kæt", "hæt", vec![
                (6.0, SUB),
                (0.0, EQ),
                (0.0, EQ),
            ]),
            ("hayes", "kæt", "æt", vec![
                (23.0, DEL),
                (0.0, EQ),
                (0.0, EQ),
            ]),
            ("hayes", "æt", "kæt", vec![
                (23.0, INS),
                (0.0, EQ),
                (0.0, EQ),
            ]),
            ("hayes-arpabet", "R", "ER", vec![
                (1.0, SUB),
            ]),
            ("hayes-arpabet", "ER", "R", vec![
                (1.0, SUB),
            ]),
            ("hayes-arpabet", "ER", "UW", vec![
                (8.0, SUB),
            ]),
            ("hayes-arpabet", "UW", "ER", vec![
                (8.0, SUB),
            ]),
            ("hayes-arpabet", "K AE T", "K AE T <spn>", vec![
                (0.0, EQ),
                (0.0, EQ),
                (0.0, EQ),
                (0.0, INS),
            ]),
            ("hayes-arpabet", "K AE T", "K <spn> T", vec![
                (0.0, EQ),
                (0.0, INS),
                (21.5, DEL),
                (0.0, EQ),
            ]),
            ("hayes-arpabet", "S W IH NG Z S Z", "S W IH M IH NG IH SH IH Z", vec![
                (0.0, EQ),
                (0.0, EQ),
                (0.0, EQ),
                (19.5, INS),
                (22.0, INS),
                (0.0, EQ),
                (10.5, SUB),
                (2.0, SUB),
                (22.0, INS),
                (0.0, EQ),
            ])
        ];

        let system_names = test_cases.clone().into_iter().map(|(sn, _, _, _)| sn);
        let systems = load_analyzers(system_names.collect());

        for (system_name, a, b, expected) in test_cases {
            let system = systems.get(&system_name).unwrap();
            let tokenizer = PhonemeTokenizer::build(system);
            let left_tokens = tokenizer.tokenize(a);
            let right_tokens = tokenizer.tokenize(b);

            let calculator = FeatureCostCalculator{ system };
            let actual = calculator.diff_steps(&left_tokens, &right_tokens).unwrap();


            let actual_left_tokens: Vec<_> = actual
                .iter()
                .filter_map(|s| s.expected.clone())
                .collect();
            assert_eq!(left_tokens, actual_left_tokens);

            let actual_right_tokens: Vec<_> = actual
                .iter()
                .filter_map(|s| s.actual.clone())
                .collect();
            assert_eq!(right_tokens, actual_right_tokens);

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

    fn load_analyzers(system_names: HashSet<&str>) -> HashMap<&str, PhonologicalFeatureSystem>{
        system_names
            .clone()
            .into_iter()
            .map(|system_name| (system_name, PhonologicalFeatureSystem::load(system_name).unwrap()))
            .collect()
    }
}