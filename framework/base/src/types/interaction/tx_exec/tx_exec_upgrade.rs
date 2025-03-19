use crate::types::TxToSpecified;
use crate::{
    api::CallTypeApi,
    contract_base::SendRawWrapper,
    types::{
        Code, CodeMetadata, FromSource, ManagedAddress, ManagedBuffer, Tx, TxCodeValue,
        TxEmptyResultHandler, TxFromSourceValue, TxGas, TxPaymentKlvOnly, TxScEnv, UpgradeCall,
    },
};

impl<Api, Payment, Gas, CodeValue, RH>
    Tx<
        TxScEnv<Api>,
        (),
        ManagedAddress<Api>,
        Payment,
        Gas,
        UpgradeCall<TxScEnv<Api>, Code<CodeValue>>,
        RH,
    >
where
    Api: CallTypeApi,
    Payment: TxPaymentKlvOnly<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    CodeValue: TxCodeValue<TxScEnv<Api>>,
    RH: TxEmptyResultHandler<TxScEnv<Api>>,
{
    /// Launches the upgrade sync call.
    ///
    /// TODO: change return type to `!`.
    pub fn upgrade_sync_call(self) {
        let gas = self.gas.explicit_or_gas_left(&self.env);
        self.payment.with_klv_value(&self.env, |klv_value| {
            SendRawWrapper::<Api>::new().upgrade_contract(
                &self.to,
                gas,
                klv_value,
                &self.data.code_source.0.into_value(&self.env),
                self.data.code_metadata,
                &self.data.arg_buffer,
            );
        });
    }
}

impl<Api, Payment, Gas, FromSourceValue, RH>
    Tx<
        TxScEnv<Api>,
        (),
        ManagedAddress<Api>,
        Payment,
        Gas,
        UpgradeCall<TxScEnv<Api>, FromSource<FromSourceValue>>,
        RH,
    >
where
    Api: CallTypeApi,
    Payment: TxPaymentKlvOnly<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    FromSourceValue: TxFromSourceValue<TxScEnv<Api>>,
    RH: TxEmptyResultHandler<TxScEnv<Api>>,
{
    /// Launches the upgrade from source sync call.
    ///
    /// TODO: change return type to `!`.
    pub fn upgrade_sync_call(self) {
        let gas = self.gas.explicit_or_gas_left(&self.env);
        self.payment.with_klv_value(&self.env, |klv_value| {
            SendRawWrapper::<Api>::new().upgrade_from_source_contract(
                &self.to,
                gas,
                klv_value,
                &self.data.code_source.0.into_value(&self.env),
                self.data.code_metadata,
                &self.data.arg_buffer,
            );
        });
    }
}

impl<Api, To, Payment, Gas, RH>
    Tx<TxScEnv<Api>, (), To, Payment, Gas, UpgradeCall<TxScEnv<Api>, ()>, RH>
where
    Api: CallTypeApi,
    To: TxToSpecified<TxScEnv<Api>>,
    Payment: TxPaymentKlvOnly<TxScEnv<Api>>,
    Gas: TxGas<TxScEnv<Api>>,
    RH: TxEmptyResultHandler<TxScEnv<Api>>,
{
    /// Backwards compatibility, immitates the old API.
    ///
    /// Note that the data type (the `UpgradeCall`) doesn't have the code set.
    /// This is because the old API was passing it as paramter, so we use it from the `code` argument.
    ///
    /// Also note that the code metadata is taken from the `code_metadata` argument.
    /// If another one was previously set in the `Tx` object, that one will be ignored.
    pub fn upgrade_contract(self, code: &ManagedBuffer<Api>, code_metadata: CodeMetadata) {
        let gas = self.gas.explicit_or_gas_left(&self.env);
        self.payment.with_klv_value(&self.env, |klv_value| {
            self.to.with_value_ref(&self.env, |to| {
                SendRawWrapper::<Api>::new().upgrade_contract(
                    to,
                    gas,
                    klv_value,
                    code,
                    code_metadata,
                    &self.data.arg_buffer,
                );
            });
        });
    }

    /// Backwards compatibility, immitates the old API.
    ///
    /// Note that the data type (the `UpgradeCall`) doesn't have the code set.
    /// This is because the old API was passing it as paramter, so we use it from the `code` argument.
    ///
    /// Also note that the code metadata is taken from the `code_metadata` argument.
    /// If another one was previously set in the `Tx` object, that one will be ignored.
    pub fn upgrade_from_source(
        self,
        source_address: &ManagedAddress<Api>,
        code_metadata: CodeMetadata,
    ) {
        let gas = self.gas.explicit_or_gas_left(&self.env);
        self.payment.with_klv_value(&self.env, |klv_value| {
            self.to.with_value_ref(&self.env, |to| {
                SendRawWrapper::<Api>::new().upgrade_from_source_contract(
                    to,
                    gas,
                    klv_value,
                    source_address,
                    code_metadata,
                    &self.data.arg_buffer,
                );
            });
        });
    }
}
