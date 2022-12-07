use js_sys::Array;
use wasm_bindgen::prelude::*;

use phonologic::distance::feature_distance::FeatureCostCalculator;
use phonologic::distance::levenshtein::{Action, ComputeCost, LevenshteinStep};
use phonologic::distance::phoneme_distance::PhonemeCostCalculator;
use phonologic::distance::phoneme_tokenizer::PhonemeTokenizer;
use phonologic::errors::PhlDistanceError;
use phonologic::phl::systems::PhonologicalFeatureSystem;
use phonologic::phl::tokenizer;
use phonologic::phl::tokenizer::Symbol;

#[wasm_bindgen(inspectable)]
#[derive(Clone)]
pub struct Analysis {
    pub(crate) steps: Vec<AnalysisStep>,
    pub cost: f64,
    pub length: f64,
    #[wasm_bindgen(js_name = errorRate)]
    pub error_rate: f64,
}

#[wasm_bindgen]
impl Analysis {
    #[wasm_bindgen(getter)]
    pub fn steps(&self) -> Array {
        self.steps.iter().cloned().map(JsValue::from).collect()
    }
}

#[wasm_bindgen(inspectable)]
#[derive(Clone)]
pub struct AnalysisStep {
    pub action: AnalysisAction,
    pub(crate) left: String,
    pub(crate) right: String,
    pub cost: f64,
    pub length: f64,
}

#[wasm_bindgen]
impl AnalysisStep {
    #[wasm_bindgen(getter)]
    pub fn left(&self) -> String {
        self.left.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn right(&self) -> String {
        self.right.clone()
    }
}

impl AnalysisStep {
    fn from(levenshtein_step: &LevenshteinStep<Symbol>, length: f64) -> Self {
        let action = levenshtein_step.action.into();
        let left = levenshtein_step.expected.clone().unwrap_or(Symbol::new("")).to_string();
        let right = levenshtein_step.actual.clone().unwrap_or(Symbol::new("")).to_string();
        let cost = levenshtein_step.cost.0;
        AnalysisStep { action, left, right, cost, length }
    }
}


#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum AnalysisAction {
    EQ = "EQ",
    DEL = "DEL",
    INS = "INS",
    SUB = "SUB",
}

impl From<Action> for AnalysisAction {
    fn from(a: Action) -> Self {
        match a {
            Action::EQ => EQ,
            Action::DEL => DEL,
            Action::INS => INS,
            Action::SUB => SUB,
        }
    }
}

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug)]
pub struct FeatureDelta {
    pub(crate) name: String,
    pub left: FeatureValue,
    pub right: FeatureValue,
    pub cost: f64,
}

#[wasm_bindgen]
impl FeatureDelta {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum FeatureValue {
    Plus = "+",
    PlusMinus = "+-",
    Zero = "0",
    MinusPlus = "-+",
    Minus = "-",
}

impl From<tokenizer::FeatureValue> for FeatureValue {
    fn from(fv: tokenizer::FeatureValue) -> Self {
        if fv.symbol == "+" { FeatureValue::Plus }
        else if fv.symbol == "+-" { FeatureValue::PlusMinus }
        else if fv.symbol == "0" { FeatureValue::Zero }
        else if fv.symbol == "-+" { FeatureValue::MinusPlus }
        else if fv.symbol == "-" { FeatureValue::Minus }
        else {
            panic!("Unknown feature value {fv:?}")
        }
    }
}

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug)]
pub struct FeatureDeltaCollection {
    deltas: Vec<FeatureDelta>
}

#[wasm_bindgen]
impl FeatureDeltaCollection {
    #[wasm_bindgen(getter)]
    pub fn deltas(&self) -> Array {
        self.deltas.iter().cloned().map(JsValue::from).collect()
    }
}

#[wasm_bindgen(inspectable)]
pub struct PhlAnalyzer {
    system: PhonologicalFeatureSystem,
    tokenizer: PhonemeTokenizer
}

impl PhlAnalyzer {
    fn analysis<C: ComputeCost<Symbol>, LFn: Fn(&Vec<Symbol>) -> f64>(
        &self,
        calculator: &C,
        left: &str,
        right: &str,
        length_fn: LFn
    ) -> Result<Analysis, PhlDistanceError> {
        let left_tokens = self.tokenizer.tokenize(left);
        let right_tokens = self.tokenizer.tokenize(right);
        let length = length_fn(&left_tokens);
        let steps = calculator.diff_steps(&left_tokens, &right_tokens);
        let steps = steps?;
        let analysis = self.compile_analysis(steps, length);
        Ok(analysis)
    }

    fn compile_analysis(
        &self,
        levenshtein_steps: Vec<LevenshteinStep<Symbol>>,
        length: f64
    ) -> Analysis {
        let steps: Vec<_> = levenshtein_steps
            .into_iter()
            .map(|step| AnalysisStep::from(&step, length))
            .collect();
        let cost = steps.iter().map(|step| step.cost).sum();
        let error_rate = if length != 0.0 { cost / length as f64 } else { 0.0 };
        Analysis { steps, cost, length, error_rate }
    }
}

#[wasm_bindgen]
impl PhlAnalyzer {
    #[wasm_bindgen(constructor)]
    pub fn new(system_name: &str) -> Self {
        let system = PhonologicalFeatureSystem::load(system_name);
        let system = system.unwrap_throw();
        let tokenizer = PhonemeTokenizer::build(&system);
        Self { system, tokenizer }
    }

    #[wasm_bindgen(method, js_name = featureDiff)]
    pub fn feature_diff(&self, left: &str, right: &str) -> Analysis {
        let calculator = FeatureCostCalculator { system: &self.system };
        let length_fn = |tokens: &Vec<Symbol>| { (tokens.len() * self.system.num_features) as f64};
        let analysis = self.analysis(&calculator, left, right, length_fn);
        analysis.unwrap()
    }

    #[wasm_bindgen(method, js_name = phonemeDiff)]
    pub fn phoneme_diff(&self, left: &str, right: &str) -> Analysis {
        let calculator = PhonemeCostCalculator { system: &self.system };
        let length_fn = |tokens: &Vec<Symbol>| tokens.len() as f64;
        let analysis = self.analysis(&calculator, left, right, length_fn);
        analysis.unwrap()
    }

    #[wasm_bindgen(method, js_name = featureDeltas)]
    pub fn feature_deltas(&self, left: &str, right: &str) -> FeatureDeltaCollection {
        let left_tokens = self.tokenizer.tokenize(left);
        let right_tokens = self.tokenizer.tokenize(right);
        if left_tokens.len() != 1 || right_tokens.len() != 1 {
            panic!("Invalid input {left} / {right}")
        }
        let left_token = &left_tokens[0];
        let right_token = &right_tokens[0];
        let analysis = self.feature_diff(&left.to_string(), &right.to_string());
        let deltas = self.system.deltas(left_token, right_token)
            .into_iter()
            .map(|d| FeatureDelta{
                name: d.name,
                left: d.left.into(),
                right: d.right.into(),
                cost: analysis.cost,
            })
            .collect();
        FeatureDeltaCollection{ deltas }
    }
}