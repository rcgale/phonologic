use std::any::type_name;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static!{
    pub(crate) static ref CLASS_NAME: Regex = Regex::new( r"^.*::(\w+)|").unwrap();
}

pub(crate) trait ClassName {
    fn class_name(&self) -> &str {
        let fq_name = type_name::<Self>();
        let captures = CLASS_NAME.captures(fq_name).unwrap();
        captures.get(1).unwrap_or(captures.get(0).unwrap()).as_str()
    }
}

// impl<T: pyo3::PyClass> ClassName for T {}