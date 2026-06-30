use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

trait PhlError: Debug {}

impl Error for dyn PhlError {}

impl Display for dyn PhlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone)]
pub enum PhlParseError {
    InvalidTokenError(String),
    InvalidFeatureValueError(String),
    InvalidFeatureNameError(String),
    SyntaxError(String),
    RedefinedSymbolError(String),
    FileReadError(String),
    MustHaveDefaultError(),
    SymbolNotDefinedError(String),
    UnexpectedFeaturesError(String),
}

impl PhlError for PhlParseError {}

impl Display for PhlParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            PhlParseError::InvalidTokenError(s) => s,
            PhlParseError::InvalidFeatureValueError(s) => s,
            PhlParseError::InvalidFeatureNameError(s) => s,
            PhlParseError::SyntaxError(s) => s,
            PhlParseError::RedefinedSymbolError(s) => s,
            PhlParseError::FileReadError(s) => s,
            PhlParseError::MustHaveDefaultError() => "",
            PhlParseError::SymbolNotDefinedError(s) => s,
            PhlParseError::UnexpectedFeaturesError(s) => s,
        })
    }
}

#[derive(Debug)]
pub enum PhlDistanceError {
    PhonemeNotFoundError(String)
}

impl PhlError for PhlDistanceError {}
