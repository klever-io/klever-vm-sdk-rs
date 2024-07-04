use klever_sc_derive::{ManagedVecItem, type_abi};
use crate::{
    api::{ErrorApiImpl, ManagedTypeApi, RawHandle},
    codec,
    codec::{
        derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    },
    types::{
        BigUint, KdaTokenType, ManagedAddress, ManagedBuffer, ManagedType, ManagedVec,
        TokenIdentifier,
    },
};

use crate as klever_sc; // needed by the TypeAbi generated code

const _DECODE_ATTRIBUTE_ERROR_PREFIX: &[u8] = b"error decoding KDA attributes: ";

#[type_abi]
#[derive(Clone, TopDecode, TopEncode, NestedDecode, NestedEncode, Debug, ManagedVecItem)]
pub struct KdaTokenData<M: ManagedTypeApi> {
    pub asset_type: KdaTokenType,
    pub id: ManagedBuffer<M>,
    pub name: ManagedBuffer<M>,
    pub ticker: ManagedBuffer<M>,
    pub owner_address: ManagedAddress<M>,
    pub admin_address: ManagedAddress<M>,
    pub logo: ManagedBuffer<M>,
    pub uris: ManagedVec<M, URI<M>>,
    pub precision: BigUint<M>,
    pub initial_supply: BigUint<M>,
    pub circulating_supply: BigUint<M>,
    pub max_supply: BigUint<M>,
    pub minted_value: BigUint<M>,
    pub burned_value: BigUint<M>,
    pub issue_date: BigUint<M>,
    pub royalties: RoyaltiesData<M>,
    pub properties: PropertiesInfo,
    pub attributes: AttributesInfo,
    pub roles: ManagedVec<M, RolesInfo<M>>,
}

#[type_abi]
#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, Debug)]
pub struct StakingInfo {
    pub interest_type: u32,
    pub apr: u32,
    pub min_epochs_to_claim: u32,
    pub min_epochs_to_unstake: u32,
    pub min_epochs_to_withdraw: u32,
}

impl Default for StakingInfo {
    fn default() -> Self {
        StakingInfo {
            interest_type: 0,
            apr: 0,
            min_epochs_to_claim: 0,
            min_epochs_to_unstake: 0,
            min_epochs_to_withdraw: 0,
        }
    }
}

#[type_abi]
#[derive(
    Clone, TopDecode, TopEncode, NestedDecode, NestedEncode, Debug, ManagedVecItem,
)]
pub struct PropertiesInfo {
    pub can_freeze: bool,
    pub can_wipe: bool,
    pub can_pause: bool,
    pub can_mint: bool,
    pub can_burn: bool,
    pub can_change_owner: bool,
    pub can_add_roles: bool,
    pub limit_transfer: bool,
}

fn check_bit(value: u64, bit: u8) -> bool {
    value & (1 << bit) != 0
}

impl From<u64> for PropertiesInfo {
    fn from(v: u64) -> Self {
        PropertiesInfo {
            can_freeze: check_bit(v, 0),
            can_wipe: check_bit(v, 1),
            can_pause: check_bit(v, 2),
            can_mint: check_bit(v, 3),
            can_burn: check_bit(v, 4),
            can_change_owner: check_bit(v, 5),
            can_add_roles: check_bit(v, 6),
            limit_transfer: check_bit(v, 7),
        }
    }
}

impl<M> From<BigUint<M>> for PropertiesInfo
where
    M: ManagedTypeApi,
{
    fn from(v: BigUint<M>) -> Self {
        match v.to_u64() {
            Some(value) => return PropertiesInfo::from(value),
            None => {
                M::error_api_impl().signal_error(b"Invalid value for PropertiesInfo conversion.")
            },
        }
    }
}

impl Default for PropertiesInfo {
    fn default() -> Self {
        PropertiesInfo {
            can_freeze: true,
            can_wipe: true,
            can_pause: true,
            can_mint: true,
            can_burn: true,
            can_change_owner: true,
            can_add_roles: true,
            limit_transfer: false,
        }
    }
}

#[type_abi]
#[derive(
    TopDecode, TopEncode, NestedDecode, NestedEncode, Debug, Clone, ManagedVecItem,
)]
pub struct AttributesInfo {
    pub is_paused: bool,
    pub is_nft_mint_stopped: bool,
    pub is_royalties_change_stopped: bool,
    pub is_nft_metadata_change_stopped: bool,
}

