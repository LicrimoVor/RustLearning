use std::ops::Add;

use super::transaction::{Transaction, TxError};
use crate::{BalanceManager, BalanceManagerError, Storage, transaction::TxCombinator};

#[derive(Debug, Clone)]
pub struct Withdraw {
    account: String,
    amount: i64,
}

impl Withdraw {
    pub fn new(account: String, amount: i64) -> Self {
        Self { account, amount }
    }
}

impl Transaction for Withdraw {
    fn apply(&self, storage: &mut Storage) -> Result<(), TxError> {
        storage
            .withdraw(&self.account, self.amount)
            .map_err(|e| match e {
                BalanceManagerError::NotEnoughMoney { .. } => TxError::InsufficientFunds,
                BalanceManagerError::UserNotFound(_) => TxError::InvalidAccount,
            })?;
        Ok(())
    }
}

impl<Rhs: Transaction> Add<Rhs> for Withdraw {
    type Output = TxCombinator<Withdraw, Rhs>;

    fn add(self, rhs: Rhs) -> Self::Output {
        TxCombinator::new(self, rhs)
    }
}
