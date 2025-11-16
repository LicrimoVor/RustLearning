pub mod analitics;
mod balance;
pub mod manager;
pub mod operations;

pub use manager::{BalanceManager, BalanceManagerError};
pub use operations::{BalanceOp, BalanceOpError};

#[derive(Debug, Clone, PartialEq)]
pub struct Balance {
    value: i64,
    history: Vec<BalanceOp>,
}
