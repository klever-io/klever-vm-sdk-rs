use crate::{
    generate::util::{generate_call_method_name, generate_endpoints_mod_alias},
    model::{ContractTrait, Method, PublicRole},
};

pub fn generate_endpoints_mod(
    contract_trait: &ContractTrait,
    _is_contract_main: bool,
) -> proc_macro2::TokenStream {
    let endpoint_aliases_decl: Vec<proc_macro2::TokenStream> = contract_trait
        .supertraits
        .iter()
        .enumerate()
        .map(|(index, supertrait)| {
            let module_path = &supertrait.module_path;
            let endpoints_alias = generate_endpoints_mod_alias(index);
            quote! {
                pub use #module_path endpoints as #endpoints_alias;
            }
        })
        .collect();

    let mut endpoint_aliases_use: Vec<proc_macro2::TokenStream> = Vec::new();
    for index in 0..contract_trait.supertraits.len() {
        let endpoints_alias = generate_endpoints_mod_alias(index);
        endpoint_aliases_use.push(quote! {
            pub use super::#endpoints_alias::*;
        })
    }

    let endpoints = generate_wasm_endpoints(contract_trait);

    quote! {
        #(#endpoint_aliases_decl)*

        #[allow(non_snake_case)]
        pub mod endpoints {
            use super::EndpointWrappers;

            #(#endpoint_aliases_use)*

            #(#endpoints)*

            // #wasm_callback_fn
            // quote! { }
        }
    }
}

fn generate_wasm_endpoints(contract_trait: &ContractTrait) -> Vec<proc_macro2::TokenStream> {
    contract_trait
        .methods
        .iter()
        .filter_map(|m| match &m.public_role {
            PublicRole::Init(_) => Some(generate_wasm_endpoint(m, &quote! { init })),
            PublicRole::Upgrade(_) => Some(generate_wasm_endpoint(m, &quote! { upgrade })),
            PublicRole::Endpoint(endpoint_metadata) => {
                let endpoint_ident = &endpoint_metadata.public_name;
                Some(generate_wasm_endpoint(m, &quote! { #endpoint_ident }))
            },
            _ => None,
        })
        .collect()
}

fn generate_wasm_endpoint(
    m: &Method,
    _endpoint_ident: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let fn_ident = &m.name;
    let call_method_ident = generate_call_method_name(fn_ident);
    quote! {
        pub fn #fn_ident<A>()
        where
            A: klever_sc::api::VMApi,
        {
            super::EndpointWrappers::#call_method_ident(
                &klever_sc::contract_base::UniversalContractObj::<A>::new(),
            );
        }
    }
}
