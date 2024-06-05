klever_sc::imports!();

#[klever_sc::derive::proxy]
pub trait UserBuiltin {
    #[endpoint(KleverSetAccountName)]
    fn set_account_name(&self, name: &ManagedBuffer);
}
