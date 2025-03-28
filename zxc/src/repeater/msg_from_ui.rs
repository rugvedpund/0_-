use std::path::PathBuf;

use serde::Deserialize;

use crate::commander::codec::Codec;
use crate::forward_info::{ForwardInfo, Module};
use crate::proxy::server_info::json::ServerInfoJson;

/* Message received from Repeater UI
 *
 * Supported Messages:
 *     [0,{"Encode" : {"Codec": Base64, "data": "aGVsbG8gd29ybGQ="}}]
 *     [0,{"Decode" : {"Codec": Base64, "data": "aGVsbG8gd29ybGQ="}}]
 *     [0,{"Send" : {"file": "1.req", "update": true, "server_info": {
 *      "scheme": "https", "host": "www.google.com", "update": true, "sni": "www.google.com"}]
 *     [0,{"WsEstablish": "1.req"}}]
 *     [0,{"WsSend": "1"}]
 */

#[derive(Debug, Deserialize)]
pub struct RepeaterMsg {
    pub id: usize,
    pub operation: Operation,
}

impl RepeaterMsg {
    pub fn to_repeater(&self) -> bool {
        if let Operation::Forward(finfo) = &self.operation {
            return matches!(finfo.to_module(), &Module::Repeater);
        }
        false
    }
}

#[derive(Debug, Deserialize)]
pub enum Operation {
    Encode {
        codec: Codec,
        data: String,
    },
    Decode {
        codec: Codec,
        data: String,
    },
    // http
    Send(SendInfo),
    // ws
    WsEstablish(SendInfo),
    WsSend(usize),
    WsClose(usize),
    Forward(ForwardInfo),
    Close,
}

// http request send info
#[derive(Debug, Deserialize)]
pub struct SendInfo {
    pub file: PathBuf,
    pub update: Option<bool>,
    pub server_info: ServerInfoJson,
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_msg() {
//        let val = r#"[1,{"Forward":{"file":"./history/1/1.req","to":"Repeater","server_info":{"host":"www.google.com"}}}]"#;
//        let msg: RepeaterMsg = serde_json::from_slice(val.as_bytes()).unwrap();
//    }
//    }
