use crate::consts;

pub struct Branch {
    uri: String,
    data: String,
    header: String,
    action: consts::BranchAction,
    level: consts::Level,
    timeout: u8,
    protocol: consts::Protocol,
}