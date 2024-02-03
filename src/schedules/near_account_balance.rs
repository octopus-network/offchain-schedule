use near_workspaces::AccountId;
use tracing::info;

use crate::cmd_args::NearNetwork;
use crate::global::{NEAR_MAINNET_WORKER, NEAR_TESTNET_WORKER, SYS_ENV};
use crate::near::contracts::nep141::Nep141;
use crate::*;

pub async fn check_near_account_balance() -> anyhow::Result<()> {
    let sys_env = SYS_ENV
        .get()
        .ok_or(anyhow::anyhow!("Failed to get SYS_ENV"))?;

    for account in sys_env.near_account_id_list.iter() {
        let balance = get_account_available_balance(account).await?;
        info!("{:?}: {:?}", account, balance);
        if balance < 20 {
            info!(
                "OCTOPUS_ALERT: insufficient near account balance: {:?}: {:?}",
                account, balance
            );
        }
    }

    Ok(())
}

pub async fn check_near_account_ft_balance() -> anyhow::Result<()> {
    let sys_env = SYS_ENV
        .get()
        .ok_or(anyhow::anyhow!("Failed to get SYS_ENV"))?;

    let signer = SIGNER
        .get()
        .ok_or_else(|| anyhow!("Failed to get SIGNER."))?;

    for (account_id, token_contract_id, minimum_amount_str) in
        sys_env.near_account_ft_check_list.iter()
    {
        let token_contract = Nep141::new(token_contract_id.clone());
        let token_balance: u128 = token_contract
            .ft_balance_of(signer, account_id.clone())
            .await?
            .parse()?;
        info!(
            "{:?} token balance of {:?}: {:?}",
            token_contract_id, account_id, minimum_amount_str
        );
        let minimum_amount: u128 = minimum_amount_str.parse()?;

        if token_balance < minimum_amount {
            info!(
                "OCTOPUS_ALERT:  The {:?} balance of {:?} is {:?}, less than minimum_amount {:?}",
                token_contract_id, account_id, token_balance, minimum_amount_str
            );
        }
    }

    Ok(())
}

pub const STORAGE_PRICE_PER_BYTE: u128 = 10_000_000_000_000_000_000;
pub const YOCTO_TO_NEAR: u128 = 1_000_000_000_000_000_000_000_000;

pub async fn get_account_available_balance(account_id: &AccountId) -> anyhow::Result<u128> {
    let sys_env = SYS_ENV.get().unwrap();
    let available_balance = match sys_env.near_env {
        NearNetwork::Mainnet => {
            let worker = NEAR_MAINNET_WORKER.get().unwrap();
            let account_state = worker.view_account(account_id).await?;
            account_state.balance - account_state.storage_usage as u128 * STORAGE_PRICE_PER_BYTE
        }
        NearNetwork::Testnet => {
            let worker = NEAR_TESTNET_WORKER.get().unwrap();
            let account_state = worker.view_account(account_id).await?;
            account_state.balance - account_state.storage_usage as u128 * STORAGE_PRICE_PER_BYTE
        }
    };
    Ok(available_balance / YOCTO_TO_NEAR)
}
