mod tx_env_sc;
mod tx_exec_deploy;
mod tx_exec_sync;
mod tx_exec_te;
mod tx_exec_upgrade;

use unwrap_infallible::UnwrapInfallible;
pub use tx_env_sc::*;
pub use tx_exec_deploy::*;
pub use tx_exec_sync::*;

use crate::{
    api::CallTypeApi,
    io::{ArgErrorHandler, ArgId, ManagedResultArgLoader},
    types::{ManagedBuffer, ManagedVec},
};
use klever_sc_codec::TopDecodeMulti;

/// In case of `transfer_execute`, we leave by default a little gas for the calling transaction to finish.
pub(crate) const TRANSFER_EXECUTE_DEFAULT_LEFTOVER: u64 = 100_000;

pub(crate) fn decode_result<SA, RequestedResult>(
    raw_result: ManagedVec<SA, ManagedBuffer<SA>>,
) -> RequestedResult
where
    SA: CallTypeApi + 'static,
    RequestedResult: TopDecodeMulti,
{
    let mut loader = ManagedResultArgLoader::new(raw_result);
    let arg_id = ArgId::from(&b"sync result"[..]);
    let h: ArgErrorHandler<SA> = ArgErrorHandler::<SA>::from(arg_id);
    RequestedResult::multi_decode_or_handle_err(&mut loader, h).unwrap_infallible()
}
