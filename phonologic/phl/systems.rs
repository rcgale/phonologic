use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use phf::phf_map;
use crate::errors::PhlParseError;
use crate::errors::PhlParseError::{MustHaveDefaultError, RedefinedSymbolError, SymbolNotDefinedError, UnexpectedFeaturesError};
use crate::phl::parser::{parse_file, parse_lines};
use crate::phl::spec::DEFAULT_SYMBOL_STR;
use crate::phl::tokenizer::{Definition, DefinitionItem, Feature, FeatureValue, FeatureVector, Symbol};

pub struct PhonologicalFeatureSystem {
    pub(crate) entries: Vec<PhonologicalFeatureEntry>,
    by_symbol: HashMap<Symbol, usize>,
    // by_features: HashMap<Vec<Feature>, usize>,
    pub(crate) ignore_symbols: HashSet<Symbol>,
    pub(crate) zero_cost_symbols: HashSet<Symbol>,
    pub(crate) separators: HashSet<Symbol>,
    pub num_features: usize,
}

pub struct FeatureDelta {
    pub name: String,
    pub left: FeatureValue,
    pub right: FeatureValue,
}

impl PhonologicalFeatureSystem {
    pub(crate) fn is_zero_cost(&self, s: &Symbol) -> bool {
        self.zero_cost_symbols.contains(s)
    }

