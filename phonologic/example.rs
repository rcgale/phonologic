use std::collections::HashMap;
use std::slice::Iter;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct Phoneme {
    symbol: String,
    features: Features,
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct Features {
    voiced: bool,
    labial: bool,
    coronal: bool,
}

pub struct DistinctiveFeatureSystem<'a> {
    entries: Vec<Phoneme>,
    indexed_by_symbol: HashMap<String, &'a Phoneme>,
    indexed_by_features: HashMap<Features, &'a Phoneme>
}

impl<'a> DistinctiveFeatureSystem<'a> {
    pub fn iter(&self) -> Iter<Phoneme> {
        self.entries.iter()
    }
    pub fn get_by_symbol(&self, s: &str) -> Option<Phoneme>{
        match self.indexed_by_symbol.get(s) {
            None => None,
            Some(&p) => Some(p.clone())
        }
    }

    pub fn get_by_features(&self, f: &Features) -> Option<Phoneme>{
        Self::as_clone(self.indexed_by_features.get(f))
    }

    fn as_clone(p: Option<&&Phoneme>) -> Option<Phoneme> {
        match p {
            None => None,
            Some(&p) => Some(p.clone())
        }
    }
}

pub fn build_system<'a>(raw_data: Vec<(String, Features)>) -> DistinctiveFeatureSystem<'a> {
    let entries: Vec<Phoneme> = raw_data
        .into_iter()
        .map(|(symbol, features)| Phoneme{ symbol, features })
        .collect();

    let indexed_by_symbol: HashMap<String, &Phoneme> = entries
        .iter()
        .map(|item| (item.symbol.clone(), item))
        .collect();

    let indexed_by_features: HashMap<Features, &Phoneme> = entries
        .iter()
        .map(|item| (item.features.clone(), item))
        .collect();

    return DistinctiveFeatureSystem { entries, indexed_by_symbol, indexed_by_features };
}

#[cfg(test)]
mod tests {
    use crate::example::{Features, build_system};

    #[test]
    fn test_multi_index() {
        let my_phonemes = vec![
            ("p".to_string(), Features{ labial: true, coronal: false, voiced: true }),
            ("b".to_string(), Features{ labial: true, coronal: false, voiced: false }),
            ("t".to_string(), Features{ labial: false, coronal: false, voiced: true }),
            ("d".to_string(), Features{ labial: false, coronal: false, voiced: false }),
            ("k".to_string(), Features{ labial: false, coronal: true, voiced: true }),
            ("g".to_string(), Features{ labial: false, coronal: true, voiced: false }),
        ];
        let my_system = build_system(my_phonemes);

        let p_by_symbol = my_system.get_by_symbol("p").unwrap();
        let p_by_features = my_system.get_by_features(
            &Features{ labial: true, coronal: false, voiced: true }
        ).unwrap();
        assert_eq!(p_by_symbol.symbol, "p");
        assert_eq!(p_by_symbol, p_by_features);

        let g_by_symbol = my_system.get_by_symbol("g").unwrap();
        let g_by_features = my_system.get_by_features(
            &Features{ labial: false, coronal: true, voiced: false }
        ).unwrap();
        assert_eq!(g_by_symbol.symbol, "p");
        assert_eq!(g_by_symbol, g_by_features);

    }
}