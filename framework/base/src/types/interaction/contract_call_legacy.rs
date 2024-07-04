mod contract_call_convert;
mod contract_call_exec;
mod contract_call_no_payment;
mod contract_call_trait;
mod contract_call_with_any_payment;
mod contract_call_with_klv;
mod contract_call_with_klv_or_single_kda;
mod contract_call_with_multi_kda;
mod contract_deploy;
mod typed_function_call;

pub use contract_call_no_payment::ContractCallNoPayment;
pub use contract_call_trait::{ContractCall, ContractCallBase};
pub use contract_call_with_any_payment::ContractCallWithAnyPayment;
pub use contract_call_with_klv::ContractCallWithKlv;
pub use contract_call_with_klv_or_single_kda::ContractCallWithKlvOrSingleKda;
pub use contract_call_with_multi_kda::ContractCallWithMultiKda;
pub use contract_deploy::{new_contract_deploy, ContractDeploy};
pub use typed_function_call::TypedFunctionCall;

/// Using max u64 to represent maximum possible gas,
/// so that the value zero is not reserved and can be specified explicitly.
/// Leaving the gas limit unspecified will replace it with `api.get_gas_left()`.
pub(crate) const UNSPECIFIED_GAS_LIMIT: u64 = u64::MAX;
