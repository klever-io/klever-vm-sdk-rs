#![no_std]

use klever_sc::imports::*;

use klever_sc_modules::pause;

#[klever_sc::contract]
pub trait CheckPauseContract: pause::PauseModule {
    #[init]
    fn init(&self) {}

    #[endpoint(checkPause)]
    fn check_pause(&self) -> bool {
        self.is_paused()
    }
}
