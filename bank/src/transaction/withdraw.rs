use super::{Transaction, TxError};
use crate::{BalanceManager, BalanceManagerError, BalanceOp, Storage};

#[derive(Debug, Clone)]
pub struct Withdraw {
    account: String,
    amount: i64,
}

/// Списание с счета
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

impl Into<BalanceOp> for Withdraw {
    fn into(self) -> BalanceOp {
        BalanceOp::Withdraw(self.amount)
    }
}
