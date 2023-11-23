use std::str::FromStr;

use clap::{command, Parser};

#[derive(clap::ValueEnum, Debug, Clone)]
pub enum NearNetwork {
    Mainnet,
    Testnet,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseNearNetworkError;

impl FromStr for NearNetwork {
    type Err = ParseNearNetworkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mainnet" => Result::Ok(NearNetwork::Mainnet),
            "testnet" => Result::Ok(NearNetwork::Testnet),
            _ => Result::Err(ParseNearNetworkError),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CmdArgs {
    #[arg(short, long)]
    pub signer: String,

    #[arg(short, long)]
    pub rpc: String,

    #[clap(value_enum, default_value_t=NearNetwork::Testnet)]
    pub network: NearNetwork,
}

#[test]
pub fn test_network_dbg() {
    dbg!(NearNetwork::Mainnet);
}
