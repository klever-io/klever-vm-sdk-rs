#![no_std]

extern crate alloc;

#[cfg(feature = "klever-sc-codec-derive")]
pub use klever_sc_codec_derive as derive;

/// Reexport needed by derive.
pub use alloc::vec::Vec;

/// Reexported for convenience.
pub use arrayvec;

/// Reexported for convenience.
#[cfg(feature = "num-bigint")]
pub use num_bigint;

// TODO: group into smaller sub-modules

pub mod codec_convert;
mod codec_err;
mod codec_err_handler;
mod default_traits;
mod impl_for_types;
mod multi;
pub mod multi_types;
mod num_conv;
mod single;
pub mod test_util;
mod transmute;
mod try_static_cast;

pub use crate::{
    num_conv::{top_encode_number, universal_decode_number},
    try_static_cast::{
        try_cast_execute_or_else, try_cast_ref, try_execute_then_cast, TryStaticCast,
    },
};
pub use codec_err::{DecodeError, EncodeError};
pub use codec_err_handler::*;
pub use default_traits::{DecodeDefault, EncodeDefault};
pub use impl_for_types::impl_empty::Empty;
pub use multi::*;
pub use single::*;

pub use transmute::{boxed_slice_into_vec, vec_into_boxed_slice};
