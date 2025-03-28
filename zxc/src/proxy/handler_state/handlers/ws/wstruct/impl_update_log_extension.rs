use super::WsStruct;
use crate::file_types::{EXT_WREQ, EXT_WRES};
use crate::proxy::handler_state::role::Role;
use crate::proxy::handler_state::transition::write_log::update_log_extension::UpdateLogExt;

pub const fn ws_ext(role: &Role) -> &'static str {
    match role {
        Role::Server => EXT_WREQ,
        Role::Client => EXT_WRES,
    }
}

/* Steps:
 *      1. Push the id to the path.
 *      2. Set the extension based on role.
 *          a. Client: wres
 *          b. Server: wreq
 */

impl<T, E> UpdateLogExt for WsStruct<T, E> {
    fn update_extension(&mut self) {
        self.path
            .set_file_name(self.log_id.to_string());
        let ext = ws_ext(&self.role);
        self.path.set_extension(ext);
    }
}