impl<M> From<BigUint<M>> for AttributesInfo
where
    M: ManagedTypeApi,
{
    fn from(v: BigUint<M>) -> Self {
        let value = v.to_u64().unwrap();
        // convert value to properties with bitmask operations
        AttributesInfo {
            is_paused: check_bit(value, 0),
            is_nft_mint_stopped: check_bit(value, 1),
            is_royalties_change_stopped: check_bit(value, 2),
            is_nft_metadata_change_stopped: check_bit(value, 3),
        }
    }
}
impl Default for AttributesInfo {
    fn default() -> Self {
        AttributesInfo {
            is_paused: false,
            is_nft_mint_stopped: false,
            is_royalties_change_stopped: false,
            is_nft_metadata_change_stopped: false,
        }
    }
}

#[type_abi]
#[derive(
    Clone, TopDecode, TopEncode, NestedDecode, NestedEncode, Debug, ManagedVecItem,
)]
pub struct RoyaltiesData<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub transfer_percentage: ManagedVec<M, RoyaltyData<M>>,
    pub transfer_fixed: BigUint<M>,
    pub market_percentage: u32,
    pub market_fixed: BigUint<M>,
    pub split_royalties: ManagedVec<M, RoyaltyInfo<M>>,
    pub ito_percentage: u32,
    pub ito_fixed: BigUint<M>,
}

pub fn get_raw_handle<M: ManagedTypeApi>(mb: &ManagedBuffer<M>, pos: usize) -> RawHandle {
    let mut dest_slice = [0u8; 4];
    mb.load_slice(pos, &mut dest_slice).unwrap();

    RawHandle::from_be_bytes(dest_slice)
}

pub fn get_u32<M: ManagedTypeApi>(mb: &ManagedBuffer<M>, pos: usize) -> u32 {
    let mut dest_slice = [0u8; 4];
    mb.load_slice(pos, &mut dest_slice).unwrap();

    u32::from_be_bytes(dest_slice)
}

impl<M> From<ManagedBuffer<M>> for RoyaltiesData<M>
where
    M: ManagedTypeApi,
{
    fn from(v: ManagedBuffer<M>) -> Self {
        if v.is_empty() {
            return Default::default();
        }

        let address_handle = get_raw_handle(&v, 0);
        let transfer_percentage_handle = get_raw_handle(&v, 4);
        let transfer_fixed_handle = get_raw_handle(&v, 8);
        let market_percentage = get_u32(&v, 12);
        let market_fixed_handle = get_raw_handle(&v, 16);
        let split_royalties = get_raw_handle(&v, 20);
        let ito_fixed_handle = get_raw_handle(&v, 24);
        let ito_percentage = get_u32(&v, 28);

        RoyaltiesData {
            address: ManagedAddress::from_raw_handle(address_handle),
            transfer_percentage: ManagedVec::from_raw_handle(transfer_percentage_handle),
            transfer_fixed: BigUint::from_raw_handle(transfer_fixed_handle),
            market_percentage: market_percentage,
            market_fixed: BigUint::from_raw_handle(market_fixed_handle),
            split_royalties: ManagedVec::from_raw_handle(split_royalties),
            ito_fixed: BigUint::from_raw_handle(ito_fixed_handle),
            ito_percentage: ito_percentage,
            ..Default::default()
        }
    }
}

impl<M: ManagedTypeApi> Default for RoyaltiesData<M> {
    fn default() -> Self {
        RoyaltiesData {
            address: ManagedAddress::default(),
            transfer_percentage: ManagedVec::default(),
            transfer_fixed: BigUint::default(),
            market_percentage: 0,
            market_fixed: BigUint::default(),
            split_royalties: ManagedVec::default(),
            ito_percentage: 0,
            ito_fixed: BigUint::default(),
        }
    }
}

pub fn convert_buff_to_roles<M: ManagedTypeApi>(
    mb: ManagedBuffer<M>,
) -> ManagedVec<M, RolesInfo<M>> {
    let mut roles = ManagedVec::new();
    let mut i = 0;

    while (i + 1) * 8 <= mb.len() {
        let dest_slice = mb.copy_slice(8 * i, 8).unwrap();

        roles.push(RolesInfo::from(dest_slice.clone()));
        i += 1;
    }

    roles
}

