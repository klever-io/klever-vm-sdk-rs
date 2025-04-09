#[klever_sc::module]
pub trait ContractBaseFullPathTestModule: klever_sc::contract_base::ContractBase {
    #[endpoint]
    fn call_contract_base_full_path_endpoint(&self) {}
}
