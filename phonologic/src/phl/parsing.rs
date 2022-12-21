use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use pest::iterators::Pair;
use pest::Parser;
use std::fs;
use std::collections::HashMap;
use crate::errors::PhlParseError;

#[derive(Parser)]
#[grammar = "phl/phl.pest"]
struct PhlParser;

pub(crate) type ParseResult<T> = Result<T, PhlParseError>;

pub(crate) trait Parseable: Sized + Debug {
    const RULE: Rule;

    fn from_pair(pair: Pair<Rule>) -> ParseResult<Self>;

    fn parse<'a, T: Into<&'a str>>(s: T) -> ParseResult<Self> {
        match PhlParser::parse(Self::RULE, s.into()) {
            Ok(mut p) => Self::from_pair(p.next().unwrap()),
            Err(e) => Err(PhlParseError::FileReadError(format!("{e}")))
        }
    }
}

#[derive(Debug)]
struct Name(String);

impl Parseable for Name {
    const RULE: Rule = Rule::name;
    fn from_pair(pair: Pair<Rule>) -> ParseResult<Self> {
        Ok(Self(pair.to_string()))
    }
}

impl Parseable for Symbol {
    const RULE: Rule = Rule::symbol;
    fn from_pair(pair: Pair<Rule>) -> ParseResult<Self> {
        Ok(Self::new(pair.as_str()))
    }
}

#[derive(Debug)]
struct ClassName(String);

impl Parseable for ClassName {
    const RULE: Rule = Rule::class_name;
    fn from_pair(pairs: Pair<Rule>) -> ParseResult<Self> {
        Ok(Self(pairs.to_string()))
    }
}

impl Parseable for FeatureValue {
    const RULE: Rule = Rule::feature;
    fn from_pair(pair: Pair<Rule>) -> ParseResult<Self> {
        Ok(FeatureValue::new(pair.as_str()))
    }
}

impl Parseable for Feature {
    const RULE: Rule = Rule::feature;
    fn from_pair(pair: Pair<Rule>) -> ParseResult<Self> {
        let mut inner_rules = pair.into_inner();
        let value = FeatureValue::from_pair(inner_rules.next().unwrap())?;
        let name = inner_rules.next().unwrap().as_str().to_string();
        Ok(Feature { value, name })
    }
}

pub(crate) type FeatureSet = Vec<Feature>;

impl Parseable for FeatureSet {
    const RULE: Rule = Rule::feature_set;
    fn from_pair(pair: Pair<Rule>) -> ParseResult<Self> {
        let features: FeatureSet = pair
            .into_inner()
            .map(|p| Feature::from_pair(p).unwrap())
            .collect();
        Ok(features)
    }
}


impl Parseable for DefinitionItem {
    const RULE: Rule = Rule::definition_item;
    fn from_pair(pair: Pair<Rule>) -> ParseResult<Self> {
        let inner = pair.into_inner().next().unwrap();
        Ok(match inner.as_rule() {
            Rule::feature_set => DefinitionItem::Feats(FeatureSet::from_pair(inner)?),
            Rule::name | Rule::class_name => DefinitionItem::Sym(Symbol::from_pair(inner)?),
            _ => panic!("wuh oh")
        })
    }
}

impl Parseable for Definition {
    const RULE: Rule = Rule::feature_definition;
    fn from_pair(pair: Pair<Rule>) -> ParseResult<Self> {
        let mut inner = pair.into_inner();
        let symbol = Symbol::from_pair(inner.next().unwrap())?;
        let items = inner
            .next()
            .unwrap()
            .into_inner()
            .map(|p| DefinitionItem::from_pair(p).unwrap())
            .collect();
        Ok(Definition {symbol, items})
    }
}

pub(crate) type PhlFile = Vec<Definition>;

impl Parseable for PhlFile {
    const RULE: Rule = Rule::phl_file;
    fn from_pair(pair: Pair<Rule>) -> ParseResult<Self> {
        let results: Vec<_> = pair
            .into_inner()
            .filter(|p| p.as_rule() == Rule::feature_definition)
            .map(|p| Definition::from_pair(p))
            .collect();
        for error_result in results.iter().filter(|r| r.is_err()) {
            return Err(error_result.as_ref().unwrap_err().clone());
        }
        Ok(results.into_iter().map(|r| r.unwrap()).collect())
    }
}

pub(crate) fn parse_file(filepath: &str) -> Result<PhlFile, PhlParseError> {
    match fs::read_to_string(filepath) {
        Ok(file_contents) => Ok(PhlFile::parse(file_contents.as_str())),
        Err(e) => Err(PhlParseError::FileReadError(e.to_string()))
    }?
}


