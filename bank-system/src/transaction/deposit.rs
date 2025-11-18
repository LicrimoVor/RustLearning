use std::ops::Add;

use super::transaction::{Transaction, TxError};
use crate::{BalanceManager, Storage, transaction::TxCombinator};

#[derive(Debug, Clone)]
pub struct Deposit {
    account: String,
    amount: i64,
}

impl Deposit {
    pub fn new(account: String, amount: i64) -> Self {
        Self { account, amount }
    }
}

impl Transaction for Deposit {
    fn apply(&self, storage: &mut Storage) -> Result<(), TxError> {
        storage
            .deposit(&self.account, self.amount)
            .map_err(|_| TxError::InvalidAccount)?;
        Ok(())
    }
}

impl<Rhs: Transaction> Add<Rhs> for Deposit {
    type Output = TxCombinator<Deposit, Rhs>;

    fn add(self, rhs: Rhs) -> Self::Output {
        TxCombinator::new(self, rhs)
    }
}
