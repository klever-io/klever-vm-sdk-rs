use super::{attributes::PayableAttribute, MethodAttributesPass1};
use crate::model::MethodPayableMetadata;

pub fn process_payable_attribute(
    attr: &syn::Attribute,
    pass_1_data: &mut MethodAttributesPass1,
) -> bool {
    PayableAttribute::parse(attr).map(|payable_attr| {
        if let Some(identifier) = payable_attr.identifier {
            pass_1_data.payable = parse_payable_identifier(identifier.as_str());
        } else {
            panic!(
                "Endpoint `payable` attribute requires one argument. Replace with `#[payable(\"*\")]` or `#[payable(\"KLV\")]`. Method name: {}",
                &pass_1_data.method_name);
        }
    }).is_some()
}

fn parse_payable_identifier(identifier: &str) -> MethodPayableMetadata {
    match identifier {
        "KLV" => MethodPayableMetadata::Klv,
        "*" => MethodPayableMetadata::AnyToken,
        "" => panic!("empty token name not allowed in #[payable] attribute"),
        _ => MethodPayableMetadata::SingleKdaToken(identifier.to_string()),
    }
}
