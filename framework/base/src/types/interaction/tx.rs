use core::convert::From;
use klever_sc_codec::TopEncodeMulti;

use crate::api::CallTypeApi;
use crate::types::{AnnotatedValue, BigUint, CodeMetadata, heap::H256, KdaTokenPayment, KlvPayment, ManagedAddress, ManagedBuffer, ManagedVec, MultiKdaPayment, TxEnvMockDeployAddress, TxEnvWithTxHash, TxFromSpecified, UNSPECIFIED_GAS_LIMIT};
use crate::types::{Code, ContractCallBase, ContractCallNoPayment, ContractCallWithKlv, ContractDeploy, FromSource, KdaTokenPaymentRefs, Klv, KlvOrMultiKdaPayment, ManagedOption, TokenIdentifier, TxCodeSource, TxCodeValue, TxFromSourceValue, TxGasValue, TxKlvValue, TxProxyTrait, TxScEnv, UpgradeCall};

use super::{
    DeployCall, ExplicitGas, FunctionCall, ManagedArgBuffer, OriginalResultMarker,
    RHList, RHListAppendNoRet, RHListAppendRet, RHListItem, TxData, TxDataFunctionCall, TxEnv,
    TxFrom, TxGas, TxPayment, TxPaymentKlvOnly, TxResultHandler, TxTo,
    TxToSpecified,
};

#[must_use]
pub struct Tx<Env, From, To, Payment, Gas, Data, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
    RH: TxResultHandler<Env>,
{
    pub env: Env,
    pub from: From,
    pub to: To,
    pub payment: Payment,
    pub gas: Gas,
    pub data: Data,
    pub result_handler: RH,
}

impl<Env, From, To, Payment, Gas, Data, RH> Tx<Env, From, To, Payment, Gas, Data, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
    RH: TxResultHandler<Env>,
{
    /// TODO: does nothing, delete, added for easier copy-paste.
    #[inline]
    pub fn nothing(self) -> Tx<Env, From, To, Payment, Gas, Data, RH> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: self.data,
            result_handler: self.result_handler,
        }
    }
}

impl<Env, From, To, Payment, Gas, Data, RH> Tx<Env, From, To, Payment, Gas, Data, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    Data: TxDataFunctionCall<Env>,
    RH: TxResultHandler<Env>,
{
    pub fn to_call_data_string(&self) -> ManagedBuffer<Env::Api> {
        self.data.to_call_data_string()
    }
}

pub type TxBaseWithEnv<Env> = Tx<Env, (), (), (), (), (), ()>;

impl<Env> TxBaseWithEnv<Env>
where
    Env: TxEnv,
{
    #[inline]
    pub fn new_with_env(env: Env) -> Self {
        Tx {
            env,
            from: (),
            to: (),
            payment: (),
            gas: (),
            data: (),
            result_handler: (),
        }
    }
}

impl<Env, To, Payment, Gas, Data, RH> Tx<Env, (), To, Payment, Gas, Data, RH>
where
    Env: TxEnv,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
    RH: TxResultHandler<Env>,
{
    pub fn from<From>(self, from: From) -> Tx<Env, From, To, Payment, Gas, Data, RH>
    where
        From: TxFrom<Env>,
    {
        Tx {
            env: self.env,
            from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: self.data,
            result_handler: self.result_handler,
        }
    }
}

impl<Env, From, Payment, Gas, Data, RH> Tx<Env, From, (), Payment, Gas, Data, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
    RH: TxResultHandler<Env>,
{
    /// Specifies the recipient of the transaction.
    ///
    /// Allows argument to also be `()`.
    pub fn to<To>(self, to: To) -> Tx<Env, From, To, Payment, Gas, Data, RH>
    where
        To: TxTo<Env>,
    {
        Tx {
            env: self.env,
            from: self.from,
            to,
            payment: self.payment,
            gas: self.gas,
            data: self.data,
            result_handler: self.result_handler,
        }
    }
}

