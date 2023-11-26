use crate::*;
use near_workspaces::AccountId;

#[allow(unused)]
pub type ConsumerChainId = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct CrossChainTransferInfo {
    pub channel: AccountId,
    pub token: AccountId,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RewardDistribution {
    pub validator_set_id: String,
    pub amount: String,
    pub timestamp: String,
}