    pub fn deltas(&self, a: &Symbol, b: &Symbol) -> Vec<FeatureDelta>{
        let a_features = &self.get_for_symbol(a).unwrap().features;
        let b_features = &self.get_for_symbol(b).unwrap().features;
        a_features
            .iter()
            .zip(b_features)
            .filter(|(a_feat, b_feat)| (a_feat != b_feat))
            .map(|(a_feat, b_feat)| FeatureDelta{
                name: a_feat.name.clone(),
                left: a_feat.value.clone(),
                right: b_feat.value.clone()
            })
            .collect()
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub(crate) enum FeatureSystemIdx {
    Sym(Symbol),
    Feats(Vec<Feature>),
}

impl Into<FeatureSystemIdx> for &Symbol {
    fn into(self) -> FeatureSystemIdx {
        FeatureSystemIdx::Sym(self.clone())
    }
}

impl Into<FeatureSystemIdx> for &Vec<Feature> {
    fn into(self) -> FeatureSystemIdx {
        FeatureSystemIdx::Feats(self.clone())
    }
}

impl PhonologicalFeatureSystem {
    pub fn load(name: &str) -> Result<PhonologicalFeatureSystem, PhlParseError> {
        static PREDEFINED_SYSTEMS: phf::Map<&'static str, &'static str> = phf_map! {
            "hayes" => include_str!("../assets/systems/hayes.phl"),
            "hayes-arpabet" => include_str!("../assets/systems/hayes-arpabet.phl"),
            "hayes-ipa-arpabet" => include_str!("../assets/systems/hayes-ipa-arpabet.phl"),
        };

        let definitions = match PREDEFINED_SYSTEMS.get(name) {
            Some(&predefined_contents) => parse_lines(&predefined_contents.split("\n").collect()),
            None => parse_file(name)
        };
        let definitions = definitions?;

        let system = PhonologicalFeatureSystem::build(&definitions)?;
        Ok(system)
    }

    pub fn build(definitions: &Vec<Definition>) -> Result<Self, PhlParseError> {
        let entries = Self::fold_entries(definitions)?;
        if entries.len() == 0 || entries[0].symbol != Symbol::new(DEFAULT_SYMBOL_STR) {
            return Err(MustHaveDefaultError());
        }
        let num_features = entries[0].features.len();
        let mut by_symbol: HashMap<Symbol, _> = HashMap::new();
        // let mut by_features: HashMap<Vec<Feature>, _> = HashMap::new();
        for (idx, entry) in entries.iter().enumerate() {
            by_symbol.insert(entry.symbol.clone(), idx);
            // by_features.insert(entry.features.clone(), idx);
        }
        let ignore_symbols = HashSet::from([" ", "ˌ", "ˈ", "/", "[", "]"].map(|s| Symbol(s.to_string())));
        let zero_cost_symbols = HashSet::from(["<sil>", "<unk>", "<spn>"].map(|s| Symbol(s.to_string())));
        let separators: HashSet<_> = vec![" "].into_iter().map(|s| Symbol(s.to_string())).collect();
        Ok(Self { entries, by_symbol, ignore_symbols, zero_cost_symbols, separators, num_features })
    }

    pub fn get_for_symbol(&self, symbol: &Symbol) -> Option<&PhonologicalFeatureEntry> {
        match self.by_symbol.get(symbol) {
            Some(&i) => self.entries.get(i),
            None => None
        }
    }

    // pub fn get_for_features(&self, features: &Vec<Feature>) -> Option<&PhonologicalFeatureEntry> {
    //     match self.by_features.get(features) {
    //         Some(&i) => self.entries.get(i),
    //         None => None
    //     }
    // }

    fn fold_entries(definitions: &Vec<Definition>) -> Result<Vec<PhonologicalFeatureEntry>, PhlParseError> {
        let (_, entries, errors) = definitions
            .iter()
            .fold((HashMap::new(), vec![], vec![]), |
                (mut symbol_map, mut entries, mut errors),
                definition
            | {
                let symbol = definition.symbol.clone();
                if symbol_map.contains_key(&symbol) {
                    errors.push(RedefinedSymbolError(format!("{symbol}")));
                    return (symbol_map, entries, errors);
                }
                let features = compute_features(&symbol_map, definition);
                if features.is_err() {
                    errors.push(features.unwrap_err());
                    return (symbol_map, entries, errors);
                }
                let entry = PhonologicalFeatureEntry {symbol, features: features.unwrap() };
                entries.push(entry.clone());
                symbol_map.insert(entry.symbol.clone(), entry);
                (symbol_map, entries, errors)
            });
        for error in errors {
            return Err(error)
        }
        Ok(entries)
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PhonologicalFeatureEntry {
    pub(crate) symbol: Symbol,
    // definition: Vec<DefinitionItem>,
    pub(crate) features: Vec<Feature>
}

fn compute_features(
        symbol_map: &HashMap<Symbol, PhonologicalFeatureEntry>,
        definition: &Definition,
) -> Result<Vec<Feature>, PhlParseError> {
    let default_symbol = Symbol::new(DEFAULT_SYMBOL_STR);

    if definition.symbol == default_symbol {
        if definition.items.len() != 1 || definition.items[0].feat().is_none() {
            return Err(MustHaveDefaultError())
        }
        let features = definition.items[0].feat().unwrap();
        // return FeatureVector(features, features, is_class=True)
        return Ok(features)
    }

    let default_entry = symbol_map.get(&default_symbol);
    if default_entry.is_none() {
        return Err(MustHaveDefaultError())
    }
    let default_features = symbol_map.get(&default_symbol).unwrap();
    let mut features = default_features.features.clone();

    for item in &definition.items {
        features = match item {
            DefinitionItem::Sym(symbol) => {
                if !symbol_map.contains_key(&symbol) {
                    return Err(SymbolNotDefinedError(symbol.to_string()))
                }
                let base = symbol_map.get(&symbol).unwrap().features.clone();
                base.apply_features(&features)
            },
            DefinitionItem::Feats(f) => {
                if symbol_map.len() > 0 {
                    features = features.apply_features(f)
                }
                if definition.symbol == default_symbol {
                    features = f.clone();
                }
                features
            },
        };
    }
    if features.len() != default_features.features.len() {
        let left_features: HashSet<_> = default_features.features.iter().map(|f| f.name.to_string()).collect();
        let right_features: HashSet<_> = features.iter().map(|f| f.name.to_string()).collect();
        let diff = right_features.symmetric_difference(&left_features);
        return Err(UnexpectedFeaturesError(format!("{diff:?}")));
    }
    // let is_class = definition.symbol.starts_with("<") && definition.symbol.ends_with(">");
    // return FeatureVector(features, default_features, is_class)
    Ok(features)
}

#[cfg(test)]
mod tests {
    use wasm_bindgen::UnwrapThrowExt;
    use crate::phl::parser::parse_lines;
    use crate::phl::systems::PhonologicalFeatureSystem;

    #[test]
    fn test_build_system() {
        let test_cases = [
            (
                vec![
                    "<default> = [0feat1, 0feat2, 0feat3]",
                    "a = [+feat1, -feat2]",
                ],
                vec![
                    "<default> = [0feat1, 0feat2, 0feat3]",
                    "a = [+feat1, -feat2, 0feat3]",
                ],
            ),
            (
                vec![
                    "<default> = [?feat1, ?feat2, 0feat3]",
                    "a = [+feat1, -feat2]",
                ],
                vec![
                    "<default> = [?feat1, ?feat2, 0feat3]",
                    "a = [+feat1, -feat2, 0feat3]",
                ],
            ),
            (
                vec![
                    "<default> = [?feat1, ?feat2, 0feat3]",
                    "<myclass> = [+feat1, -feat2]",
                    "a = <myclass> [+feat1]",
                    "b = <myclass> [-feat1]",
                ],
                vec![
                    "<default> = [?feat1, ?feat2, 0feat3]",
                    "a = [+feat1, -feat2, 0feat3]",
                    "b = [-feat1, -feat2, 0feat3]",
                ],
            )
        ];
        for (actual_lines, check_entries ) in test_cases {
            let actual_definitions = parse_lines(&actual_lines).unwrap_throw();
            let actual_system = PhonologicalFeatureSystem::build(&actual_definitions).unwrap_throw();

            let expect_definitions = parse_lines(&check_entries).unwrap_throw();
            let expect_system = PhonologicalFeatureSystem::build(&expect_definitions).unwrap_throw();

            for expect_entry in expect_system.entries {
                let actual_entry = actual_system.get_for_symbol(&expect_entry.symbol).unwrap_throw();
                let expect_str: Vec<_> = expect_entry.features.iter().map(|f| format!("{f:?}")).collect();
                let actual_str: Vec<_> = actual_entry.features.iter().map(|f| format!("{f:?}")).collect();
                assert_eq!(expect_str, actual_str);
            }
        }
    }
}
