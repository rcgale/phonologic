use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
// use pyo3::prelude::*;
use regex::Captures;
use crate::errors::PhlParseError;
use crate::errors::PhlParseError::{InvalidFeatureNameError, InvalidFeatureValueError, InvalidTokenError, SyntaxError};
use crate::phl::spec::{FEATURE_SET_TOKENIZER, SYMBOL, TOKENIZER};
use crate::phl::tokenizer::TokenizerCaptures::{Comment, Illegal, Separator, Token};

// #[pyclass]
#[derive(Debug, PartialEq, Clone)]
pub struct Definition {
    // #[pyo3(get)]
    pub(crate) symbol: Symbol,
    // #[pyo3(get)]
    pub(crate) items: Vec<DefinitionItem>
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum DefinitionItem {
    Sym(Symbol),
    Feats(Vec<Feature>),
}

impl DefinitionItem {
    pub(crate) fn feat(&self) -> Option<Vec<Feature>> {
        match self {
            Self::Sym(_) => None,
            Self::Feats(f) => Some(f.clone()),
        }
    }
}

// #[pyclass]
#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Symbol(pub String);

impl Symbol {
    pub fn is_class(&self) -> bool {
        self.len() > 0 && self.0.starts_with("<") && self.0.ends_with(">")
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

}

impl Symbol {
    pub fn new(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// #[pyclass]
#[derive(Eq, PartialEq, Clone, Hash)]
pub struct Feature {
    // #[pyo3(get)]
    pub(crate) value: FeatureValue,
    // #[pyo3(get)]
    pub(crate) name: String
}

impl Debug for Feature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{}", self.value, self.name)
    }
}

impl From<(&str, &str)> for Feature {
    fn from(pair: (&str, &str)) -> Self {
        let (value, name) = pair;
        Self { value: value.into(), name: name.to_string() }
    }
}

pub(crate) trait FeatureVector {
    fn apply_features(&self, other: &Self) -> Self;
}

impl FeatureVector for Vec<Feature> {
    fn apply_features(&self, other: &Self) -> Self {
        let to_apply: HashMap<_, _> = other
            .clone()
            .into_iter()
            .filter(|f| !f.value.value.is_nan())
            .map(|f| (f.name.clone(), f))
            .collect();
        self
            .clone()
            .into_iter()
            .map(|f| to_apply.get(&f.name).unwrap_or(&f).clone())
            .collect()
    }
}

// #[pymethods]
// impl Feature {
//     #[new]
//     fn __new__(value: FeatureValue, name: String) -> Self {
//         Self { value, name }
//     }
//
//     fn __repr__(&self) -> String {
//         let cls = self.class_name();
//         let value = &self.value.__repr__();
//         let name = &self.name;
//         format!("{cls}(\"{value}\", {name})")
//     }
// }

// #[pyclass]
#[derive(Clone)]
pub struct FeatureValue {
    // #[pyo3(get)]
    pub symbol: String,
    // #[pyo3(get)]
    pub(crate) value: f64
}

impl Debug for FeatureValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol.clone())
    }
}

impl PartialEq for FeatureValue {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
    }
}
impl Eq for FeatureValue {}

impl Hash for FeatureValue {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.symbol.hash(hasher)
    }
}

impl From<&str> for FeatureValue {
    fn from(symbol: &str) -> Self {
        if symbol == "0" {
            Self { symbol: symbol.to_string(), value: 0.0 }
        }
        else if symbol == "-" {
            Self { symbol: symbol.to_string(), value: -1.0 }
        }
        else if symbol == "+" {
            Self { symbol: symbol.to_string(), value: 1.0 }
        }
        else if symbol == "+-" {
            Self { symbol: symbol.to_string(), value: 0.5 }
        }
        else if symbol == "-+" {
            Self { symbol: symbol.to_string(), value: -0.5 }
        }
        else if symbol == "?" {
            Self { symbol: symbol.to_string(), value: f64::NAN }
        }
        else {
            Self { symbol: symbol.to_string(), value: f64::NAN }
        }
    }
}