impl<Env, From, To, Gas, Data, RH> Tx<Env, From, To, (), Gas, Data, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
    RH: TxResultHandler<Env>,
{
    /// Adds any payment to a transaction, if no payment has been added before.
    pub fn payment<Payment>(self, payment: Payment) -> Tx<Env, From, To, Payment, Gas, Data, RH>
    where
        Payment: TxPayment<Env>,
    {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment,
            gas: self.gas,
            data: self.data,
            result_handler: self.result_handler,
        }
    }

    /// Adds KLV value to a transaction.
    ///
    /// Accepts any type that can represent and KLV amount: BigUint, &BigUint, etc.
    pub fn klv<KlvValue>(
        self,
        klv_value: KlvValue,
    ) -> Tx<Env, From, To, Klv<KlvValue>, Gas, Data, RH>
    where
        KlvValue: TxKlvValue<Env>,
    {
        self.payment(Klv(klv_value))
    }

    /// Backwards compatibility. Use method `klv` instead.
    pub fn with_klv_transfer(
        self,
        klv_amount: BigUint<Env::Api>,
    ) -> Tx<Env, From, To, KlvPayment<Env::Api>, Gas, Data, RH> {
        self.klv(klv_amount)
    }

    /// Adds the first single, owned KDA token payment to a transaction.
    ///
    /// Since this is the first KDA payment, a single payment tx is produced.
    ///
    /// Can subsequently be called again for multiple payments.
    pub fn kda<P: Into<KdaTokenPayment<Env::Api>>>(
        self,
        payment: P,
    ) -> Tx<Env, From, To, KdaTokenPayment<Env::Api>, Gas, Data, RH> {
        self.payment(payment.into())
    }

    /// Sets a single token payment, with the token identifier and amount kept as references.
    ///
    /// This is handy whem we only want one KDA transfer and we want to avoid unnecessary object clones.
    pub fn single_kda<'a>(
        self,
        token_identifier: &'a TokenIdentifier<Env::Api>,
        token_nonce: u64,
        amount: &'a BigUint<Env::Api>,
    ) -> Tx<Env, From, To, KdaTokenPaymentRefs<'a, Env::Api>, Gas, Data, RH> {
        self.payment(KdaTokenPaymentRefs {
            token_identifier,
            token_nonce,
            amount,
        })
    }

    /// Syntactic sugar for `self.payment(KdaTokenPaymentRefs::new(...)`. Takes references.
    pub fn klv_or_single_kda<'a>(
        self,
        token_identifier: &'a TokenIdentifier<Env::Api>,
        token_nonce: u64,
        amount: &'a BigUint<Env::Api>,
    ) -> Tx<Env, From, To, KdaTokenPaymentRefs<'a, Env::Api>, Gas, Data, RH> {
        self.payment(KdaTokenPaymentRefs::new(
            token_identifier,
            token_nonce,
            amount,
        ))
    }

    /// Sets a collection of KDA transfers as the payment of the transaction.
    ///
    /// Can be formed from single KDA payments, but the result will always be a collection.
    ///
    /// Always converts the argument into an owned collection of KDA payments. For work with references, use `.payment(&p)` instead.
    pub fn multi_kda<IntoMulti>(
        self,
        payments: IntoMulti,
    ) -> Tx<Env, From, To, MultiKdaPayment<Env::Api>, Gas, Data, RH>
    where
        IntoMulti: Into<MultiKdaPayment<Env::Api>>,
    {
        self.payment(payments.into())
    }

    /// Backwards compatibility.
    pub fn with_kda_transfer<P: Into<KdaTokenPayment<Env::Api>>>(
        self,
        payment: P,
    ) -> Tx<Env, From, To, MultiKdaPayment<Env::Api>, Gas, Data, RH> {
        self.payment(MultiKdaPayment::new())
            .with_kda_transfer(payment)
    }

    /// Backwards compatibility.
    pub fn with_multi_token_transfer(
        self,
        payments: MultiKdaPayment<Env::Api>,
    ) -> Tx<Env, From, To, MultiKdaPayment<Env::Api>, Gas, Data, RH> {
        self.multi_kda(payments)
    }

    /// Backwards compatibility.
    pub fn with_klv_or_single_kda_transfer<P: Into<KdaTokenPayment<Env::Api>>>(
        self,
        payment: P,
    ) -> Tx<Env, From, To, KdaTokenPayment<Env::Api>, Gas, Data, RH> {
        self.payment(payment.into())
    }

    pub fn klv_or_multi_kda<P: Into<KlvOrMultiKdaPayment<Env::Api>>>(
        self,
        payment: P,
    ) -> Tx<Env, From, To, KlvOrMultiKdaPayment<Env::Api>, Gas, Data, RH> {
        self.payment(payment.into())
    }
}

