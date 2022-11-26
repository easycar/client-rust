#[derive(Copy, Clone)]
pub enum Protocol {
    HTTP,
    GRPC,
}

pub enum BranchAction {
    Try,
    Confirm,
    Cancel,
    Normal,
    Compensation,
}

pub enum TransactionType {
    Unknown,
    TCC,
    SAGA,
}

pub type Level = u8;