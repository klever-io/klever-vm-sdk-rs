use crate::model::KdaAttribute;
use super::{trait_prop_names::*, util::*};

pub fn is_only_owner_prop(attr: &syn::Attribute) -> bool {
    is_attribute_with_no_args(attr, PROP_ONLY_OWNER)
}

pub fn is_only_admin_prop(attr: &syn::Attribute) -> bool {
    is_attribute_with_no_args(attr, PROP_ADMIN_OWNER)
}

pub fn get_kda_attribute_prop(attr: &syn::Attribute) -> Option<KdaAttribute> {
    get_attribute_with_one_type_arg(attr, PROP_KDA_ATTRIBUTE)
}
