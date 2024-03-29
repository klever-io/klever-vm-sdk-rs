use klever_sc::codec::TopEncodeMulti;

klever_sc::imports!();

const CALLBACK_RESERVED_GAS_PER_TOKEN: u64 = 1_000_000;

pub type PaymentsVec<M> = ManagedVec<M, KdaTokenPayment<M>>;

#[klever_sc::module]
pub trait TransferRoleProxyModule {
    fn transfer_to_user(
        &self,
        original_caller: ManagedAddress,
        dest: ManagedAddress,
        payments: PaymentsVec<Self::Api>,
        data: ManagedBuffer,
    ) {
        let contract_call =
            ContractCallWithMultiKda::<Self::Api, ()>::new(dest, data, payments.clone());

        self.execute_call(original_caller, payments, contract_call);
    }

    fn transfer_to_contract_typed_call<T>(
        &self,
        original_caller: ManagedAddress,
        contract_call: ContractCallWithMultiKda<Self::Api, T>,
    ) where
        T: TopEncodeMulti,
    {
        self.execute_call(
            original_caller,
            contract_call.kda_payments.clone(),
            contract_call,
        );
    }

    fn transfer_to_contract_raw(
        &self,
        original_caller: ManagedAddress,
        dest: ManagedAddress,
        payments: PaymentsVec<Self::Api>,
        endpoint_name: ManagedBuffer,
        args: ManagedArgBuffer<Self::Api>,
    ) {
        let contract_call =
            ContractCallWithMultiKda::<Self::Api, ()>::new(dest, endpoint_name, payments.clone())
                .with_raw_arguments(args);

        self.execute_call(
            original_caller,
            payments,
            contract_call,
        );
    }
    
    fn execute_call<T>(
        &self,
        _original_caller: ManagedAddress,
        _initial_payments: PaymentsVec<Self::Api>,
        contract_call: ContractCallWithMultiKda<Self::Api, T>,
    ) where
        T: TopEncodeMulti,
    {
        require!(
            self.destination_whitelist()
                .contains(&contract_call.basic.to),
            "Destination address not whitelisted"
        );

        let remaining_gas = self.blockchain().get_gas_left();
        let cb_gas_needed =
            CALLBACK_RESERVED_GAS_PER_TOKEN * contract_call.kda_payments.len() as u64;
        require!(
            remaining_gas > cb_gas_needed,
            "Not enough gas to launch async call"
        );

        let async_call_gas = remaining_gas - cb_gas_needed;

        contract_call
            .with_gas_limit(async_call_gas)
            .execute_on_dest_context::<IgnoreValue>();
    }

    #[storage_mapper("transfer_role_proxy:destination_whitelist")]
    fn destination_whitelist(&self) -> UnorderedSetMapper<ManagedAddress>;
}
