use crate::Storage;

#[derive(Debug)]
pub enum TxError {
    InsufficientFunds,
    InvalidAccount,
}

pub trait Transaction {
    fn apply(&self, storage: &mut Storage) -> Result<(), TxError>;
}
