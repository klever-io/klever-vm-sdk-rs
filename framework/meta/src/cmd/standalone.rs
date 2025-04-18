mod all;
mod info;
pub mod install;
mod local_deps;
mod print_util;
pub mod scen_test_gen;
pub mod template;
mod test;
mod test_coverage;
pub(crate) mod upgrade;

use crate::{
    cli_args::{StandaloneCliAction, StandaloneCliArgs},
    cmd::standalone::test_coverage::test_coverage,
};
use all::call_all_meta;
use clap::Parser;
use info::call_info;
use install::install;
use local_deps::local_deps;
use scen_test_gen::test_gen_tool;
use template::{create_contract, print_template_names};
use test::test;
use upgrade::upgrade_sc;

/// Entry point in the program when calling it as a standalone tool.
pub fn cli_main_standalone() {
    let cli_args = StandaloneCliArgs::parse();
    match &cli_args.command {
        Some(StandaloneCliAction::Info(args)) => call_info(args),
        Some(StandaloneCliAction::All(args)) => call_all_meta(args),
        Some(StandaloneCliAction::Upgrade(args)) => {
            upgrade_sc(args);
        },
        Some(StandaloneCliAction::LocalDeps(args)) => {
            local_deps(args);
        },
        Some(StandaloneCliAction::Template(args)) => {
            create_contract(args);
        },
        Some(StandaloneCliAction::TemplateList(args)) => {
            print_template_names(args);
        },
        Some(StandaloneCliAction::TestGen(args)) => {
            test_gen_tool(args);
        },
        Some(StandaloneCliAction::Test(args)) => test(args),
        Some(StandaloneCliAction::TestCoverage(args)) => {
            test_coverage(args);
        },
        Some(StandaloneCliAction::Install(args)) => install(args),
        None => {},
    }
}
