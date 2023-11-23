use crate::*;
use near_workspaces::AccountId;

pub type ConsumerChainId = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct CrossChainTransferInfo {
    pub channel: AccountId,
    pub token: AccountId,
}
