use super::Balance;
use std::{
    fmt::Display,
    ops::{AddAssign, SubAssign},
};

#[derive(Debug, Clone, Copy)]
pub enum OpBalance {
    Deposit(i64),
    Withdraw(i64),
    Close,
}

impl Display for OpBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            OpBalance::Deposit(v) => format!("D{}", v),
            OpBalance::Withdraw(v) => format!("W{}", v),
            OpBalance::Close => "C".to_string(),
        };
        write!(f, "{label}")
    }
}

impl From<String> for OpBalance {
    fn from(value: String) -> Self {
        match value.as_str() {
            "D" => OpBalance::Deposit(0),
            "W" => OpBalance::Withdraw(0),
            "C" => OpBalance::Close,
            _ => OpBalance::Deposit(0),
        }
    }
}

impl AddAssign<i64> for Balance {
    fn add_assign(&mut self, rhs: i64) {
        self.apply_op(&OpBalance::Deposit(rhs));
    }
}

impl SubAssign<i64> for Balance {
    fn sub_assign(&mut self, rhs: i64) {
        self.apply_op(&OpBalance::Withdraw(rhs));
    }
}

impl From<i64> for Balance {
    fn from(value: i64) -> Self {
        Balance {
            value,
            history: vec![],
        }
    }
}

impl Balance {
    pub fn apply_op(&mut self, op: &OpBalance) -> bool {
        let ok = match op {
            OpBalance::Deposit(b) => self.value.checked_add(*b).is_some(),
            OpBalance::Withdraw(b) => {
                if self.value < *b {
                    return false;
                }
                self.value -= b;
                true
            }
            OpBalance::Close => {
                self.value = 0;
                true
            }
            _ => false,
        };

        if ok {
            self.history.push(*op);
        }
        ok
    }
}
