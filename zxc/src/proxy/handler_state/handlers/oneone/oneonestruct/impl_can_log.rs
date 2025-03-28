use super::*;
use crate::proxy::handler_state::CanLog;

/* Steps:
 *      For request, path is none.
 *
 *      For response, if request was logged then same path with res
 *      extension is used.
 */

impl<T, E, U> CanLog for OneOneStruct<T, E, U>
where
    U: InfoLine,
{
    #[inline(always)]
    fn can_log(&self) -> bool {
        self.path.is_some()
    }
}
