use hex_literal::hex;
use klever_sc_codec::{EncodeErrorHandler, TopEncode, TopEncodeOutput};

use crate::abi::TypeAbiFrom;
use crate::{
    api::{CallTypeApi, ManagedTypeApi},
    types::{AnnotatedValue, ManagedAddress, ManagedBuffer, TxScEnv, TxTo, TxToSpecified},
};

/// Address of the system smart contract that manages KDA.
const SYSTEM_SC_ADDRESS_BYTES: [u8; 32] =
    hex!("000000000000000000010000000000000000000000000000000000000002ffff");
const SYSTEM_SC_ADDRESS_BECH32: &str =
    "klv1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u";
const SYSTEM_SC_ADDRESS_ANNOTATION: &str =
    "bech32:klv1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u";

/// Indicates the system SC address, which is the same on any klever blockchain.
pub struct KDASystemSCAddress;

impl KDASystemSCAddress {
    pub fn to_managed_address<Api>(self) -> ManagedAddress<Api>
    where
        Api: ManagedTypeApi,
    {
        ManagedAddress::from(SYSTEM_SC_ADDRESS_BYTES)
    }

    pub fn to_bech32_str(&self) -> &str {
        SYSTEM_SC_ADDRESS_BECH32
    }

    pub fn to_bech32_string(&self) -> alloc::string::String {
        SYSTEM_SC_ADDRESS_BECH32.into()
    }
}

impl<Api> AnnotatedValue<TxScEnv<Api>, ManagedAddress<Api>> for KDASystemSCAddress
where
    Api: CallTypeApi,
{
    fn annotation(&self, _env: &TxScEnv<Api>) -> ManagedBuffer<Api> {
        ManagedBuffer::from(SYSTEM_SC_ADDRESS_ANNOTATION)
    }

    fn to_value(&self, _env: &TxScEnv<Api>) -> ManagedAddress<Api> {
        KDASystemSCAddress.to_managed_address()
    }
}

impl<Api> TxTo<TxScEnv<Api>> for KDASystemSCAddress where Api: CallTypeApi {}
impl<Api> TxToSpecified<TxScEnv<Api>> for KDASystemSCAddress where Api: CallTypeApi {}

impl TopEncode for KDASystemSCAddress {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        SYSTEM_SC_ADDRESS_BYTES.top_encode_or_handle_err(output, h)
    }
}

impl<M> TypeAbiFrom<KDASystemSCAddress> for ManagedAddress<M> where M: ManagedTypeApi {}

impl core::fmt::Display for KDASystemSCAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(SYSTEM_SC_ADDRESS_BECH32)
    }
}
