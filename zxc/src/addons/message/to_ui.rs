use std::path::PathBuf;

use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct AddonMsg {
    cmd: String,
    file: PathBuf,
}

impl AddonMsg {
    pub fn new(cmd: String, file: PathBuf) -> AddonMsg {
        AddonMsg {
            cmd,
            file,
        }
    }
}
