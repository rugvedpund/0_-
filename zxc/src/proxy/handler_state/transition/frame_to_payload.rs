/* Trait to convert the frame (http req/res) to log data. Applicable
 *      to http only.
 */

pub trait FrameToPayload {
    fn frame_to_payload(&mut self);
}
