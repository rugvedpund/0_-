use super::{WsStruct, *};

mod from_oneone;

// T: Client
// E: Server

type TupleWs<T, E> = (WsStruct<E, T>, WsStruct<T, E>);

// Trait to convert to TupleWs
pub trait ToWs<T, E> {
    async fn convert(self) -> Result<TupleWs<T, E>, WsCreationError>;
}
