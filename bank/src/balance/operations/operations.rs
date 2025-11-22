use super::{Status, balance::BalanceOp};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Operation {
    id: u64,
    pub tx_type: BalanceOp,
    timestamp: u64,
    pub status: Status,
    pub description: String,
}

impl Operation {
    pub fn new(id: u64, tx_type: BalanceOp, description: Option<String>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Установите актуальное время")
            .as_secs();

        Self {
            id,
            tx_type,
            timestamp,
            status: Status::PENDING,
            description: description.unwrap_or(format!("Record number #{}", id)),
        }
    }

    pub fn deposit(id: u64, amount: u64) -> Self {
        Self::new(id, BalanceOp::Deposit(amount), None)
    }

    pub fn withdraw(id: u64, amount: u64) -> Self {
        Self::new(id, BalanceOp::Withdraw(amount), None)
    }

    pub fn transfer(id: u64, name: String, amount: i64) -> Self {
        Self::new(id, BalanceOp::Transfer(name, amount), None)
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }
}
