use klever_sc::imports::*;

#[klever_sc::module]
pub trait OnlyAdminModule {
    #[view(isAdmin)]
    fn is_admin(&self, address: ManagedAddress) -> bool {
        self.admins().contains(&address)
    }

    #[only_owner]
    #[endpoint(addAdmin)]
    fn add_admin(&self, address: ManagedAddress) {
        let caller = self.blockchain().get_caller();
        self.admins().insert(address.clone());

        self.admin_added_event(&caller, &address);
    }

    #[only_owner]
    #[endpoint(removeAdmin)]
    fn remove_admin(&self, address: ManagedAddress) {
        let caller = self.blockchain().get_caller();
        self.admins().swap_remove(&address);

        self.admin_removed_event(&caller, &address);
    }

    #[view(getAdmins)]
    #[storage_mapper("only_admin_module:admins")]
    fn admins(&self) -> UnorderedSetMapper<ManagedAddress>;

    fn require_caller_is_admin(&self) {
        require!(
            self.is_admin(self.blockchain().get_caller()),
            "Endpoint can only be called by admins"
        );
    }

    #[event("admin_added")]
    fn admin_added_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] new_admin: &ManagedAddress,
    );

    #[event("admin_removed")]
    fn admin_removed_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] removed_admin: &ManagedAddress,
    );
}
