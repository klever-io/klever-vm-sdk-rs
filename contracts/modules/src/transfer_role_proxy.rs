use klever_sc::codec::TopEncodeMulti;

use klever_sc::imports::*;

const CALLBACK_RESERVED_GAS_PER_TOKEN: u64 = 1_000_000;

pub type PaymentsVec<M> = ManagedVec<M, KdaTokenPayment<M>>;

#[klever_sc::module]
pub trait TransferRoleProxyModule {
    fn transfer_to_user(
        &self,
        original_caller: ManagedAddress,
        dest: ManagedAddress,
        payments: &PaymentsVec<Self::Api>,
        data: ManagedBuffer,
    ) {
        let transaction = self.tx().to(&dest).raw_call(data).payment(payments);

        self.execute_call(original_caller, payments, transaction)
    }

    fn transfer_to_contract_typed_call<T>(
        &self,
        original_caller: ManagedAddress,
        transaction: Tx<
            TxScEnv<Self::Api>,
            (),
            &ManagedAddress,
            &ManagedVec<Self::Api, KdaTokenPayment<Self::Api>>,
            (),
            FunctionCall<Self::Api>,
            (),
        >,
    ) where
        T: TopEncodeMulti,
    {
        self.execute_call(original_caller, transaction.payment, transaction);
    }

    fn transfer_to_contract_raw(
        &self,
        original_caller: ManagedAddress,
        dest: ManagedAddress,
        payments: &PaymentsVec<Self::Api>,
        endpoint_name: ManagedBuffer,
        args: ManagedArgBuffer<Self::Api>,
    ) {
        let transaction = self
            .tx()
            .to(&dest)
            .raw_call(endpoint_name)
            .payment(payments)
            .arguments_raw(args);

        self.execute_call(original_caller, payments, transaction)
    }

    fn execute_call(
        &self,
        _original_caller: ManagedAddress,
        _initial_payments: &PaymentsVec<Self::Api>,
        transaction: Tx<
            TxScEnv<Self::Api>,
            (),
            &ManagedAddress,
            &ManagedVec<Self::Api, KdaTokenPayment<Self::Api>>,
            (),
            FunctionCall<Self::Api>,
            (),
        >,
    ) {
        require!(
            self.destination_whitelist().contains(transaction.to),
            "Destination address not whitelisted"
        );

        let remaining_gas = self.blockchain().get_gas_left();
        let cb_gas_needed = CALLBACK_RESERVED_GAS_PER_TOKEN * transaction.payment.len() as u64;
        require!(
            remaining_gas > cb_gas_needed,
            "Not enough gas to launch async call"
        );

        transaction.sync_call();
    }

    #[storage_mapper("transfer_role_proxy:destination_whitelist")]
    fn destination_whitelist(&self) -> UnorderedSetMapper<ManagedAddress>;
}
