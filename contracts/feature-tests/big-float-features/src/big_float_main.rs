#![no_std]

pub mod big_float_methods;
pub mod big_float_methods_wrapped;
pub mod big_float_operators;
pub mod big_float_operators_wrapped;

#[klever_sc::contract]
pub trait BigFloatFeatures:
    big_float_methods::BigFloatMethods
    + big_float_operators::BigFloatOperators
    + big_float_methods_wrapped::BigFloatWrappedMethods
    + big_float_operators_wrapped::BigFloatWrappedOperators
{
    #[init]
    fn init(&self) {}
}
