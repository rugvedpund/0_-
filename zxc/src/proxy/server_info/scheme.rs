use std::fmt::Display;

use serde::{Deserialize, Serialize};

const HTTP: &str = "http";
const HTTPS: &str = "https";

#[derive(Serialize, Debug, Deserialize, Copy, Clone, PartialEq)]
pub enum Scheme {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "https")]
    Https,
}

impl Display for Scheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scheme = match self {
            Scheme::Http => HTTP,
            Scheme::Https => HTTPS,
        };
        write!(f, "{}", scheme)
    }
}

impl Scheme {
    pub const fn default_port(&self) -> u16 {
        match self {
            Scheme::Http => 80,
            Scheme::Https => 443,
        }
    }

    /* Description:
     *      Function to get http as Option<bool>.
     *      Used when sending data to ui.
     *      If None, serializing can be skipped.
     *
     * Used by:
     *      ServerInfoJson
     *      RequestHistory
     */

    pub fn http_as_opt(&self) -> Option<bool> {
        match self {
            Scheme::Http => Some(true),
            Scheme::Https => None,
        }
    }
}

impl TryFrom<&str> for Scheme {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            HTTP => Ok(Scheme::Http),
            HTTPS => Ok(Scheme::Https),
            _ => Err(format!("Invalid scheme| {}", s)),
        }
    }
}
