use serde::Deserialize;

// Only Close operation supported
#[derive(Deserialize)]
pub struct AddonMsg {
    _id: u8,
    _operation: String,
}
