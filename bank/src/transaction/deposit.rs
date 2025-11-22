use super::{Transaction, TxError};
use crate::balance::{manager::BalanceManager, operations::BalanceOp};
use crate::storage::Storage;

/// Пополнение счета
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

impl Into<BalanceOp> for Deposit {
    fn into(self) -> BalanceOp {
        BalanceOp::Deposit(self.amount)
    }
}