impl<Env, From, To, Gas, Data, RH> Tx<Env, From, To, KdaTokenPayment<Env::Api>, Gas, Data, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
    RH: TxResultHandler<Env>,
{
    /// Adds the second KDA token transfer to a contract call.
    ///
    /// Can be called multiple times on the same call.
    ///
    /// When the Tx already contains a single (owned) KDA payment,
    /// adding the second one will convert it to a list.
    pub fn kda<P: Into<KdaTokenPayment<Env::Api>>>(
        self,
        payment: P,
    ) -> Tx<Env, From, To, MultiKdaPayment<Env::Api>, Gas, Data, RH> {
        let mut payments = ManagedVec::new();
        payments.push(self.payment);
        payments.push(payment.into());
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: payments,
            gas: self.gas,
            data: self.data,
            result_handler: self.result_handler,
        }
    }
}

impl<Env, From, To, Gas, Data, RH> Tx<Env, From, To, MultiKdaPayment<Env::Api>, Gas, Data, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
    RH: TxResultHandler<Env>,
{
    /// Adds a single KDA token transfer to a contract call.
    ///
    /// Can be called multiple times on the same call.
    pub fn kda<P: Into<KdaTokenPayment<Env::Api>>>(
        mut self,
        payment: P,
    ) -> Tx<Env, From, To, MultiKdaPayment<Env::Api>, Gas, Data, RH> {
        self.payment.push(payment.into());
        self
    }

    /// When the Tx already contains an owned collection of KDA payments,
    /// calling `multi_kda` is equivalent to `kda`, it just adds another payment to the list.
    ///
    /// Can be called multiple times.
    pub fn multi_kda<P: Into<KdaTokenPayment<Env::Api>>>(
        self,
        payment: P,
    ) -> Tx<Env, From, To, MultiKdaPayment<Env::Api>, Gas, Data, RH> {
        self.kda(payment)
    }

    /// Backwards compatibility.
    pub fn with_kda_transfer<P: Into<KdaTokenPayment<Env::Api>>>(
        self,
        payment: P,
    ) -> Tx<Env, From, To, MultiKdaPayment<Env::Api>, Gas, Data, RH> {
        self.multi_kda(payment)
    }
}

impl<Env, From, To, Payment, Data, RH> Tx<Env, From, To, Payment, (), Data, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Data: TxData<Env>,
    RH: TxResultHandler<Env>,
{
    /// Sets an explicit gas limit to the call.
    #[inline]
    pub fn gas<GasValue>(
        self,
        gas_value: GasValue,
    ) -> Tx<Env, From, To, Payment, ExplicitGas<GasValue>, Data, RH>
    where
        GasValue: TxGasValue<Env>,
    {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: ExplicitGas(gas_value),
            data: self.data,
            result_handler: self.result_handler,
        }
    }

    /// Backwards compatibility.
    #[inline]
    pub fn with_gas_limit(
        self,
        gas_limit: u64,
    ) -> Tx<Env, From, To, Payment, ExplicitGas<u64>, Data, RH> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: ExplicitGas(gas_limit),
            data: self.data,
            result_handler: self.result_handler,
        }
    }
}

impl<Env, From, To, Payment, Gas, RH> Tx<Env, From, To, Payment, Gas, (), RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    RH: TxResultHandler<Env>,
{
    #[inline]
    pub fn raw_call<N: Into<ManagedBuffer<Env::Api>>>(
        self,
        function_name: N,
    ) -> Tx<Env, From, To, Payment, Gas, FunctionCall<Env::Api>, RH> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: FunctionCall::new(function_name),
            result_handler: self.result_handler,
        }
    }

    #[inline]
    pub fn function_call(
        self,
        call: FunctionCall<Env::Api>,
    ) -> Tx<Env, From, To, Payment, Gas, FunctionCall<Env::Api>, RH> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: call,
            result_handler: self.result_handler,
        }
    }
}

impl<Env, From, To, Payment, Gas, RH> Tx<Env, From, To, Payment, Gas, FunctionCall<Env::Api>, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    RH: TxResultHandler<Env>,
{
    pub fn into_function_call(self) -> FunctionCall<Env::Api> {
        self.data
    }
}

