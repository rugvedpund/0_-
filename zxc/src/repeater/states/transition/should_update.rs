/* Description:
 *      Trait to check if frame should be updated.
 *      Decided by b:update set by user in interceptor and repeater windows.
 *      http only.
 *      ws should always be updated.
 *      So for ws, its blanket implementation, always returns true.
 */

pub trait ShouldUpdate {
    fn should_update(&self) -> bool;
}
