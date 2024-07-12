use crate::scenario::model::SetStateStep;

use klever_chain_vm::{
    world_mock::{
        AccountData, AccountKda, BlockInfo as CrateBlockInfo, BlockchainState, KdaData,
        KdaInstance, KdaInstanceMetadata, KdaInstances, KdaRoles,
    },
};
use klever_chain_vm::types::VMCodeMetadata;

use super::ScenarioVMRunner;

/// Refers to the default of the "setState" scenario step.
pub const DEFAULT_CODE_METADATA: VMCodeMetadata = VMCodeMetadata::all();

impl ScenarioVMRunner {
    pub fn perform_set_state(&mut self, set_state_step: &SetStateStep) {
        execute(&mut self.blockchain_mock.state, set_state_step);
    }
}

fn execute(state: &mut BlockchainState, set_state_step: &SetStateStep) {
    for (address, account) in set_state_step.accounts.iter() {
        let storage = account
            .storage
            .iter()
            .map(|(k, v)| (k.value.clone(), v.value.clone()))
            .collect();
        let kda = AccountKda::new_from_raw_map(
            account
                .kda
                .iter()
                .map(|(k, v)| (k.value.clone(), convert_mandos_kda_to_world_mock(v)))
                .collect(),
        );

        state.validate_and_add_account(AccountData {
            address: address.to_vm_address(),
            nonce: account
                .nonce
                .as_ref()
                .map(|nonce| nonce.value)
                .unwrap_or_default(),
            klv_balance: account
                .balance
                .as_ref()
                .map(|balance| balance.value.clone())
                .unwrap_or_default(),
            kda,
            username: account
                .username
                .as_ref()
                .map(|bytes_value| bytes_value.value.clone())
                .unwrap_or_default(),
            storage,
            contract_path: account
                .code
                .as_ref()
                .map(|bytes_value| bytes_value.value.clone()),
            code_metadata: account
                .code_metadata
                .as_ref()
                .map(|bytes_value| VMCodeMetadata::from(&bytes_value.value))
                .unwrap_or(DEFAULT_CODE_METADATA),
            contract_owner: account
                .owner
                .as_ref()
                .map(|address_value| address_value.to_vm_address()),
        });
    }
    for new_address in set_state_step.new_addresses.iter() {
        assert!(
            new_address.new_address.value.is_smart_contract_address(),
            "field should have SC format"
        );
        state.put_new_address(
            new_address.creator_address.to_vm_address(),
            new_address.creator_nonce.value,
            new_address.new_address.to_vm_address(),
        )
    }
    for new_token_identifier in set_state_step.new_token_identifiers.iter().cloned() {
        state.put_new_token_identifier(new_token_identifier)
    }
    if let Some(block_info_obj) = &*set_state_step.previous_block_info {
        update_block_info(&mut state.previous_block_info, block_info_obj);
    }
    if let Some(block_info_obj) = &*set_state_step.current_block_info {
        update_block_info(&mut state.current_block_info, block_info_obj);
    }
}

fn convert_mandos_kda_to_world_mock(mandos_kda: &crate::scenario::model::Kda) -> KdaData {
    match mandos_kda {
        crate::scenario::model::Kda::Short(short_kda) => {
            let balance = short_kda.value.clone();
            let mut kda_data = KdaData::default();
            kda_data.instances.add(0, balance);
            kda_data
        },
        crate::scenario::model::Kda::Full(full_kda) => KdaData {
            instances: KdaInstances::new_from_hash(
                full_kda
                    .instances
                    .iter()
                    .map(|mandos_instance| {
                        let mock_instance =
                            convert_scenario_kda_instance_to_world_mock(mandos_instance);
                        (mock_instance.nonce, mock_instance)
                    })
                    .collect(),
            ),
            last_nonce: full_kda
                .last_nonce
                .as_ref()
                .map(|last_nonce| last_nonce.value)
                .unwrap_or_default(),
            roles: KdaRoles::new(
                full_kda
                    .roles
                    .iter()
                    .map(|role| role.as_bytes().to_vec())
                    .collect(),
            ),
            frozen: if let Some(u64_value) = &full_kda.frozen {
                u64_value.value > 0
            } else {
                false
            },
        },
    }
}

fn convert_scenario_kda_instance_to_world_mock(
    scenario_kda: &crate::scenario::model::KdaInstance,
) -> KdaInstance {
    KdaInstance {
        nonce: scenario_kda
            .nonce
            .as_ref()
            .map(|nonce| nonce.value)
            .unwrap_or_default(),
        balance: scenario_kda
            .balance
            .as_ref()
            .map(|value| value.value.clone())
            .unwrap_or_default(),
        metadata: KdaInstanceMetadata {
            name: Vec::new(),
            creator: scenario_kda
                .creator
                .as_ref()
                .map(|creator| creator.to_vm_address()),
            royalties: scenario_kda
                .royalties
                .as_ref()
                .map(|royalties| royalties.value)
                .unwrap_or_default(),
            hash: scenario_kda.hash.as_ref().map(|hash| hash.value.clone()),
            uri: scenario_kda
                .uri
                .iter()
                .map(|uri| uri.value.clone())
                .collect(),
            attributes: scenario_kda
                .attributes
                .as_ref()
                .map(|attributes| attributes.value.clone())
                .unwrap_or_default(),
            can_burn: scenario_kda.can_burn.as_ref()
                .map(|m| m.clone())
                .unwrap_or_default(),
        },
    }
}

fn update_block_info(
    block_info: &mut CrateBlockInfo,
    scenario_block_info: &crate::scenario::model::BlockInfo,
) {
    if let Some(u64_value) = &scenario_block_info.block_timestamp {
        block_info.block_timestamp = u64_value.value;
    }
    if let Some(u64_value) = &scenario_block_info.block_nonce {
        block_info.block_nonce = u64_value.value;
    }
    if let Some(u64_value) = &scenario_block_info.block_epoch {
        block_info.block_epoch = u64_value.value;
    }
    if let Some(u64_value) = &scenario_block_info.block_round {
        block_info.block_round = u64_value.value;
    }
    if let Some(bytes_value) = &scenario_block_info.block_random_seed {
        const SEED_LEN: usize = 48;
        let val = &bytes_value.value;

        assert!(
            val.len() == SEED_LEN,
            "block random seed input value must be exactly 48 bytes long"
        );

        let mut seed = [0u8; SEED_LEN];
        seed[..].copy_from_slice(val.as_slice());
        block_info.block_random_seed = Box::from(seed);
    }
}
