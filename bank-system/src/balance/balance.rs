use super::{
    Balance,
    operations::{BalanceOp, BalanceOpError},
};
use std::fmt::Display;

impl Display for Balance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Balance:{}", self.value)
    }
}

impl Balance {
    pub fn new(value: i64) -> Self {
        Balance {
            value,
            history: vec![],
        }
    }

    pub fn apply_op(&mut self, op: &BalanceOp) -> Result<(), BalanceOpError> {
        let result = match op {
            BalanceOp::Deposit(b) => {
                if let Some(sum) = self.value.checked_add(*b) {
                    self.value = sum;
                    Ok(())
                } else {
                    Err(BalanceOpError::OverLimitInt64)
                }
            }
            BalanceOp::Withdraw(b) => {
                if self.value < *b {
                    Err(BalanceOpError::NotEnoughMoney {
                        required: *b,
                        available: self.value,
                    })
                } else {
                    self.value -= b;
                    Ok(())
                }
            }
            BalanceOp::Close => {
                self.value = 0;
                Ok(())
            }
            _ => Err(BalanceOpError::InvalidOperation(
                "Неверная операция".to_string(),
            )),
        };

        if result.is_ok() {
            self.history.push(*op);
        }
        result
    }

    pub fn proccess<'a>(&mut self, ops: &[&'a BalanceOp]) -> Vec<&'a BalanceOp> {
        let exclusion = ops
            .into_iter()
            .filter(|op: &&&'a BalanceOp| self.apply_op(*op).is_ok())
            .map(|op| *op)
            .collect();
        exclusion
    }

    pub fn get_value(&self) -> i64 {
        self.value
    }

    pub fn get_history(&self) -> &Vec<BalanceOp> {
        &self.history
    }
}
