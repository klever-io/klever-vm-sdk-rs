use crate::abi::{TypeAbi, TypeDescriptionContainerImpl, TypeName};

#[derive(Clone, Debug)]
pub struct KdaAttributeAbi {
    pub ticker: &'static str,
    pub ty: TypeName,
    pub type_descriptions: TypeDescriptionContainerImpl
}

impl KdaAttributeAbi {
    pub fn new<T: TypeAbi>(arg_name: &'static str) -> Self {
        let mut type_descriptions = TypeDescriptionContainerImpl::default();
        T::provide_type_descriptions(&mut type_descriptions);
        KdaAttributeAbi {
            ticker: arg_name,
            ty: T::type_name(),
            type_descriptions
        }
    }
}
