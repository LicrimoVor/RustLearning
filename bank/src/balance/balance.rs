use super::{
    BalanceSize,
    operations::{BalanceOp, BalanceOpError, Operation, Status},
};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Balance {
    value: BalanceSize,
    history: Vec<Operation>,
}

impl Display for Balance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let history = self
            .history
            .iter()
            .map(|op| format!("{:?}", op.tx_type))
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "{},[{}]", self.value, history)
    }
}

impl Default for Balance {
    fn default() -> Self {
        Self::new(0, vec![])
    }
}

impl From<i128> for Balance {
    fn from(value: i128) -> Self {
        Balance::new(value, vec![])
    }
}

impl From<i64> for Balance {
    fn from(value: i64) -> Self {
        Balance::new(value as i128, vec![])
    }
}

impl Balance {
    pub fn new(value: i128, history: Vec<Operation>) -> Self {
        Balance { value, history }
    }

    /// Применяет операцию к счету
    pub fn apply_op(&mut self, op: Operation) -> Result<(), BalanceOpError> {
        let result = match op.tx_type {
            BalanceOp::Deposit(b) => {
                if let Some(sum) = self.value.checked_add(b.into()) {
                    self.value = sum;
                    Ok(())
                } else {
                    Err(BalanceOpError::OverLimitInt64)
                }
            }
            BalanceOp::Withdraw(b) => {
                if self.value < b.into() {
                    Err(BalanceOpError::NotEnoughMoney {
                        required: b,
                        available: self.value,
                    })
                } else {
                    self.value -= b.into();
                    Ok(())
                }
            }
            BalanceOp::Transfer(_, b) => {
                if self.value < b {
                    Err(BalanceOpError::NotEnoughMoney {
                        required: b,
                        available: self.value,
                    })
                } else {
                    self.value = b;
                    Ok(())
                }
            }
            BalanceOp::Close => {
                self.value = 0;
                Ok(())
            }
        };

        if result.is_ok() {
            op.set_status(Status::SUCCESS);
            self.history.push(op);
        } else {
            op.set_status(Status::FAILURE);
        }
        result
    }

    pub fn get_value(&self) -> i64 {
        self.value
    }

    pub fn get_history(&self) -> &Vec<Operation> {
        &self.history
    }
}

impl TryFrom<String> for Balance {
    type Error = BalanceOpError;
    fn try_from(text: String) -> Result<Self, Self::Error> {
        if text.is_empty() {
            return Err(BalanceOpError::ParseError("Пустая строка".to_string()));
        }
        let Some((value, history)) = text.trim().split_once(',') else {
            return Err(BalanceOpError::ParseError("Баланс некорректен".to_string()));
        };

        let value = value
            .parse::<i64>()
            .map_err(|_| BalanceOpError::ParseError("Баланс некорректен".to_string()))?;

        let history_len = history.len();
        let history = history[1..history_len - 1] // убираем скобочки []
            .split(',')
            .map(|op| BalanceOp::try_from(op.to_string()))
            .collect::<Result<Vec<BalanceOp>, BalanceOpError>>()?;
        Ok(Balance { value, history })
    }
}
