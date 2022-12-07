use std::ops::Add;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Sum;
use crate::errors::PhlDistanceError;

pub trait ComputeCost<T: Levenshteinable> {
    fn cost_sub(&self, a: Option<&T>, b: Option<&T>) -> Result<Cost, PhlDistanceError>;

    fn cost_del(&self, a: Option<&T>) -> Result<Cost, PhlDistanceError>;

    fn cost_ins(&self, b: Option<&T>) -> Result<Cost, PhlDistanceError>;

    fn cost_eq(&self, a: Option<&T>, b: Option<&T>) -> Result<Cost, PhlDistanceError> {
        if !a.is_none() && !b.is_none() && a.as_ref().unwrap() == b.as_ref().unwrap() {
            Ok(Cost::ZERO)
        }
        else {
            Ok(Cost::INFINITY)
        }
    }

    fn diff_steps(&self, a: &Vec<T>, b: &Vec<T>) -> Result<Vec<LevenshteinStep<T>>, PhlDistanceError> {
        let mut table = LevenshteinTable::new(a.len(), b.len());

        let begin_with_none = |items: &Vec<T>| {
            [None].into_iter().chain(items.clone().into_iter().map(Some))
        };
        let expected: Vec<_> = begin_with_none(a).collect();
        let actual: Vec<_>  = begin_with_none(b).collect();
        for (i, a_i) in expected.clone().into_iter().enumerate() {
            for (j, b_j) in actual.clone().into_iter().enumerate() {
                let (i, j) = (i as i64, j as i64);
                if i == 0 && j == 0 { continue }

                let (action, cost) = [Action::DEL, Action::INS, Action::EQ, Action::SUB]
                    .into_iter()
                    .map(|action| match action {
                        Action::DEL => (action, self.cost_del(a_i.as_ref())),
                        Action::INS => (action, self.cost_ins(b_j.as_ref())),
                        Action::SUB => (action, self.cost_sub(a_i.as_ref(), b_j.as_ref())),
                        Action::EQ => (action, self.cost_eq(a_i.as_ref(), b_j.as_ref()))
                    })
                    .min_by_key(|(action, result)| match result {
                        Ok(cost) => *cost + table.prev_cost(*action, (i, j)),
                        Err(_) => Cost::INFINITY
                    })
                    .unwrap();

                let cost = cost?;

                let result = match action {
                    Action::DEL => LevenshteinStep {action, cost, expected: a_i.clone(), actual: None},
                    Action::INS => LevenshteinStep {action, cost, expected: None, actual: b_j.clone()},
                    Action::SUB => LevenshteinStep {action, cost, expected: a_i.clone(), actual: b_j.clone()},
                    Action::EQ => LevenshteinStep {action, cost, expected: a_i.clone(), actual: b_j.clone()},
                };

                let total_cost = table.prev_cost(action, (i, j)) + cost;
                table.insert((i, j), (total_cost, result));
            }
        }
        Ok(table.backtrace())
    }
}

pub(crate) struct DefaultCalculator;
impl<T: Levenshteinable> ComputeCost<T> for DefaultCalculator {
    fn cost_sub(&self, a: Option<&T>, b: Option<&T>) -> Result<Cost, PhlDistanceError> {
        Ok(
            if a.is_none() || b.is_none() { Cost::INFINITY }
            else if a == b { Cost::ZERO }
            else { Cost::ONE }
        )
    }

    fn cost_del(&self, a: Option<&T>) -> Result<Cost, PhlDistanceError> {
        Ok(if a.is_some() { Cost::ONE } else { Cost::INFINITY })
    }

    fn cost_ins(&self, b: Option<&T>) -> Result<Cost, PhlDistanceError> {
        Ok(if b.is_some() { Cost::ONE } else { Cost::INFINITY })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Action {
    EQ,
    DEL,
    INS,
    SUB,
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // let cls = self.class_name();
        let cls = "Action";
        let value = format!("{self:?}");
        write!(f, "{cls}(\"{value}\")")
    }
}

// #[wasm_bindgen]
// #[pyclass]
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct Cost(pub f64);

impl Cost {
    pub const INFINITY: Cost = Cost(f64::INFINITY);
    pub const ZERO: Cost = Cost(0.0);
    pub const ONE: Cost = Cost(1.0);
}

impl Add for Cost {
    type Output = Cost;
    fn add(self, other: Cost) -> Self {
        Self(self.0 + other.0)
    }
}

impl Sum for Cost {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        Cost(iter.map(|c| c.0).sum())
    }
}

impl Eq for Cost { }

impl Ord for Cost {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(
            if self.0.is_nan() && other.0.is_nan() { Ordering::Equal }
            else if self.0.is_nan() { Ordering::Greater }
            else { Ordering::Less }
        )
    }
}

