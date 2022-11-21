use crate::{consts, group};

struct Client {
    uri: String,
    current_level: consts::Level,
    groups: [group::Group],
    gid:String,
}