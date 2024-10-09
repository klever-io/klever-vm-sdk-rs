use crate::types::VMAddress;

#[derive(Clone, Default, Debug)]
pub struct AccountPermission {
    pub address: Option<VMAddress>,
    pub operations: u64,
}
