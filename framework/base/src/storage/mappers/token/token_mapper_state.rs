use crate::{
    api::ManagedTypeApi,
    codec::{self},
    types::{ManagedBuffer, TokenIdentifier},
};

#[derive(Default, Clone)]
pub enum TokenMapperState<M: ManagedTypeApi> {
    #[default]
    NotSet,
    Token(TokenIdentifier<M>),
}

impl<M: ManagedTypeApi> TokenMapperState<M> {
    pub fn is_set(&self) -> bool {
        matches!(self, TokenMapperState::Token(_))
    }

    pub fn is_not_set(&self) -> bool {
        matches!(self, TokenMapperState::NotSet)
    }
}

impl<M: ManagedTypeApi> codec::TopEncode for TokenMapperState<M> {
    fn top_encode_or_handle_err<O, H>(
        &self,
        output: O,
        h: H,
    ) -> core::result::Result<(), H::HandledErr>
    where
        O: codec::TopEncodeOutput,
        H: codec::EncodeErrorHandler,
    {
        match self {
            TokenMapperState::NotSet => codec::TopEncode::top_encode_or_handle_err(&"", output, h),
            TokenMapperState::Token(token) => {
                codec::TopEncode::top_encode_or_handle_err(&token, output, h)
            },
        }
    }
}

impl<M: ManagedTypeApi> codec::TopDecode for TokenMapperState<M> {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> core::result::Result<Self, H::HandledErr>
    where
        I: codec::TopDecodeInput,
        H: codec::DecodeErrorHandler,
    {
        let decoded_input = ManagedBuffer::top_decode_or_handle_err(input, h)?;
        if decoded_input.is_empty() {
            Ok(TokenMapperState::NotSet)
        } else {
            let token_id = TokenIdentifier::from_kda_bytes(decoded_input);
            Ok(TokenMapperState::Token(token_id))
        }
    }
}
