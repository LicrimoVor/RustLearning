pub mod files;
pub mod storage;
use crate::{Name, balance::Balance};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Storage {
    accounts: HashMap<Name, Balance>,
}
