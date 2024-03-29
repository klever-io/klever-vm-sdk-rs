use crate::codec::{TopDecodeMulti, TopEncodeMulti};

use crate::{
    api::CallTypeApi, contract_base::ExitCodecErrorHandler, err_msg, types::ManagedBuffer,
};

use super::{ContractCallNoPayment, ContractCallWithKlv, ManagedArgBuffer};

/// Defines a contract call object, which is the basis for all calls to other contracts.
///
/// Its implementations differ on the type of payment that gets sent with the call.
pub trait ContractCall<SA>: Sized
where
    SA: CallTypeApi + 'static,
{
    type OriginalResult: TopEncodeMulti;

    /// Converts any KDA transfers into builtin function calls,
    /// thus reducing it to a simple transaction with optional KLV value.
    #[doc(hidden)]
    fn into_normalized(self) -> ContractCallWithKlv<SA, Self::OriginalResult>;

    /// Mutable access to the common base.
    #[doc(hidden)]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<SA, Self::OriginalResult>;

    /// Used by the generated proxies to add arguments to a call.
    #[doc(hidden)]
    fn proxy_arg<T: TopEncodeMulti>(&mut self, endpoint_arg: &T) {
        let h = ExitCodecErrorHandler::<SA>::from(err_msg::CONTRACT_CALL_ENCODE_ERROR);
        let Ok(()) =
            endpoint_arg.multi_encode_or_handle_err(&mut self.get_mut_basic().arg_buffer, h);
    }

    /// Serializes and pushes a value to the arguments buffer.
    ///
    /// Accepts multi-values, so it might effectively be adding more than one raw argument.
    ///
    /// Warning: this method serializes any serializable type,
    /// but does no type checking against the destination endpoint ABI.
    /// Only use for raw calls, built without a proxy.
    fn argument<T: TopEncodeMulti>(mut self, arg: &T) -> Self {
        self.proxy_arg(arg);
        self
    }

    /// For cases where we build the contract call by hand.
    ///
    /// No serialization occurs, just direct conversion to ManagedBuffer.
    fn push_raw_argument<RawArg: Into<ManagedBuffer<SA>>>(&mut self, raw_arg: RawArg) {
        self.get_mut_basic().arg_buffer.push_arg_raw(raw_arg.into())
    }

    /// For cases where we build the contract call by hand.
    fn with_raw_arguments(mut self, raw_argument_buffer: ManagedArgBuffer<SA>) -> Self {
        self.get_mut_basic().arg_buffer = raw_argument_buffer;
        self
    }

    /// Sets an explicit gas limit to the call.
    #[inline]
    fn with_gas_limit(mut self, gas_limit: u64) -> Self {
        self.get_mut_basic().explicit_gas_limit = gas_limit;
        self
    }

    fn into_call_data_string(self) -> ManagedBuffer<SA> {
        self.into_normalized().to_call_data_string()
    }

    /// Executes immediately, synchronously, and returns contract call result.
    /// Only works if the target contract is in the same shard.
    #[inline]
    fn execute_on_dest_context<RequestedResult>(self) -> RequestedResult
    where
        RequestedResult: TopDecodeMulti,
    {
        self.into_normalized().execute_on_dest_context()
    }

    /// Executes immediately, synchronously, and returns contract call result.
    ///
    /// Performs a readonly call.    
    #[inline]
    fn execute_on_dest_context_readonly<RequestedResult>(self) -> RequestedResult
    where
        RequestedResult: TopDecodeMulti,
    {
        self.into_normalized().execute_on_dest_context_readonly()
    }

    /// Executes immediately, synchronously, and returns contract call result.
    ///
    /// Performs call on the same context, i.e. the target contract will operate with the data from this contract.    
    #[inline]
    fn execute_on_same_context<RequestedResult>(self) -> RequestedResult
    where
        RequestedResult: TopDecodeMulti,
    {
        self.into_normalized().execute_on_same_context()
    }

    /// Immediately launches a transfer-execute call.
    ///
    /// can be more than one such call per transaction.
    fn transfer_execute(self);
}
