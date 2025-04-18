use super::RawHandle;

/// Used as a flag. Reading from this handle will always result in a crash.
///
/// Do not initialize!
pub const UNINITIALIZED_HANDLE: RawHandle = i32::MAX;

/// WARNING! With the current VM this still needs to be initialized before use.
pub const BIG_INT_CONST_ZERO: RawHandle = -10;

pub const CALL_VALUE_KLV: RawHandle = -11;
pub const CALL_VALUE_SINGLE_KDA: RawHandle = -13;

pub const BIG_INT_TEMPORARY_1: RawHandle = -14;
pub const BIG_INT_TEMPORARY_2: RawHandle = -15;

/// WARNING! With the current VM this still needs to be initialized before use.
pub const MBUF_CONST_EMPTY: RawHandle = -20;
pub const CALL_VALUE_MULTI_KDA: RawHandle = -21;
pub const CALL_VALUE_SINGLE_KDA_TOKEN_NAME: RawHandle = -22;
pub const CALL_VALUE_MULTI_KDA_NO_KLV: RawHandle = -23;
pub const MBUF_TEMPORARY_1: RawHandle = -25;
pub const MBUF_TEMPORARY_2: RawHandle = -26;

pub const ADDRESS_CALLER: RawHandle = -30;
pub const ADDRESS_SELF: RawHandle = -31;

pub const NEW_HANDLE_START_FROM: RawHandle = -100; // > -100 reserved for APIs

/// Used as a flag. Do not use as a regular handle.
pub const MANAGED_OPTION_NONE: RawHandle = i32::MAX - 1;
