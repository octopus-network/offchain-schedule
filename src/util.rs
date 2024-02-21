use near_workspaces::types::AccountDetails;

use crate::cmd_args::NearNetwork;
use crate::*;
use std::path::PathBuf;

pub async fn get_account_details_by_sys_env(
    account_id: &AccountId,
) -> anyhow::Result<AccountDetails> {
    let sys_env = SYS_ENV.get().unwrap();
    match sys_env.near_env {
        NearNetwork::Mainnet => {
            let worker = NEAR_MAINNET_WORKER.get().unwrap();
            worker
                .view_account(account_id)
                .await
                .map_err(|e| anyhow!("Failed to get account detail, {}", e))
        }
        NearNetwork::Testnet => {
            let worker = NEAR_TESTNET_WORKER.get().unwrap();
            worker
                .view_account(account_id)
                .await
                .map_err(|e| anyhow!("Failed to get account detail, {}", e))
        }
    }
}

#[allow(unused)]
pub fn get_near_account_dir_path(account_id: &str, network: &NearNetwork) -> PathBuf {
    let mut home_dir = dirs::home_dir().expect("Impossible to get your home dir!");
    let network_path = match network {
        NearNetwork::Mainnet => "mainnet",
        NearNetwork::Testnet => "testnet",
    };
    home_dir.push(format!(
        ".near-credentials/{}/{}.json",
        network_path, account_id
    ));
    home_dir
}

pub mod u64_dec_format {
    use serde::de;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(num: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&num.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}

pub mod u128_dec_format {
    use serde::de;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(num: &u128, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&num.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u128, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}
