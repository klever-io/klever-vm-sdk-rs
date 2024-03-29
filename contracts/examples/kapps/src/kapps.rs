#![no_std]

klever_sc::imports!();
#[klever_sc::contract]
pub trait Kapps {
    #[init]
    fn init(&self) {}

    #[event("testEvent1")]
    fn test_event(
        &self,
        #[indexed] source_address: &ManagedAddress,
        #[indexed] ticker: &TokenIdentifier,
        #[indexed] value: &BigUint,
    );

    #[payable("*")]
    #[endpoint]
    fn deposit_check(&self) -> SCResult<bool> {
        let payments = self.call_value().all_kda_transfers();
        payments.iter().for_each(|payment| {
            self.test_event(
                &self.blockchain().get_caller(),
                &payment.token_identifier,
                &payment.amount,
            );
        });

        require!(
            payments.len() == 1,
            "Invalid payment, two tokens are needed"
        );

        return Ok(true);
    }

    #[payable("*")]
    #[endpoint]
    fn transfer_kda(
        &self,
        to: ManagedAddress,
        token: TokenIdentifier,
        nonce: u64,
        amount: BigUint,
    ) -> SCResult<bool> {
        self.send().direct_kda(&to, &token, nonce, &amount);
        return Ok(true);
    }

    #[payable("*")]
    #[endpoint]
    fn kda_create(&self) -> TokenIdentifier {
        let to = self.blockchain().get_sc_address();
        let supply = BigUint::from(100000u64);

        let result = self.send().kda_create(
            klever_sc::api::AssetType::Fungible,
            &ManagedBuffer::from("TEST"),
            &ManagedBuffer::from("TEST"),
            6,
            &to,
            &ManagedBuffer::new(),
            &supply,
            &BigUint::from(0u32),
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
            &RoyaltiesData::default(),
        );

        return result;
    }

    #[payable("*")]
    #[endpoint]
    fn kda_create_nft(&self) -> TokenIdentifier {
        let to = self.blockchain().get_sc_address();
        let supply = BigUint::default();

        let result = self.send().kda_create(
            klever_sc::api::AssetType::NFT,
            &ManagedBuffer::from("TESTNFT"),
            &ManagedBuffer::from("TESTNFT"),
            0,
            &to,
            &ManagedBuffer::new(),
            &supply,
            &BigUint::from(0u32),
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
            &RoyaltiesData::default(),
        );

        return result;
    }

    #[endpoint]
    fn mint(&self, token: TokenIdentifier, amount: BigUint) -> SCResult<bool> {
        self.send().kda_mint(&token, 0, &amount);

        Ok(true)
    }

    #[endpoint]
    fn burn(&self, token: TokenIdentifier, nonce: u64, amount: BigUint) -> SCResult<()> {
        self.send().kda_burn(&token, nonce, &amount);
        Ok(())
    }

    #[endpoint]
    fn wipe(
        &self,
        token: TokenIdentifier,
        nonce: u64,
        amount: BigUint,
        address: ManagedAddress,
    ) -> SCResult<()> {
        self.send().kda_wipe(&token, nonce, &amount, &address);
        Ok(())
    }

    #[endpoint]
    fn pause(&self, token: TokenIdentifier) -> SCResult<()> {
        self.send().kda_pause(&token);
        Ok(())
    }

    #[endpoint]
    fn resume(&self, token: TokenIdentifier) -> SCResult<()> {
        self.send().kda_resume(&token);
        Ok(())
    }

    #[endpoint]
    fn change_owner(&self, token: TokenIdentifier, new_owner: ManagedAddress) -> SCResult<()> {
        self.send().kda_change_owner(&token, &new_owner);
        Ok(())
    }

    #[endpoint]
    fn add_role(
        &self,
        token: TokenIdentifier,
        address: ManagedAddress,
        has_role_mint: bool,
        has_role_set_ito_prices: bool,
        has_role_deposit: bool,
        has_role_transfer: bool,
    ) -> SCResult<()> {
        self.send().kda_add_role(
            &token,
            &address,
            has_role_mint,
            has_role_set_ito_prices,
            has_role_deposit,
            has_role_transfer,
        );
        Ok(())
    }

