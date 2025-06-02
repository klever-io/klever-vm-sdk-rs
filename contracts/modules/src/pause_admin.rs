use klever_sc::imports::*;

/// Standard smart contract module that, when added to a smart contract, offers pausability.
///
/// It provides a flag that contracts can use to check if admins decided to pause the entire contract.
/// Use the features module for more granular on/off switches.
///
/// It is recommended to use this module in conjunction with the `only_admin` module.
/// It is Not recommended to use it with the `pause` module, as it may cause endpoints conflicts.
///
/// It offers:
/// * an endpoint where the admins can pause/unpause contract
/// * a method to check if contract is paused or not
///
#[klever_sc::module]
pub trait PauseAdminModule: crate::only_admin::OnlyAdminModule {
    #[inline]
    fn is_paused(&self) -> bool {
        self.paused_status().get()
    }

    #[inline]
    fn not_paused(&self) -> bool {
        !self.is_paused()
    }

    #[inline]
    fn set_paused(&self, paused: bool) {
        self.paused_status().set(paused);
    }

    #[only_admin]
    #[endpoint(pause)]
    fn pause_endpoint(&self) {
        self.set_paused(true);
        // TODO: event
    }

    #[only_admin]
    #[endpoint(unpause)]
    fn unpause_endpoint(&self) {
        self.set_paused(false);
        // TODO: event
    }

    fn require_paused(&self) {
        require!(self.is_paused(), "Contract is not paused");
    }

    fn require_not_paused(&self) {
        require!(self.not_paused(), "Contract is paused");
    }

    #[view(isPaused)]
    #[storage_mapper("pause_module:paused")]
    fn paused_status(&self) -> SingleValueMapper<bool>;
}
