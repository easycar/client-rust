use crate::{
    branch::Branch,
    consts,
};

pub struct Group {
    tran_type: consts::TransactionType,
    branches: Vec<Branch>,
}

impl Group {
    pub fn new_tcc_group(try_uri: String, confirm_uri: String, cancel_uri: String) -> Self {
        let branches = vec![
            Branch::new(&try_uri, consts::BranchAction::Try),
            Branch::new(&confirm_uri, consts::BranchAction::Confirm),
            Branch::new(&cancel_uri, consts::BranchAction::Cancel),
        ];
        Self {
            tran_type: consts::TransactionType::TCC,
            branches,
        }
    }


    pub fn get_tran_type(self) -> consts::TransactionType {
        self.tran_type
    }

    pub fn set_data(&mut self, data: String) {
        self.set(|b| b.set_data(data.clone()))
    }

    pub fn set_timeout(&mut self, second: u8) {
        self.set(|b| b.set_timeout(second))
    }
    //
    pub fn set_header(&mut self, data: String) {
        self.set(|b| b.set_header(data.clone()))
    }

    pub fn set_level(&mut self, level: consts::Level) {
        self.set(|b| b.set_level(level))
    }

    pub fn set_protocol(&mut self, protocol: consts::Protocol) { self.set(|b| b.set_protocol(protocol)) }

    fn set<F>(&mut self, f: F) where
        F: Fn(&mut Branch) {
        for n in 0..self.branches.len() {
            f(&mut self.branches[n])
        }
    }
}