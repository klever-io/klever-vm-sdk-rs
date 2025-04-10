#![allow(deprecated)]

use klever_sc::imports::*;

use klever_sc::types::String;

/// Legacy, deprecated macros. Will b removed once they get removed.
///
/// Error conversions should be moved to corresponding new formatter-based error tests.
#[klever_sc::module]
pub trait MacroFeaturesLegacy {
    #[allow(deprecated)]
    #[view]
    fn only_owner_legacy(&self) -> SCResult<()> {
        klever_sc::only_owner!(self, "Custom only owner message");
        SCResult::Ok(())
    }

    #[view]
    fn return_sc_error(&self) -> SCResult<()> {
        klever_sc::sc_error!("return_sc_error")
    }

    #[view]
    fn result_ok(&self) -> SCResult<()> {
        SCResult::Ok(())
    }

    #[view]
    fn result_err_from_bytes(&self, e: BoxedBytes) -> SCResult<(), ManagedSCError> {
        SCResult::Err(e.into())
    }

    #[view]
    fn result_err_from_string(&self, e: String) -> SCResult<(), ManagedSCError> {
        SCResult::Err(e.into())
    }

    #[view]
    fn result_err_from_str<'a>(&self, e: &'a str) -> SCResult<(), ManagedSCError> {
        SCResult::Err(e.into())
    }

    #[endpoint]
    fn result_echo(&self, arg: Option<String>, test: bool) -> SCResult<String> {
        require!(test, "test argument is false");
        SCResult::<String, StaticSCError>::from_result(arg.ok_or("option argument is none"))
    }

    #[endpoint]
    fn result_echo_2(&self, arg: Option<String>) -> SCResult<String> {
        arg.ok_or("option argument is none").into()
    }

    #[endpoint]
    fn result_echo_3(&self, arg: Option<String>) -> String {
        let result: SCResult<String> = arg.ok_or("option argument is none").into();
        result.unwrap_or_signal_error::<Self::Api>()
    }
}
