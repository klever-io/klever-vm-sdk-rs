// Code generated by the klever-sc build system. DO NOT EDIT.
////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            6
// Total number of exported functions:   7

#![no_std]

klever_sc_wasm_adapter::allocator!();
klever_sc_wasm_adapter::panic_handler!();

klever_sc_wasm_adapter::endpoints! {
    nft_minter
    (
        init => init
        createNft => create_nft
        claimRoyaltiesFromMarketplace => claim_royalties_from_marketplace
        issueToken => issue_token
        setLocalRoles => set_local_roles
        buyNft => buy_nft
        getNftPrice => get_nft_price
    )
}
