use crate::types::{AnnotatedValue, BigUint, ManagedRef};

use super::TxEnv;

pub trait TxKlvValue<Env>: AnnotatedValue<Env, BigUint<Env::Api>>
where
    Env: TxEnv,
{
}

impl<Env> TxKlvValue<Env> for BigUint<Env::Api> where Env: TxEnv {}
impl<Env> TxKlvValue<Env> for &BigUint<Env::Api> where Env: TxEnv {}
impl<Env> TxKlvValue<Env> for ManagedRef<'_, Env::Api, BigUint<Env::Api>> where Env: TxEnv {}
impl<Env> TxKlvValue<Env> for u64 where Env: TxEnv {}
