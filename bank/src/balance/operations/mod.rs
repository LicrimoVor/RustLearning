mod balance;
mod operations;
use std::fmt::Display;

pub use balance::{BalanceOp, BalanceOpError};
pub use operations::Operation;

#[derive(Debug, Clone)]
pub enum Status {
    FAILURE,
    PENDING,
    SUCCESS,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Status::FAILURE => "failure",
            Status::PENDING => "pending",
            Status::SUCCESS => "success",
        };
        write!(f, "{}", name)
    }
}
