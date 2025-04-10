use super::*;
use crate::abi::kda_attribute_abi::KdaAttributeAbi;
use alloc::string::ToString;
use alloc::{string::String, vec::Vec};

#[derive(Debug, Default, Clone)]
pub struct ContractAbi {
    pub build_info: BuildInfoAbi,
    pub docs: Vec<String>,
    pub name: String,
    pub constructors: Vec<EndpointAbi>,
    pub upgrade_constructors: Vec<EndpointAbi>,
    pub endpoints: Vec<EndpointAbi>,
    pub events: Vec<EventAbi>,
    pub kda_attributes: Vec<KdaAttributeAbi>,
    pub type_descriptions: TypeDescriptionContainerImpl,
}

impl ContractAbi {
    /// Used in code generation.
    pub fn new(build_info: BuildInfoAbi, docs: &[&str], name: &str) -> Self {
        ContractAbi {
            build_info,
            docs: docs.iter().map(|s| s.to_string()).collect(),
            name: name.to_string(),
            constructors: Vec::new(),
            upgrade_constructors: Vec::new(),
            endpoints: Vec::new(),
            events: Vec::new(),
            kda_attributes: Vec::new(),
            type_descriptions: TypeDescriptionContainerImpl::new(),
        }
    }

    pub fn coalesce(&mut self, other: Self) {
        self.constructors
            .extend_from_slice(other.constructors.as_slice());
        self.endpoints.extend_from_slice(other.endpoints.as_slice());
        self.upgrade_constructors
            .extend_from_slice(other.upgrade_constructors.as_slice());
        self.events.extend_from_slice(other.events.as_slice());
        self.type_descriptions.insert_all(&other.type_descriptions);
        self.kda_attributes
            .extend_from_slice(other.kda_attributes.as_slice())
    }

    /// A type can provide more than 1 type descripions.
    /// For instance, a struct can also provide the descriptions of its fields.
    pub fn add_type_descriptions<T: TypeAbi>(&mut self) {
        T::provide_type_descriptions(&mut self.type_descriptions);
    }

    /// Contract main crate name.
    pub fn get_crate_name(&self) -> &str {
        self.build_info.contract_crate.name
    }

    /// Contract main crate name, but with underscores instead of dashes.
    pub fn get_crate_name_for_code(&self) -> String {
        self.get_crate_name().replace('-', "_").to_lowercase()
    }

    pub fn generate_with_endpoints(endpoints: Vec<EndpointAbi>) -> Self {
        ContractAbi {
            endpoints,
            ..Default::default()
        }
    }

    /// All exported functions: init, endpoints
    pub fn iter_all_exports(&self) -> impl Iterator<Item = &EndpointAbi> {
        self.constructors
            .iter()
            .chain(self.upgrade_constructors.iter())
            .chain(self.endpoints.iter())
    }
}