impl<Env, From, To, Payment, Gas, RH> Tx<Env, From, To, Payment, Gas, FunctionCall<Env::Api>, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxToSpecified<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    RH: TxResultHandler<Env>,
{
    /// Produces the normalized function call, i.e. with builtin function calls for KDA transfers.
    ///
    /// The resulting transaction can differ from the input in several ways:
    /// - the recipient is changed (some builtin functions are called with recipient = sender),
    /// - the function call becomes a builtin function call.
    ///
    /// ## Important
    ///
    /// Do not call this before sending transactions! Normalization is don automatically whenever necessary.
    /// Only use when you need the normalized data, e.g. for a multisig.
    ///
    /// ## Warning
    ///
    /// To produce owned values, some clones are performed.
    /// It is not optimized for contracts, but can be used nonetheless.
    #[allow(clippy::type_complexity)]
    pub fn normalize(
        self,
    ) -> Tx<
        Env,
        From,
        ManagedAddress<Env::Api>,
        KlvPayment<Env::Api>,
        Gas,
        FunctionCall<Env::Api>,
        RH,
    > {
        let (norm_to, norm_klv, norm_fc) = self.payment.with_normalized(
            &self.env,
            &self.from,
            self.to,
            self.data,
            |norm_to, norm_klv, norm_fc| (norm_to.clone(), norm_klv.clone(), norm_fc),
        );

        Tx {
            env: self.env,
            from: self.from,
            to: norm_to,
            payment: Klv(norm_klv),
            gas: self.gas,
            data: norm_fc,
            result_handler: self.result_handler,
        }
    }
}

impl<Env, From, Payment, Gas> Tx<Env, From, (), Payment, Gas, (), ()>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
{
    /// Merges the argument data into the current tx.
    /// Used for function calls originating in proxies.
    ///
    /// Different environment in the argument allowed because of compatibility with old proxies.
    ///
    /// Method still subject to considerable change.
    pub fn call<Env2, To, O>(
        self,
        call: Tx<Env2, (), To, (), (), FunctionCall<Env::Api>, OriginalResultMarker<O>>,
    ) -> Tx<Env, From, To, Payment, Gas, FunctionCall<Env::Api>, OriginalResultMarker<O>>
    where
        Env2: TxEnv<Api = Env::Api>,
        To: TxTo<Env> + TxTo<Env2>,
    {
        Tx {
            env: self.env,
            from: self.from,
            to: call.to,
            payment: self.payment,
            gas: self.gas,
            data: call.data,
            result_handler: call.result_handler,
        }
    }
}

impl<Env, From, To, Payment, Gas, RH> Tx<Env, From, To, Payment, Gas, FunctionCall<Env::Api>, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    RH: TxResultHandler<Env>,
{
    #[inline]
    pub fn function_name<N: Into<ManagedBuffer<Env::Api>>>(mut self, function_name: N) -> Self {
        self.data.function_name = function_name.into();
        self
    }

    #[inline]
    pub fn argument<T: TopEncodeMulti>(mut self, arg: &T) -> Self {
        self.data = self.data.argument(arg);
        self
    }

    #[inline]
    pub fn arguments_raw(mut self, raw: ManagedArgBuffer<Env::Api>) -> Self {
        self.data.arg_buffer = raw;
        self
    }
}

impl<Env, From, To, Payment, Gas, Data> Tx<Env, From, To, Payment, Gas, Data, ()>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
{
    #[inline]
    pub fn original_result<OriginalResult>(
        self,
    ) -> Tx<Env, From, To, Payment, Gas, Data, OriginalResultMarker<OriginalResult>> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: self.data,
            result_handler: OriginalResultMarker::new(),
        }
    }
}

impl<Env, From, To, Gas> Tx<Env, From, To, (), Gas, (), ()>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn typed<Proxy>(self, proxy: Proxy) -> Proxy::TxProxyMethods
    where
        Proxy: TxProxyTrait<Env, From, To, Gas>,
    {
        proxy.proxy_methods(self)
    }
}

impl<Env, From, To, Payment, Gas, Data, ResultList>
    Tx<Env, From, To, Payment, Gas, Data, ResultList>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    Data: TxData<Env>,
    ResultList: RHList<Env>,
{
    #[inline]
    pub fn with_result<ResultHandler>(
        self,
        result_handler: ResultHandler,
    ) -> Tx<Env, From, To, Payment, Gas, Data, ResultList::NoRetOutput>
    where
        ResultHandler: RHListItem<Env, ResultList::OriginalResult, Returns = ()>,
        ResultList: RHListAppendNoRet<Env, ResultHandler>,
    {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: self.data,
            result_handler: self.result_handler.append_no_ret(result_handler),
        }
    }

    #[inline]
    pub fn returns<RH>(
        self,
        item: RH,
    ) -> Tx<Env, From, To, Payment, Gas, Data, ResultList::RetOutput>
    where
        RH: RHListItem<Env, ResultList::OriginalResult>,
        ResultList: RHListAppendRet<Env, RH>,
    {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: self.data,
            result_handler: self.result_handler.append_ret(item),
        }
    }
}