    #[endpoint]
    fn remove_role(&self, token: TokenIdentifier, address: ManagedAddress) -> SCResult<()> {
        self.send().kda_remove_role(&token, &address);
        Ok(())
    }

    #[endpoint]
    fn update_metadata(
        &self,
        token: TokenIdentifier,
        nonce: u64,
        address: ManagedAddress,
        mime: ManagedBuffer,
        metadata: ManagedBuffer,
    ) -> SCResult<()> {
        self.send()
            .kda_update_metadata(&token, nonce, &address, &mime, &metadata);
        Ok(())
    }

    #[endpoint]
    fn stop_nft_mint(&self, token: TokenIdentifier) -> SCResult<()> {
        self.send().kda_stop_nft_mint(&token);
        Ok(())
    }

    #[endpoint]
    fn update_logo(&self, token: TokenIdentifier, logo: ManagedBuffer) -> SCResult<()> {
        self.send().kda_update_logo(&token, &logo);
        Ok(())
    }

    #[endpoint]
    fn change_royalties_receiver(
        &self,
        token: TokenIdentifier,
        address: ManagedAddress,
    ) -> SCResult<()> {
        self.send().kda_change_royalties_receiver(&token, &address);
        Ok(())
    }

    #[endpoint]
    fn update_staking(
        &self,
        token: TokenIdentifier,
        staking_type: StakingType,
        apr: u32,
        min_epochs_to_claim: u32,
        min_epochs_to_unstake: u32,
        min_epochs_to_withdraw: u32,
    ) -> SCResult<()> {
        self.send().kda_update_staking(
            &token,
            staking_type,
            apr,
            min_epochs_to_claim,
            min_epochs_to_unstake,
            min_epochs_to_withdraw,
        );
        Ok(())
    }

    #[endpoint]
    fn update_fee_pool(
        &self,
        token: TokenIdentifier,
        is_active: bool,
        admin_address: ManagedAddress,
        f_ratio_kda: u64,
        f_ratio_klv: u64,
    ) -> SCResult<()> {
        self.send().kda_update_fee_pool(
            &token,
            is_active,
            &admin_address,
            f_ratio_kda,
            f_ratio_klv,
        );
        Ok(())
    }

    #[endpoint]
    fn stop_royalties_change(&self, token: TokenIdentifier) -> SCResult<()> {
        self.send().kda_stop_royalties_change(&token);
        Ok(())
    }

    #[endpoint]
    fn stop_metadata_change(&self, token: TokenIdentifier) -> SCResult<()> {
        self.send().kda_stop_metadata_change(&token);
        Ok(())
    }

    #[endpoint]
    fn update_royalties(&self, token: TokenIdentifier, to: ManagedAddress) -> SCResult<()> {
        let mut transfer_percentage = ManagedVec::<Self::Api, RoyaltyData<Self::Api>>::new();

        transfer_percentage.push(RoyaltyData {
            amount: BigUint::from(9u64),
            percentage: 9,
        });

        transfer_percentage.push(RoyaltyData {
            amount: BigUint::from(10u64),
            percentage: 10,
        });

        let mut split = ManagedVec::<Self::Api, RoyaltyInfo<Self::Api>>::new();

        split.push(RoyaltyInfo {
            key: self.blockchain().get_sc_address().clone(),
            percent_transfer_percentage: 7,
            percent_transfer_fixed: 7,
            percent_market_percentage: 7,
            percent_market_fixed: 7,
            percent_ito_percentage: 7,
            percent_ito_fixed: 7,
        });

        split.push(RoyaltyInfo {
            key: to.clone(),
            percent_transfer_percentage: 8,
            percent_transfer_fixed: 8,
            percent_market_percentage: 8,
            percent_market_fixed: 8,
            percent_ito_percentage: 8,
            percent_ito_fixed: 8,
        });

        self.send().kda_update_royalties(
            &token,
            RoyaltiesData {
                address: to.clone(),
                transfer_fixed: BigUint::from(256u64),
                market_percentage: 2,
                market_fixed: BigUint::from(3u64),
                ito_percentage: 4,
                ito_fixed: BigUint::from(5u64),
                split_royalties: split,
                transfer_percentage: transfer_percentage,
            },
        );

        Ok(())
    }

