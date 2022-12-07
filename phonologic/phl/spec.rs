use lazy_static::lazy_static;
use regex::Regex;

pub(crate) static DEFAULT_SYMBOL_STR: &str = "<default>";

pub(crate) static P_STR_TOKEN: &str = r"(?P<token>\[[^\]]*\]|=|<[\p{L}\p{M}_\+˗]+>|[\p{L}\p{M}_\+˗]+)";
pub(crate) static P_STR_COMMENT: &str = r"(?P<comment>\s*#.*$)";
pub(crate) static P_STR_SEPARATOR: &str = r"(?P<separator>\s+)";
pub(crate) static P_STR_ILLEGAL: &str = r"(?P<illegal>.+)";

pub(crate) static P_STR_FEATURE_VALUE: &str = r"(?P<value>[+\-0\?]{1,2})";
pub(crate) static P_STR_FEATURE_NAME: &str = r"(?P<feature>[A-Za-z]\w*)";
pub(crate) static P_STR_FEATURE_SEP: &str = r"(?P<separator>\s*,\s+|\s+)";
pub(crate) static P_STR_FEATURE_BRACKET: &str = r"(?P<bracket>^\[|\]$)";

pub fn build_regex(s: String) -> Regex {
    Regex::new(s.as_str()).unwrap()
}

lazy_static! {
    pub(crate) static ref TOKENIZER: Regex = build_regex(
        format!(r"{P_STR_TOKEN}|{P_STR_COMMENT}|{P_STR_SEPARATOR}|{P_STR_ILLEGAL}")
    );

    pub(crate) static ref FEATURE_SET_TOKENIZER: Regex = build_regex(
        format!(r"\s*\[?\s*(?:{P_STR_FEATURE_VALUE}\s*{P_STR_FEATURE_NAME}|{P_STR_FEATURE_SEP}|{P_STR_FEATURE_BRACKET}|{P_STR_ILLEGAL})\s*\]?\s*")
    );

    pub(crate) static ref SYMBOL: Regex = Regex::new(
        r"^<[\p{L}\p{M}_\+˗]+>|[\p{L}\p{M}_\+˗]+$"
    ).unwrap();
}

