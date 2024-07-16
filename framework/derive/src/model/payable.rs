/// Contains metdata from the `#[payable(...)]` attribute.
/// Only endpoints and the constructor can be marked payable.
#[derive(Clone, Debug)]
pub enum MethodPayableMetadata {
    NotPayable,
    Klv,
    SingleKdaToken(String),
    AnyToken,
}

impl MethodPayableMetadata {
    pub fn is_payable(&self) -> bool {
        !matches!(self, MethodPayableMetadata::NotPayable)
    }

    pub fn abi_strings(&self) -> Vec<String> {
        match self {
            MethodPayableMetadata::NotPayable => Vec::new(),
            MethodPayableMetadata::Klv => vec!["KLV".to_string()],
            MethodPayableMetadata::SingleKdaToken(s) => vec![s.clone()],
            MethodPayableMetadata::AnyToken => vec!["*".to_string()],
        }
    }
}
