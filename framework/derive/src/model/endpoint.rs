use super::{EndpointMutabilityMetadata, MethodPayableMetadata};

#[derive(Clone, Debug)]
pub struct InitMetadata {
    pub payable: MethodPayableMetadata,
    pub allow_multiple_var_args: bool,
}

#[derive(Clone, Debug)]
pub struct EndpointMetadata {
    pub public_name: syn::Ident,
    pub payable: MethodPayableMetadata,
    pub only_owner: bool,
    pub only_admin: bool,
    pub only_user_account: bool,
    pub mutability: EndpointMutabilityMetadata,
    pub allow_multiple_var_args: bool,
}

/// Method visibility from the point of view of the smart contract
#[derive(Clone, Debug)]
pub enum PublicRole {
    /// The smart contract constructor. There can be only one.
    Init(InitMetadata),

    /// The smart contract upgrade constructor.
    Upgrade(InitMetadata),

    /// Means it gets a smart contract function generated for it
    Endpoint(EndpointMetadata),

    /// Can only called from within the smart contract.
    Private,
}
