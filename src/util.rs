use crate::cmd_args::NearNetwork;
use std::path::PathBuf;

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
