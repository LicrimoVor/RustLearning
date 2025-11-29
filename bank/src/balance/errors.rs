use super::operations::OperationError;

#[derive(Debug)]
pub enum BalanceError {
    InvalidParseOperation(OperationError),
    InvalidParseBalance(String),
}
