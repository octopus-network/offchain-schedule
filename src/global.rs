use dotenv::dotenv;
use itertools::Itertools;
use std::env;

use anyhow::{anyhow, Ok};
use ic_agent::export::Principal;
use ic_agent::identity::Secp256k1Identity;
use near_workspaces::{
    network::{Mainnet, Testnet},
    Account, AccountId, Worker,
};
use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;

use crate::{
    cmd_args::NearNetwork,
    near::contracts::{
        lpos_market::LposMarket, restaking_base::RestakingBase, AppchainRegistryContract,
    },
    types::CanisterInfo,
};

pub static NEAR_MAINNET_WORKER: OnceCell<Worker<Mainnet>> = OnceCell::const_new();
pub static NEAR_TESTNET_WORKER: OnceCell<Worker<Testnet>> = OnceCell::const_new();
pub static SIGNER: OnceCell<Account> = OnceCell::const_new();
pub static LPOS_MARKET: OnceCell<LposMarket> = OnceCell::const_new();
pub static RESTAKING_BASE: OnceCell<RestakingBase> = OnceCell::const_new();
pub static APPCHAIN_REGISTRY_CONTRACT: OnceCell<AppchainRegistryContract> = OnceCell::const_new();
pub static SYS_ENV: OnceCell<SystemEnv> = OnceCell::const_new();

// staker list that contain un-batched pending withdrawals
pub static STAKER_LIST_CONTAIN_UNBATCHED_PENDING_WITHDRAWAL: OnceCell<Vec<AccountId>> =
    OnceCell::const_new();

#[derive(Debug)]
pub struct SystemEnv {
    pub(crate) near_env: NearNetwork,
    pub(crate) near_cli_testnet_rpc_server_url: Option<String>,
    pub(crate) near_cli_mainnet_rpc_server_url: Option<String>,
    pub(crate) schedule_signer: AccountId,
    pub(crate) schedule_signer_secret_key: String,
    pub(crate) lpos_market_contract: String,
    pub(crate) restaking_base_contract: String,
    pub(crate) appchain_registry_contract: AccountId,
    pub(crate) active_ibc_anchor_id_list: Vec<AccountId>,
    pub(crate) canister_info_list: Vec<CanisterInfo>,
    pub(crate) near_account_id_list: Vec<AccountId>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CanisterPair {
    pub identity_pem: String,
    pub canister_id: String,
}

pub async fn init_env_config() -> anyhow::Result<()> {
    dotenv().ok();

    let canister_pairs: Vec<CanisterPair> = serde_json::from_str(
        &env::var("CANISTER_INFO_LIST")
            .map_err(|_| anyhow!("CANISTER_INFO_LIST environment variable not found"))?,
    )?;

    let sys_env = SystemEnv {
        near_env: env::var("NEAR_ENV")
            .map_err(|_| anyhow!("NEAR_ENV environment variable not found"))?
            .parse::<NearNetwork>()
            .map_err(|_| anyhow!("Parse near network error."))?,
        near_cli_testnet_rpc_server_url: env::var("NEAR_CLI_TESTNET_RPC_SERVER_URL").ok(),
        near_cli_mainnet_rpc_server_url: env::var("NEAR_CLI_MAINNET_RPC_SERVER_URL").ok(),
        schedule_signer: env::var("SCHEDULE_SIGNER")
            .map_err(|_| anyhow!("SCHEDULE_SIGNER environment variable not found"))?
            .parse()?,
        schedule_signer_secret_key: env::var("SCHEDULE_SIGNER_SECRET_KEY")
            .map_err(|_| anyhow!("SCHEDULE_SIGNER_SECRET_KEY environment variable not found"))?,
        lpos_market_contract: env::var("LPOS_MARKET_CONTRACT")
            .map_err(|_| anyhow!("LPOS_MARKET_CONTRACT environment variable not found"))?,
        restaking_base_contract: env::var("RESTAKING_BASE_CONTRACT")
            .map_err(|_| anyhow!("RESTAKING_BASE_CONTRACT environment variable not found"))?,
        appchain_registry_contract: env::var("APPCHAIN_REGISTRY_CONTRACT")
            .map_err(|_| anyhow!("APPCHAIN_REGISTRY_CONTRACT environment variable not found"))?
            .parse()?,
        active_ibc_anchor_id_list: serde_json::from_str(
            &env::var("ACTIVE_IBC_ANCHOR_ID_LIST")
                .map_err(|_| anyhow!("ACTIVE_IBC_ANCHOR_ID_LIST environment variable not found"))?,
        )?,
        canister_info_list: canister_pairs
            .iter()
            .map(|pair| CanisterInfo {
                agent_identity: Secp256k1Identity::from_pem(pair.identity_pem.as_bytes())
                    .expect("Cannot create secp256k1 identity from PEM file."),
                canister_id: Principal::from_text(pair.canister_id.clone())
                    .expect("Cannot create canister_id"),
            })
            .collect(),
        near_account_id_list: serde_json::from_str(
            &env::var("NEAR_ACCOUNT_ID_LIST")
                .map_err(|_| anyhow!("NEAR_ACCOUNT_ID_LIST environment variable not found"))?,
        )?,
    };

    match &sys_env.near_env {
        NearNetwork::Mainnet => {
            let worker = near_workspaces::mainnet()
                .rpc_addr(
                    &sys_env
                        .near_cli_mainnet_rpc_server_url
                        .clone()
                        .unwrap_or("https://rpc.mainnet.near.org".to_string()),
                )
                .await?;
            NEAR_MAINNET_WORKER.set(worker)?;
            let signer = Account::from_secret_key(
                sys_env.schedule_signer.clone(),
                sys_env.schedule_signer_secret_key.parse()?,
                NEAR_MAINNET_WORKER.get().unwrap(),
            );
            SIGNER.set(signer)?;

            STAKER_LIST_CONTAIN_UNBATCHED_PENDING_WITHDRAWAL.set(
                vec![
                    "1.v1.lpos-market.near",
                    "3.v1.lpos-market.near",
                    "4.v1.lpos-market.near",
                    "8.v1.lpos-market.near",
                    "9.v1.lpos-market.near",
                ]
                .iter()
                .map(|e| e.parse().unwrap())
                .collect_vec(),
            )?;
        }
        NearNetwork::Testnet => {
            let worker = near_workspaces::testnet()
                .rpc_addr(
                    &sys_env
                        .near_cli_testnet_rpc_server_url
                        .clone()
                        .unwrap_or("https://rpc.testnet.near.org".to_string()),
                )
                .await?;
            NEAR_TESTNET_WORKER.set(worker)?;
            let signer = Account::from_secret_key(
                sys_env.schedule_signer.clone(),
                sys_env.schedule_signer_secret_key.parse()?,
                NEAR_TESTNET_WORKER.get().unwrap(),
            );
            SIGNER.set(signer)?;
            STAKER_LIST_CONTAIN_UNBATCHED_PENDING_WITHDRAWAL.set(vec![])?;
        }
    }

    LPOS_MARKET.set(LposMarket::new(sys_env.lpos_market_contract.parse()?))?;
    RESTAKING_BASE.set(RestakingBase::new(sys_env.restaking_base_contract.parse()?))?;
    APPCHAIN_REGISTRY_CONTRACT.set(AppchainRegistryContract {
        contract_id: sys_env.appchain_registry_contract.parse()?,
    })?;

    SYS_ENV.set(sys_env)?;

    Ok(())
}
