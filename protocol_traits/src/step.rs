use buffer::Event;

use crate::frame::Frame;

/* Description:
 *      Trait to read a frame.
 *
 * Methods:
 *      next()
 *          Description : Process the event generated by IO to proceed with
 *                        the next step.
 *          Args        : Event
 *          Returns     : Self
 *          Errors      : StateError
 *
 *      is_ended()
 *          Description : Check if a frame has been read.
 *          Returns     : bool
 *
 *      get_frame()
 *          Description : Consumes the state machine and returns the frame.
 *          Returns     : T
 *          Errors      : FrameError
 *
 *  Implementation:
 *          OneOneState
 */

pub trait Step<T>
where
    T: Frame,
{
    type StateError;
    type FrameError;

    fn next(self, event: Event) -> Result<Self, Self::StateError>
    where
        Self: Sized;

    fn is_ended(&self) -> bool;

    fn into_frame(self) -> Result<T, Self::FrameError>;
}
