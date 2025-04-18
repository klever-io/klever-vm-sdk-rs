use alloc::string::ToString;
use alloc::{string::String, vec::Vec};

use super::TypeNames;

#[derive(Clone, Debug)]
pub struct TypeDescription {
    pub docs: Vec<String>,
    pub names: TypeNames,
    pub contents: TypeContents,
    pub macro_attributes: Vec<String>,
}

impl TypeDescription {
    /// Used as temporary value.
    /// To avoid an infinite loop for recursive types,
    /// we must reserve the type key (type name) before computing its fields.
    /// We use this as value while the fields are being computed.
    pub const PLACEHOLDER: TypeDescription = TypeDescription {
        docs: Vec::new(),
        names: TypeNames {
            abi: String::new(),
            rust: String::new(),
        },
        contents: TypeContents::NotSpecified,
        macro_attributes: Vec::new(),
    };
}

impl TypeDescription {
    /// Used in code generation.
    pub fn new(
        docs: &[&str],
        names: TypeNames,
        contents: TypeContents,
        macro_attributes: &[&str],
    ) -> Self {
        TypeDescription {
            docs: docs.iter().map(|s| s.to_string()).collect(),
            names,
            contents,
            macro_attributes: macro_attributes.iter().map(|s| s.to_string()).collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum TypeContents {
    NotSpecified,
    Enum(Vec<EnumVariantDescription>),
    Struct(Vec<StructFieldDescription>),
    ExplicitEnum(Vec<ExplicitEnumVariantDescription>),
}

impl TypeContents {
    pub fn is_specified(&self) -> bool {
        !matches!(*self, TypeContents::NotSpecified)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnumVariantDescription {
    pub docs: Vec<String>,
    pub name: String,
    pub discriminant: usize,
    pub fields: Vec<StructFieldDescription>,
}

impl EnumVariantDescription {
    /// Used in code generation.
    ///
    /// TODO: builder pattern for more elegant code.
    pub fn new(
        docs: &[&str],
        name: &str,
        discriminant: usize,
        fields: Vec<StructFieldDescription>,
    ) -> Self {
        EnumVariantDescription {
            docs: docs.iter().map(|s| s.to_string()).collect(),
            name: name.to_string(),
            discriminant,
            fields,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructFieldDescription {
    pub docs: Vec<String>,
    pub name: String,
    pub field_type: TypeNames,
}

impl StructFieldDescription {
    /// Used in code generation.
    pub fn new(docs: &[&str], name: &str, field_type: TypeNames) -> Self {
        Self {
            docs: docs.iter().map(|s| s.to_string()).collect(),
            name: name.to_string(),
            field_type,
        }
    }
}

/// An explicit enum is an enum that gets serialized by name instead of discriminant.
///
/// This makes it easier for humans to read readable in the transaction output.
///
/// It cannot have data fields, only simple enums allowed.
#[derive(Clone, Debug)]
pub struct ExplicitEnumVariantDescription {
    pub docs: Vec<String>,
    pub name: String,
}

impl ExplicitEnumVariantDescription {
    /// Used in code generation.
    pub fn new(docs: &[&str], name: &str) -> Self {
        Self {
            docs: docs.iter().map(|s| s.to_string()).collect(),
            name: name.to_string(),
        }
    }
}
