use bytes::BytesMut;
pub const WS_ADD_RAW_PANIC: &str = "No add raw for ws";

/* Description:
 *      Trait to add bytes to data without any protocol checks.
 *
 *      http frames can/cannot be updated based on b:update value set by user
 *      in ui.
 *
 *      ws frames are always updated. (Blanket implementation)
 */

pub trait AddRaw {
    fn add_raw(&mut self, buf: BytesMut);
}
