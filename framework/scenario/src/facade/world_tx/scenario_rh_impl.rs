use klever_sc::abi::{TypeAbi, TypeAbiFrom};
use klever_sc::types::{ReturnsRawResult, ReturnsResultUnmanaged};
use klever_sc::{
    codec::TopDecodeMulti,
    types::{
        ManagedAddress, RHListItemExec, ReturnsNewAddress, ReturnsNewManagedAddress, ReturnsResult,
        ReturnsResultAs, TxEnv, WithNewAddress, WithResultConv,
    },
};

use crate::scenario_model::{TxResponse, TypedResponse};

impl<Env, Original> RHListItemExec<TxResponse, Env, Original> for ReturnsResult
where
    Env: TxEnv,
    Original: TopDecodeMulti,
{
    fn item_process_result(self, tx_response: &TxResponse) -> Self::Returns {
        let response = TypedResponse::<Original>::from_raw(tx_response);
        response
            .result
            .expect("ReturnsExact expects that transaction is successful")
    }
}

impl<Env, Original, T> RHListItemExec<TxResponse, Env, Original> for ReturnsResultAs<T>
where
    Env: TxEnv,
    T: TopDecodeMulti + TypeAbiFrom<Original>,
{
    fn item_process_result(self, tx_response: &TxResponse) -> Self::Returns {
        let response = TypedResponse::<T>::from_raw(tx_response);
        response
            .result
            .expect("ReturnsSimilar expects that transaction is successful")
    }
}

impl<Env, Original> RHListItemExec<TxResponse, Env, Original> for ReturnsResultUnmanaged
where
    Env: TxEnv,
    Original: TypeAbi,
    Original::Unmanaged: TopDecodeMulti,
{
    fn item_process_result(self, tx_response: &TxResponse) -> Self::Returns {
        let response = TypedResponse::<Original::Unmanaged>::from_raw(tx_response);
        response
            .result
            .expect("ReturnsResultUnmanaged expects that transaction is successful")
    }
}

impl<Env, Original, T, F> RHListItemExec<TxResponse, Env, Original> for WithResultConv<T, F>
where
    Env: TxEnv,
    T: TopDecodeMulti + TypeAbiFrom<Original>,
    F: FnOnce(T),
{
    fn item_process_result(self, tx_response: &TxResponse) -> Self::Returns {
        let response = TypedResponse::<T>::from_raw(tx_response);
        let value = response
            .result
            .expect("ReturnsExact expects that transaction is successful");
        (self.f)(value);
    }
}

impl<Env, Original> RHListItemExec<TxResponse, Env, Original> for ReturnsNewAddress
where
    Env: TxEnv,
{
    fn item_process_result(self, tx_response: &TxResponse) -> Self::Returns {
        tx_response
            .new_deployed_address
            .clone()
            .expect("missing returned address")
    }
}

impl<Env, Original> RHListItemExec<TxResponse, Env, Original> for ReturnsRawResult
where
    Env: TxEnv,
{
    fn item_process_result(self, tx_response: &TxResponse) -> Self::Returns {
        tx_response.out.clone().into()
    }
}

impl<Env, Original> RHListItemExec<TxResponse, Env, Original> for ReturnsNewManagedAddress
where
    Env: TxEnv,
{
    fn item_process_result(self, tx_response: &TxResponse) -> Self::Returns {
        let new_address = tx_response
            .new_deployed_address
            .clone()
            .expect("missing returned address");

        new_address.into()
    }
}

impl<Env, Original, F> RHListItemExec<TxResponse, Env, Original> for WithNewAddress<Env, F>
where
    Env: TxEnv,
    F: FnOnce(&ManagedAddress<Env::Api>),
{
    fn item_process_result(self, tx_response: &TxResponse) -> Self::Returns {
        let new_address = tx_response
            .new_deployed_address
            .clone()
            .expect("missing returned address");

        (self.f)(&ManagedAddress::from_address(&new_address));
    }
}
