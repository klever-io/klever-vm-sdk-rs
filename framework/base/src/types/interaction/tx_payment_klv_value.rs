use crate::types::BigUint;

use super::{
    AnnotatedValue, TxEnv
};

pub trait TxKlvValue<Env>: AnnotatedValue<Env, BigUint<Env::Api>>
where
    Env: TxEnv,
{
    fn with_klv_value<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R;
}

impl<Env> TxKlvValue<Env> for BigUint<Env::Api>
where
    Env: TxEnv,
{
    fn with_klv_value<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R,
    {
        f(self)
    }
}

impl<Env> TxKlvValue<Env> for &BigUint<Env::Api>
where
    Env: TxEnv,
{
    fn with_klv_value<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R,
    {
        f(*self)
    }
}

impl<Env> TxKlvValue<Env> for u64
where
    Env: TxEnv,
{
    fn with_klv_value<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R,
    {
        f(&BigUint::<Env::Api>::from(*self))
    }
}
