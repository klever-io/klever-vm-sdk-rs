use adder::*;
use klever_sc_scenario::imports::*;

const ADDER_PATH_EXPR: &str = "kleversc:output/adder.kleversc.json";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "kleversc:output/adder.kleversc.json",
        adder::ContractBuilder,
    );
    blockchain
}

#[test]
fn adder_whitebox() {
    let mut world = world();
    let adder_whitebox = WhiteboxContract::new("sc:adder", adder::contract_obj);
    let adder_code = world.code_expression(ADDER_PATH_EXPR);

    world
        .set_state_step(
            SetStateStep::new()
                .put_account("address:owner", Account::new().nonce(1))
                .new_address("address:owner", 2, "sc:adder"),
        )
        .whitebox_deploy(
            &adder_whitebox,
            ScDeployStep::new().from("address:owner").code(adder_code),
            |sc| {
                sc.init(5u32.into());
            },
        )
        .whitebox_query(&adder_whitebox, |sc| {
            let sum_value = sc.sum();
            assert_eq!(sum_value.get(), 5u32);
        })
        .whitebox_call(
            &adder_whitebox,
            ScCallStep::new().from("address:owner"),
            |sc| sc.add(3u32.into()),
        )
        .check_state_step(
            CheckStateStep::new()
                .put_account("address:owner", CheckAccount::new())
                .put_account(
                    "sc:adder",
                    CheckAccount::new().check_storage("str:sum", "8"),
                ),
        );
}
