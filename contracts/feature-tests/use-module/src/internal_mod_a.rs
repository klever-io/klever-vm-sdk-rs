klever_sc::imports!();

/// Example of a module that lies in the same crate.
/// It also includes another module, also from the same crate.
#[klever_sc::module]
#[kda_attribute("INMODULE", u32)]
pub trait InternalModuleA:
    super::internal_mod_b::InternalModuleB + super::internal_mod_init::InternalModuleInit
{
    #[view]
    fn call_mod_a(&self) {}

    #[view]
    #[label("module-external-view")]
    fn external_view_mod_a(&self) {}
}
