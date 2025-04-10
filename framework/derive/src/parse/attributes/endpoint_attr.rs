use super::{attr_names::*, util::*};

pub fn is_init(attr: &syn::Attribute) -> bool {
    is_attribute_with_no_args(attr, ATTR_INIT)
}

pub fn is_only_owner(attr: &syn::Attribute) -> bool {
    is_attribute_with_no_args(attr, ATTR_ONLY_OWNER)
}

pub fn is_only_admin(attr: &syn::Attribute) -> bool {
    is_attribute_with_no_args(attr, ATTR_ONLY_ADMIN)
}

pub fn is_only_user_account(attr: &syn::Attribute) -> bool {
    is_attribute_with_no_args(attr, ATTR_ONLY_USER_ACCOUNT)
}

pub fn is_proxy(attr: &syn::Attribute) -> bool {
    is_attribute_with_no_args(attr, ATTR_PROXY)
}

pub fn is_upgrade(attr: &syn::Attribute) -> bool {
    is_attribute_with_no_args(attr, ATTR_UPGRADE)
}

pub fn is_allow_multiple_var_args(attr: &syn::Attribute) -> bool {
    is_attribute_with_no_args(attr, ATTR_ALLOW_MULTIPLE_VAR_ARGS)
}

#[derive(Clone, Debug)]
pub struct EndpointAttribute {
    pub endpoint_name: Option<syn::Ident>,
}

impl EndpointAttribute {
    pub fn parse(attr: &syn::Attribute) -> Option<EndpointAttribute> {
        match is_attr_with_one_opt_token_tree_arg(attr, ATTR_ENDPOINT) {
            None => None,
            Some(Some(proc_macro2::TokenTree::Ident(ident))) => Some(EndpointAttribute {
                endpoint_name: Some(ident),
            }),
            Some(None) => Some(EndpointAttribute {
                endpoint_name: None,
            }),
            _ => panic!("unexpected endpoint argument tokens"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ViewAttribute {
    pub view_name: Option<syn::Ident>,
}

impl ViewAttribute {
    pub fn parse(attr: &syn::Attribute) -> Option<ViewAttribute> {
        match is_attr_with_one_opt_token_tree_arg(attr, ATTR_VIEW) {
            None => None,
            Some(Some(proc_macro2::TokenTree::Ident(ident))) => Some(ViewAttribute {
                view_name: Some(ident),
            }),
            Some(None) => Some(ViewAttribute { view_name: None }),
            _ => panic!("unexpected view argument tokens"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExternalViewAttribute {
    pub view_name: Option<syn::Ident>,
}

impl ExternalViewAttribute {
    pub fn parse(attr: &syn::Attribute) -> Option<ExternalViewAttribute> {
        match is_attr_with_one_opt_token_tree_arg(attr, ATTR_EXTERNAL_VIEW) {
            None => None,
            Some(Some(proc_macro2::TokenTree::Ident(ident))) => Some(ExternalViewAttribute {
                view_name: Some(ident),
            }),
            Some(None) => Some(ExternalViewAttribute { view_name: None }),
            _ => panic!("unexpected external view argument tokens"),
        }
    }
}