impl Display for Cost {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

pub trait Levenshteinable: Clone + Eq { }
impl<T> Levenshteinable for T where T: Clone + Eq {}

#[derive(Copy, Clone, Debug)]
pub struct LevenshteinStep<T: Levenshteinable> {
    pub action: Action,
    pub expected: Option<T>,
    pub actual: Option<T>,
    pub cost: Cost,
}

impl<T: Levenshteinable> PartialOrd for LevenshteinStep<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl<T: Levenshteinable> PartialEq for LevenshteinStep<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

struct LevenshteinTable<T: Levenshteinable> {
    cost_table: Vec<Vec<Option<Cost>>>,
    step_table: Vec<Vec<Option<LevenshteinStep<T>>>>
}

impl<T: Levenshteinable> LevenshteinTable<T> {
    fn new(a_size: usize, b_size: usize) -> Self {
        let cost_table = vec![vec![None; b_size + 1]; a_size + 1];
        let step_table = vec![vec![None; b_size + 1]; a_size + 1];
        Self { cost_table, step_table }
    }

    fn get_cost(&self, (i, j): (i64, i64)) -> Option<Cost> {
        self.cost_table.get(i as usize)?.get(j as usize)?.clone()
    }

    fn get_step(&self, (i, j): (i64, i64)) -> Option<LevenshteinStep<T>> {
        self.step_table.get(i as usize)?.get(j as usize)?.clone()
    }

    fn prev_idx(&self, action: Action, (i, j): (i64, i64)) -> (i64, i64){
        match action {
            Action::EQ => (i - 1, j - 1),
            Action::SUB => (i - 1, j - 1),
            Action::DEL => (i - 1, j),
            Action::INS => (i, j - 1),
        }
    }

    fn prev_cost(&self, action: Action, (i, j): (i64, i64)) -> Cost {
        self.get_cost(self.prev_idx(action, (i, j))).unwrap_or(Cost::ZERO)
    }

    fn insert(&mut self, (i, j): (i64, i64), (cost, step): (Cost, LevenshteinStep<T>)) where T: Levenshteinable {
        self.cost_table[i as usize][j as usize] = Some(cost);
        self.step_table[i as usize][j as usize] = Some(step);
    }

    // fn get_best_cost(&self) {
    //     let (action, cost) = [Action::DEL, Action::INS, Action::EQ, Action::SUB]
    //         .into_iter()
    //         .map(|action| match action {
    //             Action::DEL => (action, self.cost_del(a_i_ref)),
    //             Action::INS => (action, self.cost_ins(b_j_ref)),
    //             Action::SUB => (action, self.cost_sub(a_i_ref, b_j_ref)),
    //             Action::EQ => (action, self.cost_eq(a_i_ref, b_j_ref))
    //         })
    //         .min_by_key(|(action, step_cost)| Ok(*step_cost? + table.prev_cost(*action, (i, j))))
    //         .unwrap();
    // }

    fn backtrace(&self) -> Vec<LevenshteinStep<T>> {
        let (mut i, mut j) = self.size();
        if (i, j) == (0, 0) {
            return vec![];
        }
        let mut steps = vec![];
        while i > 0 || j > 0 {
            let current = self.get_step((i, j)).unwrap();
            steps.insert(0, current.clone());
            (i, j) = self.prev_idx(current.action, (i, j));
        }
        return steps;
    }

    fn size(&self) -> (i64, i64) {
        (self.step_table.len() as i64 - 1, self.step_table[0].len() as i64 - 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::distance::levenshtein::edit_distance;
    use string_join::Join;
    #[test]
    fn test_backtrace() {
        let test_cases: Vec<(&str, &str, f64, Vec<&str>)> = vec![
            ("", "", 0.0, vec![]),
            ("A", "A", 0.0, vec!["EQ"]),
            ("AA", "AA", 0.0, vec!["EQ", "EQ"]),
            ("A", "B", 1.0, vec!["SUB"]),
            ("AA", "BB", 2.0, vec!["SUB", "SUB"]),
            ("AB", "A", 1.0, vec!["EQ", "DEL"]),
            ("A", "AB", 1.0, vec!["EQ", "INS"]),
            ("ABC", "BB", 2.0, vec!["SUB", "EQ", "DEL"]),
            ("BB", "ABC", 2.0, vec!["SUB", "EQ", "INS"]),
            ("AAA", "AA", 1.0, vec!["EQ", "EQ", "DEL"]),
            ("AA", "AAA", 1.0, vec!["EQ", "EQ", "INS"]),
        ];

        for (a, b, expect_cost, actions) in test_cases {
            // let steps = levenshtein_str(a, b);
            let steps = edit_distance(a.chars().collect(), b.chars().collect()).unwrap();
            let actual_actions = " ".join(steps.iter().map(|x| format!("{:?}", x.action)));
            let expect_actions = " ".join(actions);
            let actual_cost = steps.iter().map(|s| s.cost.0).sum();
            assert_eq!(expect_actions, actual_actions);
            assert_eq!(expect_cost, actual_cost);
        }
    }
}