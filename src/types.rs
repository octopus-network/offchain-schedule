use std::collections::HashMap;

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

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ValidatorStatus {
    Deployed,
    Staking,
    Unstaking,
    ToBeDestroyed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidatorInfo {
    pub validator_id: AccountId,
    pub escrow_id: AccountId,
    pub total_share_balance: String,
    pub total_staked_balance: String,
    pub select_staking_pool: Option<AccountId>,
    pub share_balance: String,
    pub status: ValidatorStatus,
    pub unstake_withdraw_certificate: Option<String>,
    pub is_destroyable: bool,
    pub undistributed_reward_count: u32,
}

#[derive(Serialize, Deserialize)]
pub enum MultiTxsOperationProcessingResult {
    NeedMoreGas,
    Ok,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidatorDetail {
    pub validator_id: AccountId,
    pub escrow_id: AccountId,
    pub total_share_balance: String,
    pub total_staked_balance: String,
    pub delegators: Vec<AccountId>,
    pub select_staking_pool: Option<AccountId>,
    pub share_balance: String,
    pub rewards: HashMap<AccountId, String>,
    pub validator_staked_balance: String,
    pub status: ValidatorStatus,
    pub unstake_withdraw_certificate: Option<String>,
    pub is_destroyable: bool,
}
