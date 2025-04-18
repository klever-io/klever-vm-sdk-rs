use alloc::format;

use crate::{
    abi::TypeAbiFrom,
    codec::{EncodeErrorHandler, TopEncodeMulti, TopEncodeMultiOutput},
};

use crate::{
    abi::{OutputAbis, TypeAbi, TypeDescriptionContainer, TypeName},
    api::EndpointFinishApi,
};

use super::{SCError, StaticSCError};

/// Default way to optionally return an error from a smart contract endpoint.
#[deprecated(
    since = "0.43.4",
    note = "Use in-place error handling instead, such as `require!` or `sc_panic!`"
)]
#[must_use]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SCResult<T, E = StaticSCError> {
    Ok(T),
    Err(E),
}

impl<T, E> SCResult<T, E> {
    pub fn is_ok(&self) -> bool {
        matches!(self, SCResult::Ok(_))
    }

    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    #[inline]
    pub fn ok(self) -> Option<T> {
        if let SCResult::Ok(t) = self {
            Some(t)
        } else {
            None
        }
    }

    #[inline]
    pub fn err(self) -> Option<E> {
        if let SCResult::Err(e) = self {
            Some(e)
        } else {
            None
        }
    }

    #[inline]
    /// Returns the contained Ok value or signals the error and exits.
    pub fn unwrap_or_signal_error<FA: EndpointFinishApi>(self) -> T
    where
        E: SCError,
    {
        match self {
            SCResult::Ok(t) => t,
            SCResult::Err(e) => e.finish_err::<FA>(),
        }
    }

    /// Used to convert from a regular Rust result.
    /// Any error type is accepted as long as it can be converted to a SCError
    /// (`Vec<u8>`, `&[u8]`, `BoxedBytes`, `String`, `&str` are covered).
    pub fn from_result<FromErr>(r: core::result::Result<T, FromErr>) -> Self
    where
        FromErr: Into<E>,
    {
        match r {
            Ok(t) => SCResult::Ok(t),
            Err(e) => SCResult::Err(e.into()),
        }
    }
}

impl<T, E> TopEncodeMulti for SCResult<T, E>
where
    T: TopEncodeMulti,
    E: TopEncodeMulti,
{
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        match self {
            SCResult::Ok(t) => t.multi_encode_or_handle_err(output, h),
            SCResult::Err(e) => e.multi_encode_or_handle_err(output, h),
        }
    }
}

impl<T: TypeAbi, E> TypeAbiFrom<Self> for SCResult<T, E> {}

impl<T: TypeAbi, E> TypeAbi for SCResult<T, E> {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        T::type_name()
    }

    fn type_name_rust() -> TypeName {
        format!(
            "SCResult<{}, {}>",
            T::type_name_rust(),
            core::any::type_name::<E>()
        )
    }

    /// Gives `SCResult<()>` the possibility to produce 0 output ABIs,
    /// just like `()`.
    /// It is also possible to have `SCResult<MultiResultX<...>>`,
    /// so this gives the MultiResult to dissolve into its multiple output ABIs.
    fn output_abis(output_names: &[&'static str]) -> OutputAbis {
        T::output_abis(output_names)
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }
}

impl<T> SCResult<T> {
    pub fn unwrap(self) -> T {
        match self {
            SCResult::Ok(t) => t,
            SCResult::Err(_) => panic!("called `SCResult::unwrap()`"),
        }
    }
}

impl<T> From<SCResult<T>> for Result<T, StaticSCError> {
    fn from(result: SCResult<T>) -> Self {
        match result {
            SCResult::Ok(ok) => Result::Ok(ok),
            SCResult::Err(error) => Result::Err(error),
        }
    }
}

impl<T, Err> From<Result<T, Err>> for SCResult<T>
where
    Err: Into<StaticSCError>,
{
    fn from(result: Result<T, Err>) -> Self {
        match result {
            Result::Ok(ok) => SCResult::Ok(ok),
            Result::Err(err) => SCResult::Err(err.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::DecodeError;

    use super::*;

    #[test]
    fn test_result_to_sc_result() {
        let result_ok: Result<i32, DecodeError> = Result::Ok(5);
        let sc_result_ok: SCResult<i32> = result_ok.into();

        assert!(sc_result_ok.unwrap() == 5);

        let result_err: Result<i32, DecodeError> = Result::Err(DecodeError::from("Decode Error"));
        let sc_result_err: SCResult<i32> = result_err.into();

        assert!(sc_result_err.err().unwrap().as_bytes() == &b"Decode Error"[..]);
    }
}
