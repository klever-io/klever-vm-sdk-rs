use crate::abi::{TypeAbi, TypeDescriptionContainerImpl, TypeName};
use alloc::string::{String, ToString};

#[derive(Clone, Debug)]
pub struct KdaAttributeAbi {
    pub ticker: String,
    pub ty: TypeName,
    pub type_descriptions: TypeDescriptionContainerImpl,
}

impl KdaAttributeAbi {
    pub fn new<T: TypeAbi>(arg_name: &str) -> Self {
        let mut type_descriptions = TypeDescriptionContainerImpl::default();
        T::provide_type_descriptions(&mut type_descriptions);
        KdaAttributeAbi {
            ticker: arg_name.to_string(),
            ty: T::type_name(),
            type_descriptions,
        }
    }
}
