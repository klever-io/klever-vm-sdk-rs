use std::{fs::File, io::Write};

use klever_sc::abi::{ContractAbi, EndpointAbi, InputAbi, OutputAbi};

use super::proxy_naming::proxy_methods_type_name;

pub(crate) fn write_content(file: &mut File, abi: ContractAbi) {
    write_header_impl_constructor(file, &abi.name);
    for (i, constructor_abi) in abi.constructors.into_iter().enumerate() {
        if i > 0 {
            writeln!(file).unwrap();
        }
        write_constructor_header(file, constructor_abi.clone());
        write_constructor_content(file, constructor_abi.inputs);
        write_end_of_function(file);
    }

    writeln!(file, "}}").unwrap();

    write_header_impl_upgrade_constructor(file, &abi.name);
    for (i, upgrade_abi) in abi.upgrade_constructors.clone().into_iter().enumerate() {
        if i > 0 {
            writeln!(file).unwrap();
        }
        write_upgrade_constructor_header(file, upgrade_abi.clone());
        write_upgrade_constructor_content(file, upgrade_abi.inputs);
        write_end_of_function(file);
    }

    writeln!(file, "}}").unwrap();

    write_header_impl_endpoints(file, &abi.name);
    for (i, endpoint_abi) in abi.endpoints.into_iter().enumerate() {
        if i > 0 {
            writeln!(file).unwrap();
        }
        write_endpoint_header(file, endpoint_abi.clone());
        write_endpoint_content(file, endpoint_abi.name, endpoint_abi.inputs);
        write_end_of_function(file);
    }

    writeln!(file, "}}").unwrap();
}

