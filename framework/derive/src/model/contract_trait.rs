use super::{Method, Supertrait, TraitProperties};

#[derive(Clone, Debug)]
pub struct KdaAttribute {
    pub ticker: String,
    pub ty: proc_macro2::TokenStream,
}

/// Models a contract or module trait.
pub struct ContractTrait {
    pub docs: Vec<String>,
    pub original_attributes: Vec<syn::Attribute>,
    pub trait_name: proc_macro2::Ident,
    pub supertraits: Vec<Supertrait>,

    /// It is possible to automatically implement a contract module for all contracts that use it indirectly.
    /// The drawback is that the developer make sure multiple inheritance does not happen.
    /// This feature is currently disabled.
    pub auto_inheritance_modules: Vec<Supertrait>,

    pub methods: Vec<Method>,
    pub trait_attributes: TraitProperties,
}
