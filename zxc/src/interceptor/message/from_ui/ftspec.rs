use serde::Deserialize;

use crate::proxy::server_info::json::ServerInfoJson;

/* FileType Specific Resume Information
 *
 *      request:
 *          { req: {
 *              need_response: Option<bool>,
 *              update: Option<bool>,
 *              server_info: {
 *                    host: String,
 *                    scheme: String,
 *                    sni: String } } }
 *
 *      response:
 *          { res: { update: bool } }
 *
 *      websocket request:
 *          { wreq: { need_response: Option<bool> } }
 *
 *      websocket response:
 *           wres
 */

#[derive(Debug, Deserialize)]
pub enum FtSpecific {
    #[serde(rename = "req")]
    Req {
        server_info: Option<ServerInfoJson>,
        need_response: Option<bool>,
        update: Option<bool>,
    },
    #[serde(rename = "res")]
    Res {
        update: Option<bool>,
    },
    #[serde(rename = "wreq")]
    WReq {
        need_response: Option<bool>,
    },
    #[serde(rename = "wres")]
    WRes,
}
