#![no_std]

use klever_sc::imports::*;

use hex_literal::hex;

pub mod message_me_proxy;
pub mod pay_me_proxy;

static HARDCODED_ADDRESS: [u8; 32] =
    hex!("fefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefe");

#[klever_sc::contract]
pub trait ProxyTestFirst {
    #[storage_get("other_contract")]
    fn get_other_contract(&self) -> ManagedAddress;

    #[storage_set("other_contract")]
    fn set_other_contract(&self, other_contract: &ManagedAddress);

    #[storage_set("callback_info")]
    fn set_callback_info(&self, callback_info: i64);

    #[init]
    fn init(&self, other_contract_addr: &ManagedAddress) {
        self.set_other_contract(other_contract_addr);
    }

    #[payable("KLV")]
    #[endpoint(deploySecondContract)]
    fn deploy_second_contract(&self, code: ManagedBuffer) -> i32 {
        let payment = self.call_value().klv_value();

        let (address, init_result) = self
            .tx()
            .typed(message_me_proxy::MessageMeProxy)
            .init(123)
            .code(code)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsNewManagedAddress)
            .returns(ReturnsResult)
            .klv(payment)
            .sync_call();

        self.set_other_contract(&address);
        init_result + 1
    }

    #[payable("KLV")]
    #[endpoint(upgradeSecondContract)]
    fn upgrade_second_contract(&self, code: ManagedBuffer) {
        let payment = self.call_value().klv_value();
        let other_contract = self.get_other_contract();

        self.tx()
            .to(other_contract)
            .typed(pay_me_proxy::PayMeProxy)
            .upgrade()
            .argument(&456)
            .klv(payment)
            .code(code)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .upgrade_sync_call();
    }

    #[payable("KLV")]
    #[endpoint(forwardToOtherContract)]
    fn forward_to_other_contract(&self) {
        let payment = self.call_value().klv_value();
        let other_contract = self.get_other_contract();

        self.tx()
            .to(&other_contract)
            .typed(pay_me_proxy::PayMeProxy)
            .pay_me(0x56)
            .klv(payment)
            .transfer_execute();
    }

    #[payable("KLV")]
    #[endpoint(forwardToOtherContractWithCallback)]
    fn forward_to_other_contract_with_callback(&self) {
        let payment = self.call_value().klv_value();
        let other_contract = self.get_other_contract();
        self.tx()
            .to(&other_contract)
            .typed(pay_me_proxy::PayMeProxy)
            .pay_me_with_result(0x56)
            .klv(payment)
            .transfer_execute();
    }

    #[endpoint(messageOtherContract)]
    fn message_other_contract(&self) {
        let other_contract = self.get_other_contract();
        self.tx()
            .to(&other_contract)
            .typed(message_me_proxy::MessageMeProxy)
            .message_me(
                0x01,
                &BigUint::from(2u32),
                [3u8; 3].to_vec(),
                &ManagedAddress::from(&HARDCODED_ADDRESS),
            )
            .execute_on_dest_context::<IgnoreValue>();
    }
}
