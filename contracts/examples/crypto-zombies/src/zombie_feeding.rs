use klever_sc::imports::*;

use crate::{storage, zombie_factory, zombie_helper};
// use crypto_kitties_proxy::Kitty;

mod crypto_kitties_proxy {
    use klever_sc::derive_imports::*;

    #[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
    pub struct Kitty {
        pub is_gestating: bool,
        pub is_ready: bool,
        pub cooldown_index: u64,
        pub next_action_at: u64,
        pub siring_with_id: u64,
        pub birth_time: u64,
        pub matron_id: u64,
        pub sire_id: u64,
        pub generation: u64,
        pub genes: u64,
    }

    #[klever_sc::proxy]
    pub trait CryptoKitties {
        #[endpoint]
        fn get_kitty(&self, id: usize) -> Kitty;
    }
}

#[klever_sc::module]
pub trait ZombieFeeding:
    storage::Storage + zombie_factory::ZombieFactory + zombie_helper::ZombieHelper
{
    fn feed_and_multiply(&self, zombie_id: usize, target_dna: u64, species: ManagedBuffer) {
        let caller = self.blockchain().get_caller();
        self.check_zombie_belongs_to_caller(zombie_id, &caller);
        require!(self.is_ready(zombie_id), "Zombie is not ready");
        let my_zombie = self.zombies(&zombie_id).get();
        let dna_digits = self.dna_digits().get();
        let max_dna_value = u64::pow(10u64, dna_digits as u32);
        let verified_target_dna = target_dna % max_dna_value;
        let mut new_dna = (my_zombie.dna + verified_target_dna) / 2;
        if species == b"kitty" {
            new_dna = new_dna - new_dna % 100 + 99
        }
        self.create_zombie(caller, ManagedBuffer::from(b"NoName"), new_dna);
        self.trigger_cooldown(zombie_id);
    }

    fn trigger_cooldown(&self, zombie_id: usize) {
        let cooldown_time = self.cooldown_time().get();
        self.zombies(&zombie_id).update(|my_zombie| {
            my_zombie.ready_time = self.blockchain().get_block_timestamp() + cooldown_time
        });
    }

    #[view]
    fn is_ready(&self, zombie_id: usize) -> bool {
        let my_zombie = self.zombies(&zombie_id).get();
        my_zombie.ready_time <= self.blockchain().get_block_timestamp()
    }
}
