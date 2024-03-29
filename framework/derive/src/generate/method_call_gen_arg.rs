use super::{convert_to_owned_type::*, util::*};
use crate::model::{Method, MethodArgument};

/// Generates an expression of the form `(a1, (a2, ... (an, ())))`,
/// where `a1 ... an` are the results of applying `arg_mapping` on each argument.
pub fn generate_arg_nested_tuple<ArgFilter, ArgMapping>(
    method_args: &[MethodArgument],
    arg_filter: ArgFilter,
    arg_mapping: ArgMapping,
) -> proc_macro2::TokenStream
where
    ArgFilter: Fn(&MethodArgument) -> bool,
    ArgMapping: Fn(&MethodArgument) -> proc_macro2::TokenStream,
{
    method_args
        .iter()
        .rev()
        .filter(|arg| arg_filter(arg))
        .fold(quote! {()}, |ts, arg| {
            let arg_tokens = arg_mapping(arg);
            quote! { ( #arg_tokens, #ts ) }
        })
}

/// In one go it generates the var names, var types, and var_names as string, all as nested tuples.
pub fn generate_arg_nested_tuples<ArgFilter>(
    method_args: &[MethodArgument],
    arg_filter: ArgFilter,
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
)
where
    ArgFilter: Fn(&MethodArgument) -> bool,
{
    let var_names = generate_arg_nested_tuple(
        method_args,
        |arg| arg_filter(arg),
        |arg| quote::ToTokens::to_token_stream(&arg.pat),
    );
    let var_types = generate_arg_nested_tuple(
        method_args,
        |arg| arg_filter(arg),
        |arg| convert_to_owned_type(&arg.ty),
    );
    let var_names_str = generate_arg_nested_tuple(
        method_args,
        |arg| arg_filter(arg),
        |arg| quote::ToTokens::to_token_stream(&pat_string(&arg.pat)),
    );
    (var_names, var_types, var_names_str)
}

pub fn generate_call_method_arg_load(m: &Method) -> proc_macro2::TokenStream {
    let (var_names, var_types, var_names_str) =
        generate_arg_nested_tuples(m.method_args.as_slice(), |arg| arg.is_endpoint_arg());
    quote! {
        let #var_names = klever_sc::io::load_endpoint_args::<Self::Api, #var_types>(#var_names_str);
    }
}
