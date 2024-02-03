use crate::*;

use self::{appchain_anchor_ibc::AppchainAnchorIbc, appchain_registry::AppchainRegistry};

pub mod appchain_anchor_ibc;
pub mod appchain_registry;
pub mod lpos_market;
pub mod nep141;
pub mod restaking_base;
pub mod rpc;

pub trait NearContract {
    fn get_contract_id(&self) -> &AccountId;
}

#[derive(Debug)]
pub struct AppchainRegistryContract {
    pub contract_id: AccountId,
}

impl NearContract for AppchainRegistryContract {
    fn get_contract_id(&self) -> &AccountId {
        &self.contract_id
    }
}

impl AppchainRegistry for AppchainRegistryContract {}

#[derive(Debug)]
pub struct AppchainAnchorIbcContract {
    pub contract_id: AccountId,
}

impl NearContract for AppchainAnchorIbcContract {
    fn get_contract_id(&self) -> &AccountId {
        &self.contract_id
    }
}

impl AppchainAnchorIbc for AppchainAnchorIbcContract {}
