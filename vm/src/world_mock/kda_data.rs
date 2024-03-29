use num_bigint::BigUint;
use num_traits::Zero;

use crate::{display_util::key_hex, types::VMTokenType};
use std::{
    collections::{hash_map::Iter, HashMap},
    fmt::{self, Write},
};

use super::{KdaInstanceMetadata, KdaInstances, KdaRoles};

#[derive(Clone, Default, Debug)]
pub struct KdaData {
    pub instances: KdaInstances,
    pub last_nonce: u64,
    pub roles: KdaRoles,
    pub frozen: bool,
}

impl KdaData {
    pub fn is_empty(&self) -> bool {
        self.instances.is_empty_kda()
            && self.last_nonce == 0
            && self.roles.is_empty()
            && !self.frozen
    }

    pub fn get_roles(&self) -> Vec<Vec<u8>> {
        self.roles.get()
    }
}

#[derive(Clone, Default, Debug)]
pub struct AccountKda(HashMap<Vec<u8>, KdaData>);

impl AccountKda {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get_by_identifier(&self, identifier: &[u8]) -> Option<&KdaData> {
        self.0.get(identifier)
    }

    /// Will provide a clone.
    pub fn get_roles(&self, identifier: &[u8]) -> Vec<Vec<u8>> {
        self.get_by_identifier_or_default(identifier).get_roles()
    }

    pub fn set_roles(&mut self, token_identifier: Vec<u8>, roles: Vec<Vec<u8>>) {
        let kda_data = self.0.entry(token_identifier).or_insert_with(|| KdaData {
            instances: KdaInstances::new(),
            last_nonce: 0,
            roles: KdaRoles::default(),
            frozen: false,
        });
        kda_data.roles = KdaRoles::new(roles);
    }

    /// Will provide a clone.
    pub fn get_by_identifier_or_default(&self, identifier: &[u8]) -> KdaData {
        if let Some(value) = self.0.get(identifier) {
            value.clone()
        } else {
            KdaData::default()
        }
    }

    pub fn get_mut_by_identifier(&mut self, identifier: &[u8]) -> Option<&mut KdaData> {
        self.0.get_mut(identifier)
    }

    pub fn new_from_raw_map(hash: HashMap<Vec<u8>, KdaData>) -> Self {
        AccountKda(hash)
    }

    pub fn increase_balance(
        &mut self,
        token_identifier: Vec<u8>,
        nonce: u64,
        value: &BigUint,
        metadata: KdaInstanceMetadata,
    ) {
        let kda_data = self.0.entry(token_identifier).or_insert_with(|| KdaData {
            instances: KdaInstances::new(),
            last_nonce: nonce,
            roles: KdaRoles::default(),
            frozen: false,
        });
        kda_data.instances.increase_balance(nonce, value, metadata);
    }

    pub fn set_kda_balance(
        &mut self,
        token_identifier: Vec<u8>,
        nonce: u64,
        value: &BigUint,
        metadata: KdaInstanceMetadata,
    ) {
        let kda_data = self.0.entry(token_identifier).or_insert_with(|| KdaData {
            instances: KdaInstances::new(),
            last_nonce: nonce,
            roles: KdaRoles::default(),
            frozen: false,
        });
        kda_data.instances.set_balance(nonce, value, metadata);
    }

    pub fn get_kda_balance(&self, token_identifier: &[u8], nonce: u64) -> BigUint {
        if let Some(kda_data) = self.get_by_identifier(token_identifier) {
            if let Some(instance) = kda_data.instances.get_by_nonce(nonce) {
                instance.balance.clone()
            } else {
                BigUint::zero()
            }
        } else {
            BigUint::zero()
        }
    }

    pub fn add_uris(&mut self, token_identifier: &[u8], nonce: u64, mut new_uris: Vec<Vec<u8>>) {
        self.0
            .get_mut(token_identifier)
            .unwrap_or_else(|| panic!("invalid token"))
            .instances
            .get_mut_by_nonce(nonce)
            .unwrap_or_else(|| panic!("invalid token nonce"))
            .metadata
            .uri
            .append(&mut new_uris);
    }

    pub fn update_attributes(
        &mut self,
        token_identifier: &[u8],
        nonce: u64,
        new_attribute_bytes: Vec<u8>,
    ) {
        self.0
            .get_mut(token_identifier)
            .unwrap_or_else(|| panic!("invalid token"))
            .instances
            .get_mut_by_nonce(nonce)
            .unwrap_or_else(|| panic!("invalid token nonce"))
            .metadata
            .attributes = new_attribute_bytes;
    }

    pub fn iter(&self) -> Iter<Vec<u8>, KdaData> {
        self.0.iter()
    }

    pub fn set_special_role(&mut self, token_identifier: &[u8], role: &[u8]) {
        if let Some(kda_data) = self.get_mut_by_identifier(token_identifier) {
            let roles = kda_data.roles.get();
            if !roles.contains(role.to_vec().as_ref()) {
                let mut new_roles = roles;
                new_roles.push(role.to_vec());
                kda_data.roles = KdaRoles::new(new_roles);
            }
        }
    }

    pub fn register_and_set_roles(&mut self, token_identifier: &[u8], token_type: VMTokenType) {
        self.issue_token(token_identifier);
        self.set_roles(
            token_identifier.to_vec(),
            Self::get_all_roles_for_token_type(token_type),
        );
    }

    fn issue_token(&mut self, token_identifier: &[u8]) {
        self.0.insert(
            token_identifier.to_vec(),
            KdaData {
                instances: KdaInstances::new(),
                last_nonce: 0,
                roles: KdaRoles::default(),
                frozen: false,
            },
        );
    }

    fn get_all_roles_for_token_type(token_type: VMTokenType) -> Vec<Vec<u8>> {
        match token_type {
            VMTokenType::NonFungible => vec![
                "KDARoleNFTCreate".as_bytes().to_vec(),
                "KDARoleNFTBurn".as_bytes().to_vec(),
                "KDARoleNFTUpdateAttributes".as_bytes().to_vec(),
                "KDARoleNFTAddURI".as_bytes().to_vec(),
            ],
            VMTokenType::SemiFungible | VMTokenType::Meta => vec![
                "KDARoleNFTCreate".as_bytes().to_vec(),
                "KDARoleNFTBurn".as_bytes().to_vec(),
                "KDARoleNFTAddQuantity".as_bytes().to_vec(),
            ],
            VMTokenType::Fungible => vec![
                "KDARoleLocalMint".as_bytes().to_vec(),
                "KDARoleLocalBurn".as_bytes().to_vec(),
                "KDARoleLocalTransfer".as_bytes().to_vec(),
            ],
        }
    }
}

impl fmt::Display for KdaData {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut kda_buf = String::new();
        write!(
            kda_buf,
            "{{
                instances: [{}],
                last_nonce: {},
                roles: [{}],
                frozen: {},
            }}",
            self.instances, self.last_nonce, self.roles, self.frozen
        )?;
        Ok(())
    }
}

impl fmt::Display for AccountKda {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut kda_buf = String::new();
        let kda_keys: Vec<Vec<u8>> = self.0.keys().cloned().collect();

        for key in &kda_keys {
            let value = self.0.get(key).unwrap();
            write!(kda_buf, "\n\t\t\t{} -> {value}", key_hex(key.as_slice()))?;
        }
        Ok(())
    }
}