impl<Api, To, Payment, OriginalResult> ContractCallBase<Api>
    for Tx<
        TxScEnv<Api>,
        (),
        To,
        Payment,
        (),
        FunctionCall<Api>,
        OriginalResultMarker<OriginalResult>,
    >
where
    Api: CallTypeApi + 'static,
    To: TxToSpecified<TxScEnv<Api>>,
    Payment: TxPayment<TxScEnv<Api>>,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    fn into_normalized(self) -> ContractCallWithKlv<Api, OriginalResult> {
        self.payment.with_normalized(
            &self.env,
            &self.from,
            self.to,
            self.data,
            |norm_to, norm_klv, norm_fc| ContractCallWithKlv {
                basic: ContractCallNoPayment {
                    _phantom: core::marker::PhantomData,
                    to: norm_to.clone(),
                    function_call: norm_fc.clone(),
                    explicit_gas_limit: UNSPECIFIED_GAS_LIMIT,
                    _return_type: core::marker::PhantomData,
                },
                klv_payment: norm_klv.clone(),
            },
        )
    }
}

impl<Env, From, To, Payment, Gas, RH> Tx<Env, From, To, Payment, Gas, (), RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPaymentKlvOnly<Env>,
    Gas: TxGas<Env>,
    RH: TxResultHandler<Env>,
{
    pub fn raw_deploy(self) -> Tx<Env, From, To, Payment, Gas, DeployCall<Env, ()>, RH> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: DeployCall::default(),
            result_handler: self.result_handler,
        }
    }
}

impl<Env, From, To, Payment, Gas, RH> Tx<Env, From, To, Payment, Gas, (), RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPaymentKlvOnly<Env>,
    Gas: TxGas<Env>,
    RH: TxResultHandler<Env>,
{
    pub fn raw_upgrade(self) -> Tx<Env, From, To, Payment, Gas, UpgradeCall<Env, ()>, RH> {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: UpgradeCall::default(),
            result_handler: self.result_handler,
        }
    }
}

impl<Env, From, To, Payment, Gas, RH> Tx<Env, From, To, Payment, Gas, UpgradeCall<Env, ()>, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPaymentKlvOnly<Env>,
    Gas: TxGas<Env>,
    RH: TxResultHandler<Env>,
{
    pub fn code<CodeValue>(
        self,
        code: CodeValue,
    ) -> Tx<Env, From, To, Payment, Gas, UpgradeCall<Env, Code<CodeValue>>, RH>
    where
        CodeValue: TxCodeValue<Env>,
    {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: self.data.code_source(Code(code)),
            result_handler: self.result_handler,
        }
    }

    pub fn from_source<FromSourceValue>(
        self,
        source_address: FromSourceValue,
    ) -> Tx<Env, From, To, Payment, Gas, UpgradeCall<Env, FromSource<FromSourceValue>>, RH>
    where
        FromSourceValue: TxFromSourceValue<Env>,
    {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: self.data.code_source(FromSource(source_address)),
            result_handler: self.result_handler,
        }
    }
}

impl<Env, From, To, Payment, Gas, RH> Tx<Env, From, To, Payment, Gas, DeployCall<Env, ()>, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPaymentKlvOnly<Env>,
    Gas: TxGas<Env>,
    RH: TxResultHandler<Env>,
{
    pub fn code<CodeValue>(
        self,
        code: CodeValue,
    ) -> Tx<Env, From, To, Payment, Gas, DeployCall<Env, Code<CodeValue>>, RH>
    where
        CodeValue: TxCodeValue<Env>,
    {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: self.data.code_source(Code(code)),
            result_handler: self.result_handler,
        }
    }

    pub fn from_source<FromSourceValue>(
        self,
        source_address: FromSourceValue,
    ) -> Tx<Env, From, To, Payment, Gas, DeployCall<Env, FromSource<FromSourceValue>>, RH>
    where
        FromSourceValue: TxFromSourceValue<Env>,
    {
        Tx {
            env: self.env,
            from: self.from,
            to: self.to,
            payment: self.payment,
            gas: self.gas,
            data: self.data.code_source(FromSource(source_address)),
            result_handler: self.result_handler,
        }
    }
}

