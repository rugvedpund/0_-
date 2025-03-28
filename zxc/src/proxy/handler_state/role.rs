use std::fmt::{Display, Formatter};

use crate::file_types::FileType;

// Enum to represent the role played by the stream
#[derive(Copy, Clone)]
pub enum Role {
    Client,
    Server,
}

impl Display for Role {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Client => write!(f, "client"),
            Role::Server => write!(f, "server"),
        }
    }
}

pub const fn as_arrow(role: &Role) -> &'static str {
    match role {
        Role::Server => "->",
        Role::Client => "<-",
    }
}

pub const fn as_ws_ft(role: &Role) -> FileType {
    match role {
        Role::Server => FileType::Wreq,
        Role::Client => FileType::Wres,
    }
}

pub trait GetRole {
    fn role(&self) -> Role;
}
