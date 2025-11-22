use super::{Transaction, TxError};
use crate::balance::{manager::BalanceManager, operations::BalanceOp};
use crate::storage::Storage;

#[derive(Debug, Clone)]
pub struct Transfer {
    from: String,
    to: String,
    amount: i64,
}

/// Перевод средств между счетами
impl Transfer {
    pub fn new(from: String, to: String, amount: i64) -> Self {
        Self { from, to, amount }
    }
}

impl Transaction for Transfer {
    fn apply(&self, storage: &mut Storage) -> Result<(), TxError> {
        let Some(from_balance) = storage.get_balance(&self.from) else {
            return Err(TxError::InvalidAccount);
        };

        if from_balance.get_value() < self.amount {
            return Err(TxError::InsufficientFunds);
        };

        storage
            .withdraw(&self.from, self.amount)
            .map_err(|_| TxError::InvalidAccount)?;
        storage
            .deposit(&self.to, self.amount)
            .map_err(|_| TxError::InvalidAccount)?;

        Ok(())
    }
}

impl Into<BalanceOp> for Transfer {
    fn into(self) -> BalanceOp {
        BalanceOp::Withdraw(self.amount)
    }
}
