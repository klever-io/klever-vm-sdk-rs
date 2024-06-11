#![no_std]

use klever_sc::imports::*;
use klever_sc::derive_imports::*;

// used as mock attributes for NFTs
#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[klever_sc::contract]
pub trait LocalKdaAndKdaNft {
    #[init]
    fn init(&self) {}

    // Fungible Tokens

    #[payable("KLV")]
    #[endpoint(issueFungibleToken)]
    fn issue_fungible_token(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        initial_supply: BigUint,
    ) {
        let _caller = self.blockchain().get_caller();

        self.send()
            .kda_system_sc_proxy()
            .issue_fungible(
                &token_display_name,
                0,
                &token_ticker,
                &initial_supply,
                &BigUint::zero(),
                &PropertiesInfo {
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_mint: true,
                    can_burn: true,
                    can_change_owner: true,
                    can_add_roles: true,
                    limit_transfer: false,
                },
            );
    }

    #[endpoint(localMint)]
    fn local_mint(&self, token_identifier: TokenIdentifier, amount: BigUint) {
        self.send().kda_mint(&token_identifier, 0, &amount);
    }

    #[endpoint(localBurn)]
    fn local_burn(&self, token_identifier: TokenIdentifier, amount: BigUint) {
        self.send().kda_burn(&token_identifier, 0, &amount);
    }

    // Non-Fungible Tokens

    #[payable("KLV")]
    #[endpoint(nftIssue)]
    fn nft_issue(&self, token_display_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        let _caller = self.blockchain().get_caller();

        self.send()
            .kda_system_sc_proxy()
            .issue_non_fungible(
                &token_display_name,
                &token_ticker,
                &PropertiesInfo {
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_mint: true,
                    can_burn: true,
                    can_change_owner: true,
                    can_add_roles: true,
                    limit_transfer: false,
                },
            );
    }

    #[endpoint(nftAddQuantity)]
    fn nft_add_quantity(&self, token_identifier: TokenIdentifier, nonce: u64, amount: BigUint) {
        self.send()
            .kda_mint(&token_identifier, nonce, &amount);
    }

    #[endpoint(nftBurn)]
    fn nft_burn(&self, token_identifier: TokenIdentifier, nonce: u64, amount: BigUint) {
        self.send()
            .kda_burn(&token_identifier, nonce, &amount);
    }

    #[endpoint]
    fn transfer_nft_and_execute(
        &self,
        to: ManagedAddress,
        token_identifier: TokenIdentifier,
        nonce: u64,
        amount: BigUint,
        function: ManagedBuffer,
        arguments: MultiValueEncoded<ManagedBuffer>,
    ) {
        let mut arg_buffer = ManagedArgBuffer::new();
        for arg in arguments.into_iter() {
            arg_buffer.push_arg_raw(arg);
        }

        let _ = self.send_raw().transfer_kda_nft_execute(
            &to,
            &token_identifier,
            nonce,
            &amount,
            self.blockchain().get_gas_left(),
            &function,
            &arg_buffer,
        );
    }

    // Semi-Fungible

    // common

    #[endpoint(setLocalRoles)]
    fn set_local_roles(
        &self,
        address: ManagedAddress,
        token_identifier: TokenIdentifier,
        allow_mint: bool,
        allow_set_ito_price: bool,
        allow_deposit: bool,
        allow_transfer: bool,
    ) {
        self.send()
            .kda_system_sc_proxy()
            .set_special_roles(&address, &token_identifier, allow_mint, allow_set_ito_price, allow_deposit, allow_transfer);
    }

    // views

    #[view(getFungibleKdaBalance)]
    fn get_fungible_kda_balance(&self, token_identifier: &TokenIdentifier) -> BigUint {
        self.blockchain()
            .get_kda_balance(&self.blockchain().get_sc_address(), token_identifier, 0)
    }

    #[view(getNftBalance)]
    fn get_nft_balance(&self, token_identifier: &TokenIdentifier, nonce: u64) -> BigUint {
        self.blockchain().get_kda_balance(
            &self.blockchain().get_sc_address(),
            token_identifier,
            nonce,
        )
    }

    // storage

    #[view(lastIssuedToken)]
    #[storage_mapper("lastIssuedToken")]
    fn last_issued_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(lastErrorMessage)]
    #[storage_mapper("lastErrorMessage")]
    fn last_error_message(&self) -> SingleValueMapper<ManagedBuffer>;
}
