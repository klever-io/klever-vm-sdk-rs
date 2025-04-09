use crate as klever_sc;
use crate::{
    codec,
    codec::derive::{TopDecode, TopEncode},
    derive::TypeAbi,
};

pub const KLEVER_TRANSFER_FUNC_NAME: &str = "KleverTransfer";
pub const KLEVER_CREATE_ASSET_FUNC_NAME: &str = "KleverCreateAsset";
pub const KLEVER_FREEZE_FUNC_NAME: &str = "KleverFreeze";
pub const KLEVER_UNFREEZE_FUNC_NAME: &str = "KleverUnfreeze";
pub const KLEVER_DELEGATE_FUNC_NAME: &str = "KleverDelegate";
pub const KLEVER_UNDELEGATE_FUNC_NAME: &str = "KleverUndelegate";
pub const KLEVER_WITHDRAW_FUNC_NAME: &str = "KleverWithdraw";
pub const KLEVER_CLAIM_FUNC_NAME: &str = "KleverClaim";
pub const KLEVER_ASSET_TRIGGER_FUNC_NAME: &str = "KleverAssetTrigger";
pub const KLEVER_SET_ACCOUNT_NAME_FUNC_NAME: &str = "KleverSetAccountName";
pub const KLEVER_VOTE_FUNC_NAME: &str = "KleverVote";
pub const KLEVER_CONFIG_ITO_FUNC_NAME: &str = "KleverConfigITO";
pub const KLEVER_BUY_FUNC_NAME: &str = "KleverBuy";
pub const KLEVER_SELL_FUNC_NAME: &str = "KleverSell";
pub const KLEVER_CANCEL_MARKET_ORDER_FUNC_NAME: &str = "KleverCancelMarketOrder";
pub const KLEVER_CREATE_MARKETPLACE_FUNC_NAME: &str = "KleverCreateMarketplace";
pub const KLEVER_CONFIG_MARKETPLACE_FUNC_NAME: &str = "KleverConfigMarketplace";
pub const KLEVER_UPDATE_ACCOUNT_PERMISSION: &str = "KleverUpdateAccountPermission";
pub const KLEVER_DEPOSIT_FUNC_NAME: &str = "KleverDeposit";
pub const KLEVER_ITO_TRIGGER_FUNC_NAME: &str = "KleverITOTrigger";
pub const CHANGE_OWNER_BUILTIN_FUNC_NAME: &str = "ChangeOwnerAddress";
pub const UPGRADE_CONTRACT_FUNC_NAME: &str = "upgradeContract";

#[derive(TopEncode, TopDecode, TypeAbi)]
pub enum AssetType {
    Fungible,
    NFT,
    SemiFungible,
}

#[derive(TopEncode, TopDecode, TypeAbi)]
pub enum AssetTriggerType {
    Mint,
    Burn,
    Wipe,
    Pause,
    Resume,
    ChangeOwner,
    AddRole,
    RemoveRole,
    UpdateMetadata,
    StopNFTMint,
    UpdateLogo,
    UpdateURIs,
    ChangeRoyaltiesReceiver,
    UpdateStaking,
    UpdateRoyalties,
    UpdateKDAFeePool,
    StopRoyaltiesChange,
    StopNFTMetadataChange,
    ChangeAdmin,
}

#[derive(TopDecode, TopEncode, TypeAbi)]
pub enum StakingType {
    APR,
    FPR,
}

#[derive(TopEncode, TopDecode, TypeAbi)]
pub enum ClaimType {
    Staking,
    Allowance,
    Market,
}

#[derive(TopEncode, TopDecode, TypeAbi)]
pub enum WithdrawType {
    Staking,
    KDAPool,
}

#[derive(TopEncode, TopDecode, TypeAbi)]
pub enum SellType {
    BuyItNow,
    Auction,
}

#[derive(TopEncode, TopDecode, TypeAbi)]
pub enum BuyType {
    ITO,
    Market,
}

#[derive(TopEncode, TopDecode, TypeAbi)]
pub enum DepositType {
    FPR,
    KDAPool,
}

#[derive(TopEncode, TopDecode, TypeAbi)]
pub enum VoteType {
    Yes,
    No,
}

#[derive(TopEncode, TopDecode, TypeAbi)]
pub enum ITOTriggerType {
    SetITOPrices,
    UpdateStatus,
    UpdateReceiverAddress,
    UpdateMaxAmount,
    UpdateDefaultLimitPerAddress,
    UpdateTimes,
    UpdateWhitelistStatus,
    AddToWhitelist,
    RemoveFromWhitelist,
    UpdateWhitelistTimes,
}

#[derive(TopEncode, TopDecode, TypeAbi)]
pub enum ITOStatus {
    Default,
    Active,
    Paused,
}

#[derive(TopEncode, TopDecode, TypeAbi)]
pub enum ITOWhitelistStatus {
    Default,
    Active,
    Paused,
}
