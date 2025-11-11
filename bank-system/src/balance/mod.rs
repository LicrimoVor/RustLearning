pub mod balance;
pub mod operations;

pub use operations::OpBalance;

#[derive(Debug, Clone)]
pub struct Balance {
    value: i64,
    pub history: Vec<OpBalance>,
}

impl Balance {
    pub fn proccess<'a>(&mut self, ops: &[&'a OpBalance]) -> Vec<&'a OpBalance> {
        let exclusion = ops
            .into_iter()
            .filter(|op: &&&'a OpBalance| !self.apply_op(*op))
            .map(|op| *op)
            .collect();
        exclusion
    }

    pub fn get_value(&self) -> i64 {
        self.value
    }
}
