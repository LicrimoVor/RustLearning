mod combine;
mod deposit;
mod transfer;
mod withdraw;

pub use combine::TxCombinator;
pub use deposit::Deposit;
pub use transfer::Transfer;
pub use withdraw::Withdraw;
pub mod macros;

use crate::{Storage, impl_add_trait};

#[derive(Debug)]
pub enum TxError {
    InsufficientFunds,
    InvalidAccount,
}

pub trait Transaction {
    fn apply(&self, storage: &mut Storage) -> Result<(), TxError>;
}

impl_add_trait!(Deposit, Withdraw, Transfer);