    #[endpoint]
    fn update_uris(&self, token: TokenIdentifier) -> SCResult<()> {
        let mut uris = ManagedVec::<Self::Api, URI<Self::Api>>::new();
        uris.push(URI {
            key: ManagedBuffer::from("Google"),
            value: ManagedBuffer::from("https://www.google.com"),
        });

        uris.push(URI {
            key: ManagedBuffer::from("Nike Shop"),
            value: ManagedBuffer::from("https://www.nike.com.br/snkrs"),
        });

        self.send().kda_update_uris(&token, &uris);
        Ok(())
    }

    #[payable("*")]
    #[endpoint]
    fn freeze(&self, token: TokenIdentifier, amount: BigUint) -> SCResult<()> {
        self.send().freeze(&token, &amount);
        Ok(())
    }

    #[endpoint]
    fn unfreeze(&self, token: TokenIdentifier, bucket_id: ManagedBuffer) -> SCResult<()> {
        self.send().unfreeze(&token, &bucket_id);
        Ok(())
    }

    #[payable("*")]
    #[endpoint]
    fn delegate(&self, address: ManagedAddress, bucket_id: ManagedBuffer) -> SCResult<()> {
        self.send().delegate(&address, &bucket_id);
        Ok(())
    }

    #[endpoint]
    fn undelegate(&self, bucket_id: ManagedBuffer) -> SCResult<()> {
        self.send().undelegate(&bucket_id);
        Ok(())
    }

    #[endpoint]
    fn claim(&self, claim_type: ClaimType, id: ManagedBuffer) -> SCResult<()> {
        self.send().kda_claim(claim_type, &id);
        Ok(())
    }

    #[endpoint]
    fn withdraw(
        &self,
        withdraw_type: WithdrawType,
        token: TokenIdentifier,
        amount: BigUint,
        currency: TokenIdentifier,
    ) -> SCResult<()> {
        self.send()
            .withdraw(withdraw_type, &token, &amount, &currency);
        Ok(())
    }

    #[payable("*")]
    #[endpoint]
    fn sell(
        &self,
        sell_type: SellType,
        marketplace_id: ManagedBuffer,
        nft_id: TokenIdentifier,
        nft_nonce: u64,
        currency: TokenIdentifier,
        price: BigUint,
        reserve_price: BigUint,
        end_time: u64,
    ) -> SCResult<()> {
        self.send().sell(
            sell_type,
            &marketplace_id,
            &nft_id,
            nft_nonce,
            &currency,
            &price,
            &reserve_price,
            end_time,
        );
        Ok(())
    }

    #[payable("*")]
    #[endpoint]
    fn buy(
        &self,
        buy_type: BuyType,
        id: ManagedBuffer,
        currency: TokenIdentifier,
        amount: BigUint,
    ) -> SCResult<()> {
        self.send().buy(buy_type, &id, &currency, &amount);
        Ok(())
    }

    #[payable("*")]
    #[endpoint]
    fn deposit(
        &self,
        deposit_type: DepositType,
        id: ManagedBuffer,
        currency: TokenIdentifier,
        amount: BigUint,
    ) -> SCResult<()> {
        self.send().deposit(deposit_type, &id, &currency, &amount);
        Ok(())
    }

    #[endpoint]
    fn vote(&self, proposal_id: u64, vote_type: VoteType, amount: BigUint) -> SCResult<()> {
        self.send().vote(proposal_id, vote_type, &amount);
        Ok(())
    }

    #[endpoint]
    fn ito_config(
        &self,
        token: TokenIdentifier,
        receiver: ManagedAddress,
        wl_address: ManagedAddress,
        start_time: u64,
        end_time: u64,
        wl_start_time: u64,
        wl_end_time: u64,
    ) -> SCResult<()> {
        let mut split = ManagedVec::<Self::Api, ITOWhitelist<Self::Api>>::new();
        split.push(ITOWhitelist {
            address: wl_address,
            limit: BigUint::from(20u64),
        });

        let mut packs = ManagedVec::<Self::Api, ITOPackInfo<Self::Api>>::new();
        let mut p = ManagedVec::<Self::Api, ITOPackItem<Self::Api>>::new();
        p.push(ITOPackItem {
            amount: BigUint::from(10u64),
            price: BigUint::from(5u64),
        });

        packs.push(ITOPackInfo {
            token: TokenIdentifier::from("KLV"),
            packs: p,
        });

        let _ = self.send().ito_config(
            &token,
            &receiver,
            ITOStatus::Active,
            &BigUint::from(10000u64),
            &BigUint::from(100u64),
            start_time,
            end_time,
            ITOWhitelistStatus::Active,
            wl_start_time,
            wl_end_time,
            &split,
            &packs,
        );
        Ok(())
    }

