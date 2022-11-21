use crate::{branch, consts};

pub struct Group {
    tran_type: consts::TransactionType,
    branches: [branch::Branch],
}