// #[pymethods]
// impl FeatureValue {
//     #[new]
//     fn __new__(value: f64) -> Self {
//         if value == 0.0 {
//             Self {symbol: "0".into(), value}
//         }
//         else if value == -1.0 {
//             Self {symbol: "-".into(), value}
//         }
//         else if value == 1.0 {
//             Self {symbol: "+".into(), value}
//         }
//         else if value == 0.5 {
//             Self {symbol: "+-".into(), value}
//         }
//         else if value == -0.5 {
//             Self {symbol: "-+".into(), value}
//         }
//         else {
//             panic!("Unrecognized FeatureValue {value}")
//         }
//     }
//
//     fn __repr__(&self) -> String {
//         let symbol = &self.symbol;
//         let value = self.value;
//         format!("{symbol}{value}")
//     }
// }

enum TokenizerCaptures {
    Token(String),
    Comment(String),
    Separator(String),
    Illegal(String),
}

impl<'a> From<Captures<'a>> for TokenizerCaptures {
    fn from(cap: Captures) -> Self {
        if cap.name("token").is_some() {
            let token = cap.name("token").map(|v| v.as_str()).unwrap();
            Token(token.to_string())
        }
        else if cap.name("comment").is_some() {
            let token = cap.name("comment").map(|v| v.as_str()).unwrap();
            Comment(token.to_string())
        }
        else if cap.name("separator").is_some() {
            let token = cap.name("separator").map(|v| v.as_str()).unwrap();
            Separator(token.to_string())
        }
        else {
            let illegal = cap.name("illegal").map(|v| v.as_str()).unwrap_or("");
            Illegal(illegal.to_string())
        }
    }
}

pub(crate) fn tokenize(statement: &str) -> Result<Vec<String>, PhlParseError> {
    let mut tokens = vec![];
    for cap in TOKENIZER.captures_iter(statement) {
        let cap: TokenizerCaptures = cap.into();
        let result = match cap {
            Token(t) => Ok(Some(t)),
            Comment(_) | Separator(_) => Ok(None),
            Illegal(ill) => Err(InvalidTokenError(ill))
        }?;
        if result.is_none() { continue }
        let token = result.unwrap();
        if token.len() == 0 {
            return Err(InvalidTokenError(format!("The tokenizer regular expression returned an empty token.")));
        }
        tokens.insert(tokens.len(), token.clone())
    }
    Ok(tokens)
}

pub(crate) fn parse_features(feature_set_string: &str) -> Result<Vec<Feature>, PhlParseError> {
    let feature_set_string = feature_set_string.trim();
    if feature_set_string.len() == 0 {
        return Ok(vec![])
    }
    let mut tokens: Vec<Feature> = vec![];
    for cap in FEATURE_SET_TOKENIZER.captures_iter(feature_set_string) {
        if cap.name("value") == None {
            continue;
        }
        let value = cap.name("value").map(|v| v.as_str()).unwrap_or("");
        let feature = cap.name("feature").map(|v| v.as_str()).unwrap_or("");
        let separator = cap.name("separator").map(|v| v.as_str()).unwrap_or("");
        // let bracket = cap.name("bracket").map(|v| v.as_str()).unwrap_or("");
        let illegal = cap.name("illegal").map(|v| v.as_str()).unwrap_or("");
        if separator.len() > 0 {
            continue;
        }
        if illegal.len() > 0 {
            return Err(InvalidTokenError(illegal.to_string()))
        }
        if value.len() == 0 {
            return Err(InvalidFeatureValueError(feature_set_string.to_string()))
        }
        if feature.len() == 0 {
            return Err(InvalidFeatureNameError(feature_set_string.to_string()))
        }
        tokens.insert(tokens.len(), (value, feature).into() )
    }
    Ok(tokens)
}