    #[endpoint]
    fn ito_set_prices(&self, token: TokenIdentifier) -> SCResult<()> {
        let mut packs = ManagedVec::<Self::Api, ITOPackInfo<Self::Api>>::new();
        let mut p = ManagedVec::<Self::Api, ITOPackItem<Self::Api>>::new();
        p.push(ITOPackItem {
            amount: BigUint::from(10u64),
            price: BigUint::from(500u64),
        });

        packs.push(ITOPackInfo {
            token: TokenIdentifier::from("KLV"),
            packs: p,
        });

        let _ = self.send().ito_set_prices(&token, &packs);
        Ok(())
    }

    #[endpoint]
    fn ito_update_status(&self, token: TokenIdentifier, status: ITOStatus) -> SCResult<()> {
        let _ = self.send().ito_update_status(&token, status);
        Ok(())
    }

    #[endpoint]
    fn ito_update_receiver_address(
        &self,
        token: TokenIdentifier,
        receiver: ManagedAddress,
    ) -> SCResult<()> {
        let _ = self.send().ito_update_receiver_address(&token, &receiver);
        Ok(())
    }

    #[endpoint]
    fn ito_update_max_amount(&self, token: TokenIdentifier, max_amount: BigUint) -> SCResult<()> {
        let _ = self.send().ito_update_max_amount(&token, &max_amount);
        Ok(())
    }

    #[endpoint]
    fn ito_update_default_limit_per_address(
        &self,
        token: TokenIdentifier,
        default_limit_per_address: BigUint,
    ) -> SCResult<()> {
        let _ = self
            .send()
            .ito_update_default_limit_per_address(&token, &default_limit_per_address);
        Ok(())
    }

    #[endpoint]
    fn ito_update_times(
        &self,
        token: TokenIdentifier,
        start_time: u64,
        end_time: u64,
    ) -> SCResult<()> {
        let _ = self.send().ito_update_times(&token, start_time, end_time);
        Ok(())
    }

    #[endpoint]
    fn ito_update_whitelist_status(
        &self,
        token: TokenIdentifier,
        whitelist_status: ITOWhitelistStatus,
    ) -> SCResult<()> {
        let _ = self
            .send()
            .ito_update_whitelist_status(&token, whitelist_status);
        Ok(())
    }

    #[endpoint]
    fn ito_add_to_whitelist(
        &self,
        token: TokenIdentifier,
        wl_address: ManagedAddress,
    ) -> SCResult<()> {
        let mut split = ManagedVec::<Self::Api, ITOWhitelist<Self::Api>>::new();
        split.push(ITOWhitelist {
            address: wl_address,
            limit: BigUint::from(20u64),
        });

        let _ = self.send().ito_add_to_whitelist(&token, &split);
        Ok(())
    }

    #[endpoint]
    fn ito_remove_from_whitelist(
        &self,
        token: TokenIdentifier,
        wl_address: ManagedAddress,
    ) -> SCResult<()> {
        let mut split = ManagedVec::<Self::Api, ITOWhitelist<Self::Api>>::new();
        split.push(ITOWhitelist {
            address: wl_address,
            limit: BigUint::default(),
        });

        let _ = self.send().ito_remove_from_whitelist(&token, &split);
        Ok(())
    }

    #[endpoint]
    fn ito_update_whitelist_times(
        &self,
        token: TokenIdentifier,
        whitelist_start_time: u64,
        whitelist_end_time: u64,
    ) -> SCResult<()> {
        let _ = self.send().ito_update_whitelist_times(
            &token,
            whitelist_start_time,
            whitelist_end_time,
        );
        Ok(())
    }

    #[endpoint]
    fn set_account_name(&self, name: ManagedBuffer) -> SCResult<()> {
        let _ = self.send().set_account_name(name);
        Ok(())
    }
}