#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::phl::parsing::DefinitionItem::{Feats, Sym};
    use super::*;

    #[test]
    fn test_symbol_name() {
        let test_cases = vec![
            (true, "a"),
            (true, "aa"),
            (true, "a1"),
            (true, "t̪͡θ"),
            (false, ""),
            (false, "1a"),
            (false, "<a>"),
        ];
        assert_parse(test_cases, Name::parse);
    }

    #[test]
    fn test_class_feature_value() {
        let test_cases = vec![
            (true, "+abc"),
            (true, "0delrel"),
            (false, ""),
            (false, "delrel+"),
        ];
        assert_parse(test_cases, FeatureValue::parse)
    }

    #[test]
    fn test_class_feature() {
        let test_cases = vec![
            (true, "+abc"),
            (true, "0delrel"),
            (false, ""),
            (false, "delrel+"),
        ];
        assert_parse(test_cases, FeatureValue::parse)
    }

    #[test]
    fn test_feature_set() {
        let test_cases = vec![
            (true, "[+abc]"),
            (true, "[+abc, -def]"),
        ];
        assert_parse(test_cases, FeatureSet::parse)
    }

    #[test]
    fn test_definition_item() {
        let test_cases = vec![
            (true, "<def>"),
            (true, "[+abc, -def]"),
        ];
        assert_parse(test_cases, DefinitionItem::parse)
    }

    #[test]
    fn test_definition() {
        let test_cases = vec![
            (true, "a = [+abc, -def]"),
        ];
        assert_parse(test_cases, Definition::parse)
    }

    #[test]
    fn test_definition_2() {
        let test_cases = [
            (
                "a = [+abc, -def]",
                Definition {
                    symbol: Symbol::new("a"),
                    items: vec![
                        DefinitionItem::Feats(vec![
                            Feature::from(("+", "abc")),
                            Feature::from(("-", "def")),
                        ])
                    ]
                }
            ),
            (
                "a = <b> [+abc, -def]",
                Definition {
                    symbol: Symbol::new("a"),
                    items: vec![
                        Sym(Symbol::new("<b>")),
                        Feats(vec![
                            Feature::from(("+", "abc")),
                            Feature::from(("-", "def")),
                        ])
                    ]
                }
            ),
            (
                "<a> = [+abc, -def] b",
                Definition {
                    symbol: Symbol::new("<a>"),
                    items: vec![
                        Feats(vec![
                            Feature::from(("+", "abc")),
                            Feature::from(("-", "def")),
                        ]),
                        Sym(Symbol::new("b")),
                    ]
                }
            ),
        ];

        for (s, expect) in test_cases {
            let actual = Definition::parse(s).unwrap();
            assert_eq!(actual, expect);
        }
    }

    fn assert_parse<'a, T, F>(test_cases: Vec<(bool, &'a str)>, parse: F)
        where T: Parseable,
              F: Fn (&'a str) -> ParseResult<T>
    {
        for (should_work, s) in test_cases {
            match parse(s.into()) {
                Ok(x) => should_work || panic!("{x:?}"),
                Err(x) => !should_work || panic!("{x:?}")
            };
        }

    }

    #[test]
    fn test_parse_file() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"));
        let test_file = path.join("assets/systems/hayes-ipa-arpabet.phl");
        let test_file =  test_file.to_str().unwrap();

        let parsed = match parse_file(test_file) {
            Ok(p) => p,
            Err(e) => panic!("{e}")
        };
        println!("{parsed:?}");
        assert_eq!(parsed.len(), 220);
    }
}

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

static DEFAULT_SYMBOL_STR: &str = "<default>";

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Symbol(pub String);


impl Symbol {
    pub(crate) fn default() -> Self {
        Self::new(DEFAULT_SYMBOL_STR)
    }

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
        Self { value: FeatureValue::new(value), name: name.to_string() }
    }
}

pub(crate) trait FeatureVectorFunc {
    fn apply_features(&self, other: &Self) -> Self;
}

impl FeatureVectorFunc for Vec<Feature> {
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

impl FeatureValue {
    pub(crate) fn new(symbol: &str) -> Self {
        match symbol {
            "-" =>  Self { symbol: symbol.to_string(), value: -1.0 },
            "-+" => Self { symbol: symbol.to_string(), value: -0.5 },
            "0" =>  Self { symbol: symbol.to_string(), value: 0.0 },
            "+-" => Self { symbol: symbol.to_string(), value: 0.5 },
            "+" =>  Self { symbol: symbol.to_string(), value: 1.0 },
            "?" =>  Self { symbol: symbol.to_string(), value: f64::NAN },
            &_ =>   Self { symbol: symbol.to_string(), value: f64::NAN }
        }
    }
}
