use crate::consts;

pub(crate) struct Branch {
    pub(crate) uri: String,
    pub(crate) data: String,
    pub(crate) header: String,
    pub(crate) action: consts::BranchAction,
    pub(crate) level: consts::Level,
    pub(crate) timeout: u8,
    pub(crate) protocol: consts::Protocol,
}


impl Branch {
    pub fn new(uri: &String, action: consts::BranchAction) -> Self {
        Self {
            uri: uri.to_string(),
            data: "".to_string(),
            header: "".to_string(),
            action,
            level: 1,
            timeout: 5,
            protocol: get_protocol(uri),
        }
    }

    pub(crate) fn set_level(&mut self, level: consts::Level) {
        self.level = level
    }

    pub(crate) fn set_protocol(&mut self, protocol: consts::Protocol) {
        self.protocol = protocol
    }

    pub(crate) fn set_data(&mut self, data: String) {
        self.data = data;
    }
    pub(crate) fn set_header(&mut self, header: String) {
        self.header = header
    }
    pub(crate) fn set_timeout(&mut self, timeout: u8) {
        self.timeout = timeout
    }
}


fn get_protocol(uri: &String) -> consts::Protocol {
    if uri.starts_with("http://") || uri.starts_with("https://") {
        return consts::Protocol::HTTP;
    }
    if uri.starts_with("grpc://") {
        return consts::Protocol::GRPC;
    }

    consts::Protocol::GRPC
}