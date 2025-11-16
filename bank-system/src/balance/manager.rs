use crate::Name;

#[derive(Debug)]
pub enum BalanceManagerError {
    UserNotFound(Name),
    NotEnoughMoney { required: i64, available: i64 },
}

pub trait BalanceManager {
    fn deposit(&mut self, name: &Name, amount: i64) -> Result<(), BalanceManagerError>;
    fn withdraw(&mut self, name: &Name, amount: i64) -> Result<(), BalanceManagerError>;
}
