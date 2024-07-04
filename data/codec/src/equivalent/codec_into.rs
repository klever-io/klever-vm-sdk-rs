use crate::{CodecFrom, TopDecodeMulti, TopEncodeMulti};

/// Signals that we can safely serialize `Self` in order to obtain a `T` on the other size.
#[deprecated(since = "0.43.3", note = "Please use trait `TypeAbiFrom` instead.")]
pub trait CodecInto<T>: TopEncodeMulti
where
    T: TopDecodeMulti,
{
}

impl<F, I> CodecInto<F> for I
where
    I: TopEncodeMulti,
    F: CodecFrom<I>,
{
}
