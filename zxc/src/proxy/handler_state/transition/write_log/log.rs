use std::path::PathBuf;

pub trait Log {
    fn path(&self) -> &PathBuf;

    fn log_data(&self) -> &[u8];
}
