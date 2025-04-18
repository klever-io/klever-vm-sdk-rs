use klever_chain_scenario_format::serde_raw::ValueSubTree;
use klever_sc::{
    tuple_util::NestedTupleFlatten,
    types::{
        AnnotatedValue, Code, DeployCall, FunctionCall, ManagedAddress, ManagedBuffer, RHListExec,
        Tx, TxBaseWithEnv, TxCodeSource, TxCodeSourceSpecified, TxCodeValue, TxEnv,
        TxFromSpecified, TxGas, TxGasValue, TxPayment, TxToSpecified,
    },
};

use crate::{
    api::StaticApi,
    scenario_model::{
        AddressValue, BigUintValue, BytesValue, ScCallStep, ScDeployStep, ScQueryStep,
        TransferStep, TxResponse,
    },
    ScenarioEnvExec, ScenarioWorld,
};
use crate::scenario_model::U64Value;

pub fn address_annotated<Env, Addr>(env: &Env, from: Addr) -> AddressValue
where
    Env: TxEnv,
    Addr: AnnotatedValue<Env, ManagedAddress<Env::Api>>,
{
    let annotation = from.annotation(env).to_string();
    AddressValue {
        value: from.into_value(env).to_address(),
        original: ValueSubTree::Str(annotation),
    }
}

pub fn code_annotated<Env, CodeValue>(env: &Env, code: Code<CodeValue>) -> BytesValue
where
    Env: TxEnv,
    CodeValue: TxCodeValue<Env>,
{
    let annotation = code.0.annotation(env).to_string();
    BytesValue {
        value: code.0.into_value(env).to_vec(),
        original: ValueSubTree::Str(annotation),
    }
}

pub fn gas_annotated<Env, Gas>(env: &Env, gas: Gas) -> U64Value
where
    Env: TxEnv,
    Gas: TxGas<Env>,
{
    let annotation = gas.gas_annotation(env).to_string();
    U64Value {
        value: gas.gas_value(env),
        original: ValueSubTree::Str(annotation),
    }
}

pub fn tx_to_sc_call_step<Env, From, To, Payment, Gas>(
    env: &Env,
    from: From,
    to: To,
    payment: Payment,
    _gas: Gas,
    data: FunctionCall<Env::Api>,
) -> ScCallStep
where
    Env: TxEnv,
    From: TxFromSpecified<Env>,
    To: TxToSpecified<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
{
    let mut step = ScCallStep::new()
        .from(address_annotated(env, from))
        .to(address_annotated(env, to))
        .function(data.function_name.to_string().as_str());
    for arg in data.arg_buffer.iter_buffers() {
        step.tx.arguments.push(arg.to_vec().into());
    }

    let full_payment_data = payment.into_full_payment_data(env);
    if let Some(annotated_klv_payment) = full_payment_data.klv {
        step.tx.klv_value = annotated_klv_payment.into();
    }

    step
}

pub fn tx_to_sc_deploy_step<Env, From, Payment, Gas, CodeValue>(
    env: &Env,
    from: From,
    payment: Payment,
    _gas: Gas,
    data: DeployCall<Env, Code<CodeValue>>,
) -> ScDeployStep
where
    Env: TxEnv,
    From: TxFromSpecified<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    CodeValue: TxCodeValue<Env>,
{
    let mut step = ScDeployStep::new()
        .from(address_annotated(env, from))
        .code(code_annotated(env, data.code_source));
    for arg in data.arg_buffer.iter_buffers() {
        step.tx.arguments.push(arg.to_vec().into());
    }

    let full_payment_data = payment.into_full_payment_data(env);
    if let Some(annotated_klv_payment) = full_payment_data.klv {
        step.tx.klv_value = annotated_klv_payment.into();
    }

    step
}

pub fn tx_to_sc_query_step<Env, To>(env: &Env, to: To, data: FunctionCall<Env::Api>) -> ScQueryStep
where
    Env: TxEnv,
    To: TxToSpecified<Env>,
{
    let mut step = ScQueryStep::new()
        .to(address_annotated(env, to))
        .function(data.function_name.to_string().as_str());
    for arg in data.arg_buffer.iter_buffers() {
        step.tx.arguments.push(arg.to_vec().into());
    }

    step
}

pub fn tx_to_transfer_step<Env, From, To, Payment, Gas>(
    env: &Env,
    from: From,
    to: To,
    payment: Payment,
    _gas: Gas,
) -> TransferStep
where
    Env: TxEnv,
    From: TxFromSpecified<Env>,
    To: TxToSpecified<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
{
    let mut step = TransferStep::new()
        .from(address_annotated(env, from))
        .to(address_annotated(env, to));

    let full_payment_data = payment.into_full_payment_data(env);
    if let Some(annotated_klv_payment) = full_payment_data.klv {
        step.tx.klv_value = annotated_klv_payment.into();
    }

    step
}

pub fn process_result<Env, RH>(
    response: Option<TxResponse>,
    result_handler: RH,
) -> <RH::ListReturns as NestedTupleFlatten>::Unpacked
where
    Env: TxEnv,
    RH: RHListExec<TxResponse, Env>,
    RH::ListReturns: NestedTupleFlatten,
{
    let response = response.expect("step did not return result");
    let tuple_result = result_handler.list_process_result(&response);
    tuple_result.flatten_unpack()
}
