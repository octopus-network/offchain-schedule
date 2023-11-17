use dotenv::dotenv;
use std::env;

use anyhow::{anyhow, Ok};
use clap::{FromArgMatches, Parser};
use near_workspaces::{
    network::{Mainnet, Testnet},
    Account, AccountId, Worker,
};
use tokio::sync::OnceCell;
use tracing::info;

use crate::{
    cmd_args::{CmdArgs, NearNetwork},
    near::contracts::{lpos_market::LposMarket, nep141::Nep141, AppchainRegistryContract},
    util::get_near_account_dir_path,
};

// pub static CMD_ARG: OnceCell<CmdArgs> = OnceCell::const_new();
pub static NEAR_MAINNET_WORKER: OnceCell<Worker<Mainnet>> = OnceCell::const_new();
pub static NEAR_TESTNET_WORKER: OnceCell<Worker<Testnet>> = OnceCell::const_new();
pub static SIGNER: OnceCell<Account> = OnceCell::const_new();
pub static LPOS_MARKET: OnceCell<LposMarket> = OnceCell::const_new();
pub static OTTO_TOKEN: OnceCell<Nep141> = OnceCell::const_new();
pub static APPCHAIN_REGISTRY_CONTRACT: OnceCell<AppchainRegistryContract> = OnceCell::const_new();

// pub async fn init() -> anyhow::Result<()> {
//     let cmd_ars = CmdArgs::parse();
//     let rpc_address = cmd_ars.rpc.clone();
//     match cmd_ars.network {
//         NearNetwork::Mainnet => {
//             let worker = near_workspaces::mainnet().rpc_addr(&rpc_address).await?;
//             NEAR_MAINNET_WORKER.set(worker)?;
//             let account = Account::from_file(
//                 get_near_account_dir_path(&cmd_ars.signer, &cmd_ars.network.clone()),
//                 NEAR_MAINNET_WORKER.get().unwrap(),
//             )?;
//             SIGNER.set(account)?;
//         }
//         NearNetwork::Testnet => {
//             let worker = near_workspaces::testnet().rpc_addr(&rpc_address).await?;
//             NEAR_TESTNET_WORKER.set(worker)?;
//             let account = Account::from_file(
//                 get_near_account_dir_path(&cmd_ars.signer, &cmd_ars.network),
//                 NEAR_TESTNET_WORKER.get().unwrap(),
//             )?;
//             SIGNER.set(account)?;
//         }
//     }
//     LPOS_MARKET.set(LposMarket::new_from_network(&cmd_ars.network))?;
//     CMD_ARG.set(cmd_ars)?;

//     Ok(())
// }

pub static SYS_ENV: OnceCell<SystemEnv> = OnceCell::const_new();

#[derive(Debug)]
pub struct SystemEnv {
    pub(crate) near_env: NearNetwork,
    pub(crate) near_cli_testnet_rpc_server_url: Option<String>,
    pub(crate) near_cli_mainnet_rpc_server_url: Option<String>,
    pub(crate) schedule_signer: String,
    pub(crate) lpos_market_contract: String,
    pub(crate) otto_token_contract: String,
    pub(crate) appchain_registry_contract: String,
    pub(crate) cross_chain_transfer_receiver: String,
    pub(crate) dst_chain_transfer_receiver: String,
}

pub async fn init_env_config() -> anyhow::Result<()> {
    dotenv().ok();
    let sys_env = SystemEnv {
        near_env: env::var("NEAR_ENV")
            .map_err(|e| anyhow!("NEAR_ENV environment variable not found"))?
            .parse::<NearNetwork>()
            .map_err(|e| anyhow!("Parse near network error."))?,
        near_cli_testnet_rpc_server_url: env::var("NEAR_CLI_TESTNET_RPC_SERVER_URL").ok(),
        near_cli_mainnet_rpc_server_url: env::var("NEAR_CLI_MAINNET_RPC_SERVER_URL").ok(),
        schedule_signer: env::var("SCHEDULE_SIGNER")
            .map_err(|e| anyhow!("SCHEDULE_SIGNER environment variable not found"))?,
        lpos_market_contract: env::var("LPOS_MARKET_CONTRACT")
            .map_err(|e| anyhow!("LPOS_MARKET_CONTRACT environment variable not found"))?,
        otto_token_contract: env::var("OTTO_TOKEN_CONTRACT")
            .map_err(|e| anyhow!("OTTO_TOKEN_CONTRACT environment variable not found"))?,
        appchain_registry_contract: env::var("APPCHAIN_REGISTRY_CONTRACT")
            .map_err(|e| anyhow!("APPCHAIN_REGISTRY_CONTRACT environment variable not found"))?,
        cross_chain_transfer_receiver: env::var("CROSS_CHAIN_TRANSFER_RECEIVER")
            .map_err(|e| anyhow!("CROSS_CHAIN_TRANSFER_RECEIVER environment variable not found"))?,
        dst_chain_transfer_receiver: env::var("DST_CHAIN_TRANSFER_RECEIVER")
            .map_err(|e| anyhow!("DST_CHAIN_TRANSFER_RECEIVER environment variable not found"))?,
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
            let signer = Account::from_file(
                get_near_account_dir_path(&sys_env.schedule_signer, &sys_env.near_env),
                NEAR_MAINNET_WORKER.get().unwrap(),
            )?;
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
            let signer = Account::from_file(
                get_near_account_dir_path(&sys_env.schedule_signer, &sys_env.near_env),
                NEAR_TESTNET_WORKER.get().unwrap(),
            )?;
            SIGNER.set(signer)?;
        }
    }

    LPOS_MARKET.set(LposMarket::new(sys_env.lpos_market_contract.parse()?))?;
    OTTO_TOKEN.set(Nep141::new(sys_env.otto_token_contract.parse()?))?;
    APPCHAIN_REGISTRY_CONTRACT.set(AppchainRegistryContract {
        contract_id: sys_env.appchain_registry_contract.parse()?,
    })?;

    SYS_ENV.set(sys_env)?;

    Ok(())
}
