use std::collections::HashMap;

use crate::data::{
    account::{Account, AccountResponse},
    account_storage::AccountStorageResponse,
    address::Address,
    kda::{KdaBalance, KdaBalanceResponse, KdaRolesResponse},
    transaction::{
        ResponseTxCost, SendTransactionResponse, Transaction, TransactionInfo,
        TransactionOnNetwork, TransactionStatus, TxCostResponseData,
    },
    vm::{ResponseVmValue, VmValueRequest, VmValuesResponseData},
};
use anyhow::{anyhow, Result};
use reqwest::Client;

pub const MAINNET_GATEWAY: &str = "https://api.mainnet.klever.finance";
pub const TESTNET_GATEWAY: &str = "https://api.testnet.klever.finance";
pub const DEVNET_GATEWAY: &str = "https://api.devnet.klever.finance";

// MetachainShardId will be used to identify a shard ID as metachain
pub const METACHAIN_SHARD_ID: u32 = 0xFFFFFFFF;

const ACCOUNT_ENDPOINT: &str = "address/";
const KEYS_ENDPOINT: &str = "/keys/";
const COST_TRANSACTION_ENDPOINT: &str = "transaction/cost";
const SEND_TRANSACTION_ENDPOINT: &str = "transaction/send";
const GET_TRANSACTION_INFO_ENDPOINT: &str = "transaction/";
const WITH_RESULTS_QUERY_PARAM: &str = "?withResults=true";
const VM_VALUES_ENDPOINT: &str = "vm-values/query";

#[derive(Clone, Debug)]
pub struct CommunicationProxy {
    proxy_url: String,
    client: Client,
}

impl CommunicationProxy {
    pub fn new(proxy_url: String) -> Self {
        Self {
            proxy_url,
            client: Client::new(),
        }
    }

    fn get_endpoint(&self, endpoint: &str) -> String {
        format!("{}/{}", self.proxy_url, endpoint)
    }
    // request_transaction_cost retrieves how many gas a transaction will consume
    pub async fn request_transaction_cost(&self, tx: &Transaction) -> Result<TxCostResponseData> {
        let endpoint = self.get_endpoint(COST_TRANSACTION_ENDPOINT);
        let resp = self
            .client
            .post(endpoint)
            .json(tx)
            .send()
            .await?
            .json::<ResponseTxCost>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b),
        }
    }

    // get_account retrieves an account info from the network (nonce, balance)
    pub async fn get_account(&self, address: &Address) -> Result<Account> {
        if !address.is_valid() {
            return Err(anyhow!("invalid address"));
        }

        let endpoint = ACCOUNT_ENDPOINT.to_string() + address.to_string().as_str();
        let endpoint = self.get_endpoint(endpoint.as_str());
        let resp = self
            .client
            .get(endpoint)
            .send()
            .await?
            .json::<AccountResponse>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.account),
        }
    }

    // get_account_kda_roles retrieves an all kda roles of an account from the network
    pub async fn get_account_kda_roles(
        &self,
        address: &Address,
    ) -> Result<HashMap<String, Vec<String>>> {
        if !address.is_valid() {
            return Err(anyhow!("invalid address"));
        }

        let endpoint = ACCOUNT_ENDPOINT.to_string() + address.to_string().as_str() + "/kdas/roles";
        let endpoint = self.get_endpoint(endpoint.as_str());
        let resp = self
            .client
            .get(endpoint)
            .send()
            .await?
            .json::<KdaRolesResponse>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.roles),
        }
    }

    // get_account_kda_tokens retrieves an all kda token of an account from the network
    pub async fn get_account_kda_tokens(
        &self,
        address: &Address,
    ) -> Result<HashMap<String, KdaBalance>> {
        if !address.is_valid() {
            return Err(anyhow!("invalid address"));
        }

        let endpoint = ACCOUNT_ENDPOINT.to_string() + address.to_string().as_str() + "/kda";
        let endpoint = self.get_endpoint(endpoint.as_str());
        let resp = self
            .client
            .get(endpoint)
            .send()
            .await?
            .json::<KdaBalanceResponse>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.kdas),
        }
    }

    // get_account_kda_tokens retrieves an all kda token of an account from the network
    pub async fn get_account_storage_keys(
        &self,
        address: &Address,
    ) -> Result<HashMap<String, String>> {
        if !address.is_valid() {
            return Err(anyhow!("invalid address"));
        }

        let endpoint = ACCOUNT_ENDPOINT.to_string() + address.to_string().as_str() + KEYS_ENDPOINT;
        let endpoint = self.get_endpoint(endpoint.as_str());
        let resp = self
            .client
            .get(endpoint)
            .send()
            .await?
            .json::<AccountStorageResponse>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.pairs),
        }
    }

    async fn get_transaction_info_internal(
        &self,
        hash: &str,
        with_results: bool,
    ) -> Result<TransactionOnNetwork> {
        let mut endpoint = GET_TRANSACTION_INFO_ENDPOINT.to_string() + hash;

        if with_results {
            endpoint += WITH_RESULTS_QUERY_PARAM
        }

        let endpoint = self.get_endpoint(endpoint.as_str());
        let resp = self
            .client
            .get(endpoint)
            .send()
            .await?
            .json::<TransactionInfo>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.transaction),
        }
    }

    // get_transaction_info retrieves a transaction's details from the network
    pub async fn get_transaction_info(&self, hash: &str) -> Result<TransactionOnNetwork> {
        self.get_transaction_info_internal(hash, false).await
    }

    // get_transaction_info_with_results retrieves a transaction's details from the network with events
    pub async fn get_transaction_info_with_results(
        &self,
        hash: &str,
    ) -> Result<TransactionOnNetwork> {
        self.get_transaction_info_internal(hash, true).await
    }

    // get_transaction_status retrieves a transaction's status from the network
    pub async fn get_transaction_status(&self, hash: &str) -> Result<String> {
        let endpoint = format!("transaction/{hash}/status");
        let endpoint = self.get_endpoint(endpoint.as_str());

        let resp = self
            .client
            .get(endpoint)
            .send()
            .await?
            .json::<TransactionStatus>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.status),
        }
    }

    pub async fn send_transaction(&self, tx: &Transaction) -> Result<String> {
        let endpoint = self.get_endpoint(SEND_TRANSACTION_ENDPOINT);
        let resp = self
            .client
            .post(endpoint)
            .json(tx)
            .send()
            .await?
            .json::<SendTransactionResponse>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.tx_hash),
        }
    }

    // execute_vmquery retrieves data from existing SC trie through the use of a VM
    pub async fn execute_vmquery(
        &self,
        vm_request: &VmValueRequest,
    ) -> Result<VmValuesResponseData> {
        let endpoint = self.get_endpoint(VM_VALUES_ENDPOINT);
        let resp = self
            .client
            .post(endpoint)
            .json(vm_request)
            .send()
            .await?
            .json::<ResponseVmValue>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b),
        }
    }
}
