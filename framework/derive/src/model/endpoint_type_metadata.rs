/// Used to generate the EndpointType in the ABI.
#[derive(Debug, Clone)]
pub enum EndpointTypeMetadata {
    Init,
    Upgrade,
    Endpoint,
}

impl EndpointTypeMetadata {
    pub fn to_tokens(&self) -> proc_macro2::TokenStream {
        match self {
            EndpointTypeMetadata::Init => {
                quote! { klever_sc::abi::EndpointTypeAbi::Init }
            },
            EndpointTypeMetadata::Upgrade => {
                quote! { klever_sc::abi::EndpointTypeAbi::Upgrade }
            },
            EndpointTypeMetadata::Endpoint => {
                quote! { klever_sc::abi::EndpointTypeAbi::Endpoint }
            },
        }
    }
}