fn write_header_impl_constructor(file: &mut File, name: &str) {
    let proxy_methods_type_name = proxy_methods_type_name(name);
    writeln!(
        file,
        r#"
#[rustfmt::skip]
impl<Env, From, Gas> {proxy_methods_type_name}<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{{"#
    )
    .unwrap();
}

fn write_header_impl_upgrade_constructor(file: &mut File, name: &str) {
    let proxy_methods_type_name = proxy_methods_type_name(name);
    writeln!(
        file,
        r#"
#[rustfmt::skip]
impl<Env, From, To, Gas> {proxy_methods_type_name}<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{{"#
    )
    .unwrap();
}

fn write_header_impl_endpoints(file: &mut File, name: &str) {
    let proxy_methods_type_name = proxy_methods_type_name(name);
    writeln!(
        file,
        r#"
#[rustfmt::skip]
impl<Env, From, To, Gas> {proxy_methods_type_name}<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{{"#
    )
    .unwrap();
}

fn write_constructor_header(file: &mut File, constructor_abi: EndpointAbi) {
    write_fn_signature(file, constructor_abi.clone());
    write_constructor_output(file, constructor_abi.outputs);
}

fn write_upgrade_constructor_header(file: &mut File, upgrade_constructor_abi: EndpointAbi) {
    write_fn_signature(file, upgrade_constructor_abi.clone());
    write_upgrade_constructor_output(file, upgrade_constructor_abi.outputs);
}

fn write_endpoint_header(file: &mut File, constructor_abi: EndpointAbi) {
    write_fn_signature(file, constructor_abi.clone());
    write_endpoint_output(file, constructor_abi.outputs);
}

fn write_fn_signature(file: &mut File, endpoint_abi: EndpointAbi) {
    write_endpoint_docs(file, endpoint_abi.docs);
    write_function_header_endpoint(file, endpoint_abi.rust_method_name);
    write_args(file, endpoint_abi.inputs.clone());
    write_parameters(file, endpoint_abi.inputs);
}

fn write_parameters(file: &mut File, inputs: Vec<InputAbi>) {
    writeln!(file, "(").unwrap();
    writeln!(file, "        self,").unwrap();
    for (index, input) in inputs.iter().enumerate() {
        writeln!(file, "        {}: Arg{index},", &input.arg_name).unwrap();
    }
    write!(file, "    ) ").unwrap();
}

fn write_constructor_output(file: &mut File, outputs: Vec<OutputAbi>) {
    write!(file, "-> TxProxyDeploy<Env, From, Gas, ").unwrap();

    parse_and_write_outputs(file, outputs);

    writeln!(file, "> {{").unwrap();
}

fn write_upgrade_constructor_output(file: &mut File, outputs: Vec<OutputAbi>) {
    write!(file, "-> TxProxyUpgrade<Env, From, To, Gas, ").unwrap();

    parse_and_write_outputs(file, outputs);

    writeln!(file, "> {{").unwrap();
}

fn write_endpoint_output(file: &mut File, outputs: Vec<OutputAbi>) {
    write!(file, "-> TxProxyCall<Env, From, To, Gas, ").unwrap();

    parse_and_write_outputs(file, outputs);

    writeln!(file, "> {{").unwrap();
}

fn write_constructor_content(file: &mut File, inputs: Vec<InputAbi>) {
    writeln!(
        file,
        "        self.wrapped_tx
            .raw_deploy()"
    )
    .unwrap();
    for input in inputs.iter() {
        writeln!(file, "            .argument(&{})", input.arg_name).unwrap();
    }
    writeln!(file, "            .original_result()").unwrap();
}

fn write_upgrade_constructor_content(file: &mut File, inputs: Vec<InputAbi>) {
    writeln!(
        file,
        "        self.wrapped_tx
            .raw_upgrade()"
    )
    .unwrap();
    for input in inputs.iter() {
        writeln!(file, "            .argument(&{})", input.arg_name).unwrap();
    }
    writeln!(file, "            .original_result()").unwrap();
}

fn write_endpoint_content(file: &mut File, function_name: String, inputs: Vec<InputAbi>) {
    writeln!(
        file,
        "        self.wrapped_tx
            .raw_call(\"{}\")",
        function_name
    )
    .unwrap();

    for input in inputs.iter() {
        writeln!(file, "            .argument(&{})", input.arg_name).unwrap();
    }

    writeln!(file, "            .original_result()").unwrap();
}

fn write_function_header_endpoint(file: &mut File, rust_method_name: String) {
    write!(file, "    pub fn {rust_method_name}").unwrap();
}

fn write_endpoint_docs(file: &mut File, docs: Vec<String>) {
    for abi_doc in docs {
        writeln!(file, "    /// {abi_doc} ").unwrap();
    }
}

fn write_args(file: &mut File, inputs: Vec<InputAbi>) {
    if inputs.is_empty() {
        return;
    }

    writeln!(file, "<").unwrap();

    for (index, input) in inputs.iter().enumerate() {
        write_argument(file, index, &input.type_names.rust);
    }

    write!(file, "    >").unwrap();
}

fn write_argument(file: &mut File, index: usize, rust_name: &str) {
    let adjusted = adjust_type_name(rust_name);
    writeln!(file, "        Arg{index}: CodecInto<{adjusted}>,").unwrap();
}

fn write_end_of_function(file: &mut File) {
    writeln!(file, "    }}").unwrap();
}

fn adjust_type_name(original_rust_name: &str) -> String {
    original_rust_name
        .replace("klever_sc::api::uncallable::UncallableApi", "Env::Api")
        .replace("$API", "Env::Api")
        .to_string()
}

fn parse_and_write_outputs(file: &mut File, outputs: Vec<OutputAbi>) {
    match outputs.len() {
        0 => {
            write!(file, "()").unwrap();
        },
        1 => {
            let adjusted = adjust_type_name(&outputs[0].type_names.rust);
            write!(file, "{adjusted}").unwrap();
        },
        _ => {
            write!(file, "MultiValue{}<", outputs.len()).unwrap();
            for (i, output) in outputs.iter().enumerate() {
                if i > 0 {
                    write!(file, ", ").unwrap();
                }
                let adjusted = adjust_type_name(&output.type_names.rust);
                write!(file, "{adjusted}").unwrap();
            }
            write!(file, ">").unwrap();
        },
    }
}