impl<M> From<ManagedBuffer<M>> for RolesInfo<M>
where
    M: ManagedTypeApi,
{
    fn from(v: ManagedBuffer<M>) -> Self {
        let address_handle = get_raw_handle(&v, 0);
        let bools_handle = get_u32(&v, 4);

        RolesInfo {
            address: ManagedAddress::from_raw_handle(address_handle),
            has_role_mint: check_bit(bools_handle as u64, 0),
            has_role_set_ito_prices: check_bit(bools_handle as u64, 1),
            has_role_deposit: check_bit(bools_handle as u64, 2),
            has_role_transfer: check_bit(bools_handle as u64, 3),
        }
    }
}

#[type_abi]
#[derive(
    ManagedVecItem, Clone, TopDecode, TopEncode, NestedDecode, NestedEncode, Debug,
)]
pub struct RoyaltyInfo<M: ManagedTypeApi> {
    pub key: ManagedAddress<M>,
    pub percent_transfer_percentage: u32,
    pub percent_transfer_fixed: u32,
    pub percent_market_percentage: u32,
    pub percent_market_fixed: u32,
    pub percent_ito_percentage: u32,
    pub percent_ito_fixed: u32,
}

#[type_abi]
#[derive(
    ManagedVecItem, Clone, TopDecode, TopEncode, NestedDecode, NestedEncode, Debug,
)]
pub struct URI<M: ManagedTypeApi> {
    pub key: ManagedBuffer<M>,
    pub value: ManagedBuffer<M>,
}

#[type_abi]
#[derive(
    ManagedVecItem, Clone, TopDecode, TopEncode, NestedDecode, NestedEncode, Debug,
)]
pub struct RoyaltyData<M: ManagedTypeApi> {
    pub amount: BigUint<M>,
    pub percentage: u32,
}

#[type_abi]
#[derive(
    ManagedVecItem, Clone, TopDecode, TopEncode, NestedDecode, NestedEncode, Debug,
)]
pub struct RolesInfo<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub has_role_mint: bool,
    pub has_role_set_ito_prices: bool,
    pub has_role_deposit: bool,
    has_role_transfer: bool,
}

#[type_abi]
#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, Debug)]
pub struct RoyaltySplitData<M: ManagedTypeApi> {
    pub percent_transfer_percentage: u32,
    pub percent_transfer_fixed: BigUint<M>,
    pub percent_market_percentage: u32,
    pub percent_market_fixed: BigUint<M>,
    pub percent_ito_percentage: u32,
    pub percent_ito_fixed: BigUint<M>,
}
// TODO: implement user kda

impl<M: ManagedTypeApi> Default for KdaTokenData<M> {
    fn default() -> Self {
        KdaTokenData {
            asset_type: KdaTokenType::Fungible,
            id: ManagedBuffer::default(),
            name: ManagedBuffer::default(),
            ticker: ManagedBuffer::default(),
            owner_address: ManagedAddress::default(),
            admin_address: ManagedAddress::default(),
            logo: ManagedBuffer::default(),
            uris: ManagedVec::default(),
            precision: BigUint::default(),
            initial_supply: BigUint::default(),
            circulating_supply: BigUint::default(),
            max_supply: BigUint::default(),
            minted_value: BigUint::default(),
            burned_value: BigUint::default(),
            issue_date: BigUint::default(),
            royalties: RoyaltiesData::default(),
            properties: PropertiesInfo::default(),
            attributes: AttributesInfo::default(),
            roles: ManagedVec::default(),
        }
    }
}

#[type_abi]
#[derive(
    ManagedVecItem, Clone, TopDecode, TopEncode, NestedDecode, NestedEncode, Debug,
)]
pub struct ITOPackInfo<M: ManagedTypeApi> {
    pub token: TokenIdentifier<M>,
    pub packs: ManagedVec<M, ITOPackItem<M>>,
}

#[type_abi]
#[derive(
    ManagedVecItem, Clone, TopDecode, TopEncode, NestedDecode, NestedEncode, Debug,
)]
pub struct ITOPackItem<M: ManagedTypeApi> {
    pub amount: BigUint<M>,
    pub price: BigUint<M>,
}

#[type_abi]
#[derive(
    ManagedVecItem, Clone, TopDecode, TopEncode, NestedDecode, NestedEncode, Debug,
)]
pub struct ITOWhitelist<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub limit: BigUint<M>,
}
