use crate::*;
use ic_agent::export::Principal;
use ic_agent::identity::Secp256k1Identity;

#[allow(unused)]
pub type ConsumerChainId = String;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RewardDistribution {
    pub validator_set_id: String,
    pub amount: String,
    pub timestamp: u64,
}

#[derive(Debug)]
pub struct CanisterInfo {
    pub agent_identity: Secp256k1Identity,
    pub canister_id: Principal,
}