impl<Env, From, To, Payment, Gas, CodeSource, RH>
    Tx<Env, From, To, Payment, Gas, DeployCall<Env, CodeSource>, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPaymentKlvOnly<Env>,
    Gas: TxGas<Env>,
    CodeSource: TxCodeSource<Env>,
    RH: TxResultHandler<Env>,
{
    pub fn code_metadata(mut self, code_metadata: CodeMetadata) -> Self {
        self.data = self.data.code_metadata(code_metadata);
        self
    }

    #[inline]
    pub fn argument<T: TopEncodeMulti>(mut self, arg: &T) -> Self {
        self.data = self.data.argument(arg);
        self
    }

    #[inline]
    pub fn arguments_raw(mut self, raw: ManagedArgBuffer<Env::Api>) -> Self {
        self.data.arg_buffer = raw;
        self
    }
}

impl<Env, From, To, Payment, Gas, CodeSource, RH>
    Tx<Env, From, To, Payment, Gas, DeployCall<Env, CodeSource>, RH>
where
    Env: TxEnvMockDeployAddress,
    From: TxFromSpecified<Env>,
    To: TxTo<Env>,
    Payment: TxPaymentKlvOnly<Env>,
    Gas: TxGas<Env>,
    CodeSource: TxCodeSource<Env>,
    RH: TxResultHandler<Env>,
{
    /// Sets the new mock address to be used for the newly deployed contract.
    ///
    /// Only allowed in tests.
    pub fn new_address<NA>(mut self, new_address: NA) -> Self
    where
        NA: AnnotatedValue<Env, ManagedAddress<Env::Api>>,
    {
        self.env.mock_deploy_new_address(&self.from, new_address);
        self
    }
}

impl<Env, From, To, Payment, Gas, Data, RH> Tx<Env, From, To, Payment, Gas, Data, RH>
where
    Env: TxEnvWithTxHash,
    From: TxFromSpecified<Env>,
    To: TxTo<Env>,
    Payment: TxPaymentKlvOnly<Env>,
    Gas: TxGas<Env>,
    Data: TxDataFunctionCall<Env>,
    RH: TxResultHandler<Env>,
{
    /// Sets the mock transaction hash to be used in a test.
    ///
    /// Only allowed in tests.
    pub fn tx_hash<H>(mut self, tx_hash: H) -> Self
    where
        H256: core::convert::From<H>,
    {
        self.env.set_tx_hash(H256::from(tx_hash));
        self
    }
}

impl<Env, From, To, Payment, Gas, CodeSource, RH>
    Tx<Env, From, To, Payment, Gas, UpgradeCall<Env, CodeSource>, RH>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Payment: TxPaymentKlvOnly<Env>,
    Gas: TxGas<Env>,
    CodeSource: TxCodeSource<Env>,
    RH: TxResultHandler<Env>,
{
    pub fn code_metadata(mut self, code_metadata: CodeMetadata) -> Self {
        self.data = self.data.code_metadata(code_metadata);
        self
    }

    #[inline]
    pub fn argument<T: TopEncodeMulti>(mut self, arg: &T) -> Self {
        self.data = self.data.argument(arg);
        self
    }

    #[inline]
    pub fn arguments_raw(mut self, raw: ManagedArgBuffer<Env::Api>) -> Self {
        self.data.arg_buffer = raw;
        self
    }
}

impl<Api, To, Payment, OriginalResult>
    From<
        Tx<
            TxScEnv<Api>,
            (),
            To,
            Payment,
            (),
            DeployCall<TxScEnv<Api>, ()>,
            OriginalResultMarker<OriginalResult>,
        >,
    > for ContractDeploy<Api, OriginalResult>
where
    Api: CallTypeApi + 'static,
    To: TxTo<TxScEnv<Api>>,
    Payment: TxPaymentKlvOnly<TxScEnv<Api>>,
    OriginalResult: TopEncodeMulti,
{
    fn from(
        value: Tx<
            TxScEnv<Api>,
            (),
            To,
            Payment,
            (),
            DeployCall<TxScEnv<Api>, ()>,
            OriginalResultMarker<OriginalResult>,
        >,
    ) -> Self {
        ContractDeploy {
            _phantom: core::marker::PhantomData,
            to: ManagedOption::none(),
            klv_payment: value.payment.into_klv_payment(&value.env),
            explicit_gas_limit: UNSPECIFIED_GAS_LIMIT,
            arg_buffer: value.data.arg_buffer,
            _return_type: core::marker::PhantomData,
        }
    }
}
