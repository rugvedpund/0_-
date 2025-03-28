use super::RWebSocket;
use crate::repeater::states::transition::should_update::ShouldUpdate;

/* New ws frame should always be constructed.
 * So, always return true
 */

impl<T> ShouldUpdate for RWebSocket<T> {
    #[inline(always)]
    fn should_update(&self) -> bool {
        true
    }
}
