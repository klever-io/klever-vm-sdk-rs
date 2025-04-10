use alloc::vec::Vec;
use klever_sc_codec::{
    DecodeError, DecodeErrorHandler, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};

use crate::{
    abi::{
        ExplicitEnumVariantDescription, TypeAbi, TypeAbiFrom, TypeContents, TypeDescription,
        TypeDescriptionContainer, TypeName,
    },
    api::ManagedTypeApi,
    codec::EncodeErrorHandler,
    types::ManagedBuffer,
};

const COMPLETED_STR: &str = "completed";
const INTERRUPTED_STR: &str = "interrupted";

/// Standard way of signalling that an operation was interrupted early, before running out of gas.
/// An endpoint that performs a longer operation can check from time to time if it is running low
/// on gas and can decide to save its state and exit, so that it can continue the same operation later.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum OperationCompletionStatus {
    Completed,
    InterruptedBeforeOutOfGas,
}

impl OperationCompletionStatus {
    pub fn output_bytes(&self) -> &'static [u8] {
        match self {
            OperationCompletionStatus::Completed => COMPLETED_STR.as_bytes(),
            OperationCompletionStatus::InterruptedBeforeOutOfGas => INTERRUPTED_STR.as_bytes(),
        }
    }

    pub fn is_completed(&self) -> bool {
        matches!(self, OperationCompletionStatus::Completed)
    }

    pub fn is_interrupted(&self) -> bool {
        matches!(self, OperationCompletionStatus::InterruptedBeforeOutOfGas)
    }
}

impl TopEncode for OperationCompletionStatus {
    fn top_encode_or_handle_err<O, H>(&self, output: O, _h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        output.set_slice_u8(self.output_bytes());
        Ok(())
    }
}

impl TopDecode for OperationCompletionStatus {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        const BUFFER_LEN: usize = 16;
        let mut buffer = [0u8; BUFFER_LEN];
        let len = input.into_max_size_buffer_align_right(&mut buffer, h)?;
        let bytes = &buffer[BUFFER_LEN - len..];

        if bytes.starts_with(COMPLETED_STR.as_bytes()) {
            Ok(OperationCompletionStatus::Completed)
        } else if bytes.starts_with(INTERRUPTED_STR.as_bytes()) {
            Ok(OperationCompletionStatus::InterruptedBeforeOutOfGas)
        } else {
            Err(h.handle_error(DecodeError::INVALID_VALUE))
        }
    }
}

impl<M: ManagedTypeApi> TypeAbiFrom<OperationCompletionStatus> for ManagedBuffer<M> {}
impl TypeAbiFrom<OperationCompletionStatus> for crate::types::heap::BoxedBytes {}
impl TypeAbiFrom<OperationCompletionStatus> for crate::types::heap::Vec<u8> {}

impl TypeAbiFrom<Self> for OperationCompletionStatus {}

impl TypeAbi for OperationCompletionStatus {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        TypeName::from("OperationCompletionStatus")
    }

    fn type_name_rust() -> TypeName {
        TypeName::from("OperationCompletionStatus")
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        let type_names = Self::type_names();

        accumulator.insert(
            type_names,
            TypeDescription {
                docs: Vec::new(),
                names: Self::type_names(),
                contents: TypeContents::ExplicitEnum([
                    ExplicitEnumVariantDescription::new(
                        &["indicates that operation was completed"],
                        COMPLETED_STR,
                    ),
                    ExplicitEnumVariantDescription::new(
                        &["indicates that operation was interrupted prematurely, due to low gas"],
                        INTERRUPTED_STR,
                    )
                ].to_vec()),
                macro_attributes: Vec::new()
            },
        );
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use klever_sc_codec::test_util::check_top_encode_decode;

    use super::*;

    #[test]
    fn test_operation_completion_status_is() {
        assert!(OperationCompletionStatus::Completed.is_completed());
        assert!(!OperationCompletionStatus::Completed.is_interrupted());
        assert!(!OperationCompletionStatus::InterruptedBeforeOutOfGas.is_completed());
        assert!(OperationCompletionStatus::InterruptedBeforeOutOfGas.is_interrupted());
    }

    #[test]
    fn test_codec_decode_operation_completion() {
        check_top_encode_decode(
            OperationCompletionStatus::Completed,
            COMPLETED_STR.as_bytes(),
        );
        check_top_encode_decode(
            OperationCompletionStatus::InterruptedBeforeOutOfGas,
            INTERRUPTED_STR.as_bytes(),
        );
    }
}
