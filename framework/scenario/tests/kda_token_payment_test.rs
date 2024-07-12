use klever_sc::{
    codec::{self},
    types::{KdaTokenPayment},
};
use klever_sc_scenario::api::StaticApi;

/// Helper top-decode.
fn kda_token_payment_regular_top_decode_or_handle_err<I, H>(
    top_input: I,
    h: H,
) -> Result<KdaTokenPayment<StaticApi>, H::HandledErr>
where
    I: codec::TopDecodeInput,
    H: codec::DecodeErrorHandler,
{
    let mut nested_buffer = top_input.into_nested_buffer();
    let result = KdaTokenPayment::regular_dep_decode_or_handle_err(&mut nested_buffer, h)?;
    if !codec::NestedDecodeInput::is_depleted(&nested_buffer) {
        return Err(h.handle_error(codec::DecodeError::INPUT_TOO_LONG));
    }
    Ok(result)
}
