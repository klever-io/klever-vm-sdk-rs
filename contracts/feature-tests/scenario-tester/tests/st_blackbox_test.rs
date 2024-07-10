use klever_sc_scenario::{imports::*, scenario_model::U64Value};

use scenario_tester::*;

const SC_SCENARIO_TESTER_PATH_EXPR: &str = "kleversc:output/scenario-tester.kleversc.json";

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const OTHER_ADDRESS: TestAddress = TestAddress::new("other");
const ST_ADDRESS: TestSCAddress = TestSCAddress::new("scenario-tester");
const CODE_PATH: KleverscPath = KleverscPath::new("output/scenario-tester.kleversc.json");
const TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("TOKEN-123456");
const NFT_ID: TestTokenIdentifier = TestTokenIdentifier::new("NFT-123456");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        SC_SCENARIO_TESTER_PATH_EXPR,
        scenario_tester::ContractBuilder,
    );
    blockchain
}

#[test]
fn st_blackbox() {
    let mut world = world();
    let owner_address = "address:owner";
    let other_address = "address:other";

    let st_contract = ContractInfo::<scenario_tester::Proxy<StaticApi>>::new("sc:scenario-tester");

    world.start_trace();

    world
        .account(OWNER_ADDRESS)
        .nonce(1)
        .balance(100)
        .account(OTHER_ADDRESS)
        .nonce(2)
        .balance(300)
        .kda_balance(TOKEN_ID, 500)
        .commit();

    world.check_state_step(
        CheckStateStep::new()
            .put_account(
                owner_address,
                CheckAccount::new()
                    .nonce(U64Value::from(1u64))
                    .balance("100"),
            )
            .put_account(
                other_address,
                CheckAccount::new()
                    .nonce(U64Value::from(2u64))
                    .balance("300")
                    .kda_balance("str:TOKEN-123456", "500"),
            ),
    );

    world.new_address(OWNER_ADDRESS, 2, ST_ADDRESS);

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(scenario_tester_proxy::ScenarioTesterProxy)
        .init(5u32)
        .code(CODE_PATH)
        .returns(ReturnsNewAddress)
        .run();
    assert_eq!(new_address, st_contract.to_address());

    let value = world
        .query()
        .to(ST_ADDRESS)
        .typed(scenario_tester_proxy::ScenarioTesterProxy)
        .sum()
        .returns(ReturnsResultUnmanaged)
        .run();
    assert_eq!(value, RustBigUint::from(5u32));

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(ST_ADDRESS)
        .typed(scenario_tester_proxy::ScenarioTesterProxy)
        .add(1u32)
        .run();

    world.check_state_step(
        CheckStateStep::new()
            .put_account(owner_address, CheckAccount::new())
            .put_account(
                &st_contract,
                CheckAccount::new().check_storage("str:sum", "6"),
            ),
    );

    world
        .tx()
        .from(OTHER_ADDRESS)
        .to(ST_ADDRESS)
        .typed(scenario_tester_proxy::ScenarioTesterProxy)
        .add(1u32)
        .run();

    world
        .tx()
        .from(OTHER_ADDRESS)
        .to(ST_ADDRESS)
        .typed(scenario_tester_proxy::ScenarioTesterProxy)
        .multi_param(MultiValue2((1u32, 1u16)))
        .run();

    world
        .tx()
        .from(OTHER_ADDRESS)
        .to(ST_ADDRESS)
        .typed(scenario_tester_proxy::ScenarioTesterProxy)
        .multi_return(1u32)
        .returns(ExpectValue(MultiValue2((1u32, 2u32))))
        .run();

    let value = world
        .tx()
        .from(OTHER_ADDRESS)
        .to(ST_ADDRESS)
        .typed(scenario_tester_proxy::ScenarioTesterProxy)
        .multi_return(1u32)
        .returns(ReturnsResultUnmanaged)
        .run();
    assert_eq!(
        value,
        MultiValue2((RustBigUint::from(1u32), RustBigUint::from(2u32)))
    );

    world.write_scenario_trace("trace1.scen.json");
}

#[test]
fn set_state_test() {
    let mut world = world();
    let first = TestAddress::new("first");
    let second = TestAddress::new("second");
    let third = TestAddress::new("third");
    let fourth = TestAddress::new("fourth");
    let fifth = TestAddress::new("fifth");
    let sixth = TestAddress::new("sixth");

    world.start_trace();

    world
        .account(first)
        .nonce(1)
        .balance(100)
        .account(second)
        .nonce(2)
        .balance(300)
        .kda_balance(TOKEN_ID, 500)
        .commit();

    world.check_state_step(
        CheckStateStep::new()
            .put_account(
                first,
                CheckAccount::new()
                    .nonce(U64Value::from(1u64))
                    .balance("100"),
            )
            .put_account(
                second,
                CheckAccount::new()
                    .nonce(U64Value::from(2u64))
                    .balance("300")
                    .kda_balance("str:TOKEN-123456", "500"),
            ),
    );

    world
        .account(third)
        .nonce(3)
        .balance(50)
        .kda_nft_balance(TOKEN_ID, 2, 1, ())
        .commit();

    world.check_state_step(
        CheckStateStep::new().put_account(
            third,
            CheckAccount::new()
                .nonce(U64Value::from(3u64))
                .balance("50")
                .kda_nft_balance_and_attributes(
                    "str:NFT-123456",
                    "2",
                    "0",
                    Some(Vec::<u8>::new()),
                ),
        ),
    );

    // using no commit should drop the value naturally
    world
        .account(fourth)
        .nonce(4)
        .balance(400)
        .account(fifth)
        .nonce(5)
        .balance(250)
        .kda_balance(TOKEN_ID, 2);

    world.check_state_step(
        CheckStateStep::new()
            .put_account(
                fourth,
                CheckAccount::new()
                    .nonce(U64Value::from(4u64))
                    .balance("400"),
            )
            .put_account(
                fifth,
                CheckAccount::new()
                    .nonce(U64Value::from(5u64))
                    .balance("250")
                    .kda_balance("str:TOKEN-123456", "2"),
            ),
    );

    world
        .account(sixth)
        .nonce(6)
        .balance(600)
        .kda_balance(TOKEN_ID, 60);

    world.check_state_step(
        CheckStateStep::new().put_account(
            sixth,
            CheckAccount::new()
                .nonce(U64Value::from(6u64))
                .balance("600")
                .kda_balance("str:TOKEN-123456", "60"),
        ),
    );
}
