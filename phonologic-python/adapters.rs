// use std::fmt;
// use wasm_bindgen::prelude::*;
// use js_sys::Array;
// use pyo3::prelude::*;
// use pyo3;
// use crate::helpers::introspection::ClassName;
// use crate::distance::levenshtein::{Action, Cost, edit_distance, LevenshteinCost};
// use crate::phl::tokenizer::*;
// use string_join::Join;
//
//
//
// #[pyfunction(name="_levenshtein_str")]
// // #[wasm_bindgen]
// pub fn levenshtein_str(a: &str, b: &str) -> LevenshteinResult {
//     let a = a.chars().collect();
//     let b = b.chars().collect();
//     edit_distance(a, b).into()
// }
//
// #[pymodule]
// fn wasm(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(levenshtein_str, m)?)?;
//     m.add_function(wrap_pyfunction!(parse_features, m)?)?;
//     // m.add_function(wrap_pyfunction!(parse_statement, m)?)?;
//     m.add_function(wrap_pyfunction!(tokenize, m)?)?;
//     // m.add_class::<Definition>()?;
//     // m.add_class::<Feature>()?;
//     // m.add_class::<FeatureValue>()?;
//     m.add_class::<LevenshteinResult>()?;
//     m.add_class::<Symbol>()?;
//     Ok(())
// }
//
// // #[wasm_bindgen]
// #[pyclass]
// pub struct LevenshteinResult {
//     #[pyo3(get)]
//     pub total_cost: Cost,
//     #[pyo3(get)]
//     pub steps: Vec<JsLevenshteinStepStr>,
// }
//
// // #[wasm_bindgen]
// impl LevenshteinResult {
//     // #[wasm_bindgen(getter)]
//     pub fn steps(&self) -> Array {
//         self.steps.iter().cloned().map(JsValue::from).collect()
//     }
// }
//
// #[pymethods]
// impl LevenshteinResult {
//     fn __repr__(&self) -> String {
//         let cls = self.class_name();
//         let total_cost = self.total_cost.__repr__();
//         let steps_parts = ", ".join(self.steps.iter().map(|item| item.__repr__()));
//         let steps = format!("[{steps_parts}]");
//         format!("{cls}({total_cost}, {steps}")
//     }
// }
//
// impl<T: fmt::Display + Copy + Eq> From<Vec<LevenshteinCost<T>>> for LevenshteinResult {
//     fn from(steps: Vec<LevenshteinCost<T>>) -> LevenshteinResult {
//         if steps.len() == 0 {
//             return LevenshteinResult {total_cost: Cost::ZERO, steps: vec![] }
//         }
//         LevenshteinResult {
//             total_cost: steps.last().unwrap().cost,
//             steps: steps.iter().map(|s| JsLevenshteinStepStr {
//                 action: s.action,
//                 expected: if s.expected == None { "".to_string() } else { s.expected.unwrap().to_string() },
//                 actual: if s.actual == None { "".to_string() } else { s.actual.unwrap().to_string() },
//                 cost: s.cost
//             }).collect()
//         }
//     }
// }
//
// #[wasm_bindgen]
// #[pyclass]
// #[derive(Debug, Clone, Eq, PartialEq)]
// pub struct JsLevenshteinStepStr {
//     #[pyo3(get)]
//     pub action: Action,
//     #[pyo3(get)]
//     expected: String,
//     #[pyo3(get)]
//     actual: String,
//     #[pyo3(get)]
//     pub cost: Cost,
// }
//
// #[pymethods]
// impl JsLevenshteinStepStr {
//     fn __repr__(&self) -> String {
//         let cls = self.class_name();
//         let action = self.action.__repr__();
//         let expected = &self.expected;
//         let actual = &self.actual;
//         let cost = self.cost.__repr__();
//         format!("{cls}({action}, {expected}, {actual}, {cost})")
//     }
// }
//
// #[wasm_bindgen]
// impl JsLevenshteinStepStr {
//     #[wasm_bindgen(getter)]
//     pub fn expected(&self) -> String {
//         self.expected.clone().into()
//     }
//     #[wasm_bindgen(getter)]
//     pub fn actual(&self) -> String {
//         self.actual.clone().into()
//     }
// }
//
// #[pymethods]
// impl Action {
//     fn __repr__(&self) -> String {
//         format!("{self}")
//     }
// }
//
// #[pymethods]
// impl Cost {
//     fn __repr__(&self) -> String {
//         format!("{self}")
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::adapters::js_python::levenshtein_str;
//     use string_join::Join;
//
//     #[test]
//     fn test_js() {
//         let test_cases: Vec<(&str, &str, f64, Vec<&str>)> = vec![
//             ("", "", 0.0, vec![]),
//             ("A", "A", 0.0, vec!["EQ"]),
//             ("AA", "AA", 0.0, vec!["EQ", "EQ"]),
//             ("A", "B", 1.0, vec!["SUB"]),
//             ("AA", "BB", 2.0, vec!["SUB", "SUB"]),
//             ("AB", "A", 1.0, vec!["EQ", "DEL"]),
//             ("A", "AB", 1.0, vec!["EQ", "INS"]),
//             ("ABC", "BB", 2.0, vec!["SUB", "EQ", "DEL"]),
//             ("BB", "ABC", 2.0, vec!["SUB", "EQ", "INS"]),
//             ("AAA", "AA", 1.0, vec!["EQ", "EQ", "DEL"]),
//             ("AA", "AAA", 1.0, vec!["EQ", "EQ", "INS"]),
//         ];
//
//         for (a, b, expect_cost, actions) in test_cases {
//             let result = levenshtein_str(a, b);
//             let actual_actions = " ".join(result.steps.iter().map(|x| format!("{:?}", x.action)));
//             let expect_actions = " ".join(actions);
//             let actual_cost = if result.steps.len() > 0 { result.steps[result.steps.len() - 1].cost.0 } else { 0.0 };
//             assert_eq!(expect_actions, actual_actions);
//             assert_eq!(expect_cost, actual_cost);
//             println!("{} -> {} = {}", a, b, actual_actions);
//         }
//     }
// }