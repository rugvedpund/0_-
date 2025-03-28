use super::Roneone;
use crate::repeater::states::transition::should_update::ShouldUpdate;

// Based on b:update variable set by user in UI
impl<T> ShouldUpdate for Roneone<T> {
    #[inline(always)]
    fn should_update(&self) -> bool {
        self.update
    }
}