// #[pyfunction(name="_parse_statement")]
pub(crate) fn tokenize_definition(s: &str) -> Result<Option<Definition>, PhlParseError> {
    // let mut definition: Vec<String> = vec![];
    let tokens = tokenize(s)?;
    if tokens.len() == 0 {
        return Ok(None);
        // return Err(InvalidTokenError("No tokens found in statement: {s}".to_string()))
        // panic!("No tokens found in statement: {s}");
    }
    if tokens.len() < 3 || tokens[1] != "=" {
        return Err(SyntaxError(s.to_string()))
    }
    let symbol = Symbol(tokens[0].to_string());
    let mut definition: Vec<DefinitionItem> = vec![];
    for token in tokens[2..].iter() {
        if token.trim().len() == 0 {
            continue;
        }
        else if SYMBOL.is_match(token.as_str()) {
            definition.push(DefinitionItem::Sym(Symbol(token.into())));
        }
        else if token.starts_with("[") && token.ends_with("]") {
            let features = parse_features(token.as_str())?;
            definition.push(DefinitionItem::Feats(features));
        }
        else {
            return Err(PhlParseError::InvalidTokenError(token.to_string()))
        }

    }
    return Ok(Some(Definition { symbol, items: definition }))

}


#[derive(Debug)]
pub(crate) struct PhonologicalSystemParseError {
    pub(crate) message: String
}

impl Display for PhonologicalSystemParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for PhonologicalSystemParseError {}

// fn parse_features(feature_set_string: &str) -> Vec<Feature> {
//     tokenize_feature_set(feature_set_string)
// }


#[cfg(test)]
mod tests {
    use crate::phl::tokenizer::{FeatureVector, parse_features, tokenize};

    #[test]
    fn test_parse() {
        let test_cases = [
            (
                "<glide> = [-syl, -cons, +appr, +son]",
                vec!["<glide>", "=", "[-syl, -cons, +appr, +son]"]
            ),
            (
                "t̪͡θ = <voiceless> <dental> <affricate> [-strid, -lat]",
                vec!["t̪͡θ", "=", "<voiceless>", "<dental>", "<affricate>", "[-strid, -lat]"]
            ),
            (
                "<a>=<b>",
                vec!["<a>", "=", "<b>"]
            ),
            (
                "<a> =<b>",
                vec!["<a>", "=", "<b>"]
            ),
            (
                "<a>= <b>",
                vec!["<a>", "=", "<b>"]
            ),
            (
                "a͡ʊ = a [-+high, -+low, 0-tense, -+back, -+round]",
                vec!["a͡ʊ", "=", "a", "[-+high, -+low, 0-tense, -+back, -+round]"],
            ),
            (
                "# This is a comment.",
                vec![],
            ),
            (
                "<a> = <b>  # This is a comment.",
                vec!["<a>", "=", "<b>"],
            ),
            (
                "# Diphthongs use special symbols: `+-` means the feature goes from present to absent, `-+` is the reverse.",
                vec![]
            )
        ];
        for (statement, expected) in test_cases {
            let tokens = tokenize(statement).unwrap();
            assert_eq!(tokens, expected)
        }
    }

    #[test]
    fn test_parse_features() {
        let test_cases: Vec<(&str, Vec<(&str, &str, f64)>)> = vec![
            (
                "[]",
                vec![]
            ),
            (
                "[-syl]",
                vec![
                    ("-", "syl", -1.0)
                ]
            ),
            (
                "[-syl, +appr, 0son]",
                vec![
                    ("-", "syl", -1.0),
                    ("+", "appr", 1.0),
                    ("0", "son", 0.0),
                ]
            ),
        ];
        for (s, expected) in test_cases.into_iter() {
            let features = parse_features(s).unwrap();
            let feature_tuples: Vec<_> = features.iter().map(|f| (&(&f).value.symbol[..], &(&f).name[..], f.value.value)).collect();
            assert_eq!(expected, feature_tuples);
        }
    }

    #[test]
    fn test_merge_features() {
        let test_cases = vec![
            (
                "[0feat1, 0feat2, 0feat3]",
                "[+feat1, -feat2]",
                "[+feat1, -feat2, 0feat3]",
            ),
            (
                "[?feat1, ?feat2, 0feat3]",
                "[+feat1, -feat2]",
                "[+feat1, -feat2, 0feat3]",
            ),
        ];
        for (default, update, expected) in test_cases {
            let default = parse_features(default).unwrap();
            let update = parse_features(update).unwrap();
            let expected = parse_features(expected).unwrap();
            let merged = default.apply_features(&update);
            assert_eq!(merged, expected)
        }

    }
}