use super::{Transaction, TxError};
use crate::Storage;
use std::ops::Add;

#[derive(Debug, Clone)]
pub struct TxCombinator<T1: Transaction, T2: Transaction> {
    t1: T1,
    t2: T2,
}

impl<T1: Transaction, T2: Transaction> TxCombinator<T1, T2> {
    pub fn new(t1: T1, t2: T2) -> Self {
        Self { t1, t2 }
    }
}

impl<T1: Transaction, T2: Transaction> Transaction for TxCombinator<T1, T2> {
    fn apply(&self, storage: &mut Storage) -> Result<(), TxError> {
        self.t1.apply(storage)?;
        self.t2.apply(storage)?;
        Ok(())
    }
}

impl<T1: Transaction, T2: Transaction, Rhs: Transaction> Add<Rhs> for TxCombinator<T1, T2> {
    type Output = TxCombinator<TxCombinator<T1, T2>, Rhs>;

    fn add(self, rhs: Rhs) -> Self::Output {
        TxCombinator::new(self, rhs)
    }
}
