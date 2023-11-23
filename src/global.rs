use dotenv::dotenv;
use std::env;

use anyhow::{anyhow, Ok};
use near_workspaces::{
    network::{Mainnet, Testnet},
    Account, AccountId, Worker,
};
use tokio::sync::OnceCell;

use crate::{
    cmd_args::NearNetwork,
    near::contracts::{lpos_market::LposMarket, AppchainRegistryContract},
    types::CrossChainTransferInfo,
};

pub static NEAR_MAINNET_WORKER: OnceCell<Worker<Mainnet>> = OnceCell::const_new();
pub static NEAR_TESTNET_WORKER: OnceCell<Worker<Testnet>> = OnceCell::const_new();
pub static SIGNER: OnceCell<Account> = OnceCell::const_new();
pub static LPOS_MARKET: OnceCell<LposMarket> = OnceCell::const_new();
pub static APPCHAIN_REGISTRY_CONTRACT: OnceCell<AppchainRegistryContract> = OnceCell::const_new();
pub static SYS_ENV: OnceCell<SystemEnv> = OnceCell::const_new();

#[derive(Debug)]
pub struct SystemEnv {
    pub(crate) near_env: NearNetwork,
    pub(crate) near_cli_testnet_rpc_server_url: Option<String>,
    pub(crate) near_cli_mainnet_rpc_server_url: Option<String>,
    pub(crate) schedule_signer: AccountId,
    pub(crate) schedule_signer_secret_key: String,
    pub(crate) lpos_market_contract: String,
    pub(crate) appchain_registry_contract: AccountId,
    pub(crate) dst_chain_transfer_receiver: String,
    pub(crate) cross_chain_transfer_info_list: Vec<CrossChainTransferInfo>,
    pub(crate) active_ibc_anchor_id_list: Vec<AccountId>,
}

pub async fn init_env_config() -> anyhow::Result<()> {
    dotenv().ok();
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
        appchain_registry_contract: env::var("APPCHAIN_REGISTRY_CONTRACT")
            .map_err(|_| anyhow!("APPCHAIN_REGISTRY_CONTRACT environment variable not found"))?
            .parse()?,
        dst_chain_transfer_receiver: env::var("DST_CHAIN_TRANSFER_RECEIVER")
            .map_err(|_| anyhow!("DST_CHAIN_TRANSFER_RECEIVER environment variable not found"))?,
        cross_chain_transfer_info_list: serde_json::from_str(&env::var("CROSS_CHAIN_TRANSFER_INFO_LIST").map_err(|_| {
            anyhow!("CROSS_CHAIN_TRANSFER_INFO_LIST environment variable not found")
        })?)?,
        active_ibc_anchor_id_list: serde_json::from_str(
            &env::var("ACTIVE_IBC_ANCHOR_ID_LIST")
                .map_err(|_| anyhow!("ACTIVE_IBC_ANCHOR_ID_LIST environment variable not found"))?,
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
        }
    }

    LPOS_MARKET.set(LposMarket::new(sys_env.lpos_market_contract.parse()?))?;
    APPCHAIN_REGISTRY_CONTRACT.set(AppchainRegistryContract {
        contract_id: sys_env.appchain_registry_contract.parse()?,
    })?;

    SYS_ENV.set(sys_env)?;

    Ok(())
}
