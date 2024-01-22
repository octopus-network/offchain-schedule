use std::collections::HashMap;

use crate::*;
use ic_agent::export::Principal;
use ic_agent::identity::Secp256k1Identity;
use util::*;

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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StakingPoolInfo {
    pub pool_id: AccountId,
    #[serde(with = "u128_dec_format")]
    pub total_share_balance: u128,
    #[serde(with = "u128_dec_format")]
    pub total_staked_balance: u128,
    pub locked: bool,
    #[serde(with = "u64_dec_format")]
    pub unlock_epoch: u64,
    #[serde(with = "u64_dec_format")]
    pub last_unstake_epoch: u64,
    pub last_unstake_batch_id: Option<String>,
    #[serde(with = "u64_dec_format")]
    pub current_unstake_batch_id: u64,
    #[serde(with = "u128_dec_format")]
    pub batched_unstake_amount: u128,
    pub submitted_unstake_batches_count: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StakingPoolDetail {
    pub pool_id: AccountId,
    #[serde(with = "u128_dec_format")]
    pub total_share_balance: u128,
    #[serde(with = "u128_dec_format")]
    pub total_staked_balance: u128,
    pub stakers: Vec<AccountId>,
    pub locked: bool,
    // pub unlock_epoch: u64,
    // #[serde(with = "u64_dec_format")]
    // pub last_unstake_epoch: u64,
    pub last_unstake_batch_id: Option<String>,
    pub current_unstake_batch_id: String,
    #[serde(with = "u128_dec_format")]
    pub batched_unstake_amount: u128,
    // pub submitted_unstake_batches: Vec<SubmittedUnstakeBatch>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PendingWithdrawal {
    pub withdrawal_certificate: String,
    pub pool_id: AccountId,
    #[serde(with = "u128_dec_format")]
    pub amount: u128,
    #[serde(with = "u64_dec_format")]
    pub unlock_epoch: u64,
    #[serde(with = "u64_dec_format")]
    pub unlock_time: u64,
    pub beneficiary: AccountId,
    pub allow_other_withdraw: bool,
    pub unstake_batch_id: Option<String>,
}
