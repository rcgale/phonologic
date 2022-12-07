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


#[derive(Debug)]
pub enum PhlDistanceError {
    PhonemeNotFoundError(String)
}

impl PhlError for PhlDistanceError {}
