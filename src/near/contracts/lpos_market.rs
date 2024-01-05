use anyhow::{anyhow, Ok};
use near_workspaces::{result::ExecutionFinalResult, Account, AccountId};
use serde_json::json;

use crate::types::{ValidatorDetail, ValidatorInfo};

#[derive(Debug)]
pub struct LposMarket {
    pub contract_id: AccountId,
}

impl LposMarket {
    pub fn new(contract_id: AccountId) -> Self {
        Self { contract_id }
    }

    pub async fn clean_validator_state(
        &self,
        signer: &Account,
        validator_id: AccountId,
    ) -> anyhow::Result<ExecutionFinalResult> {
        signer
            .call(&self.contract_id, "clean_validator_state")
            .args_json(json!({
                "validator_id": validator_id
            }))
            .max_gas()
            .transact()
            .await
            .map_err(|e| anyhow!("Failed to clean_validator_state, error: {:?}", e))
    }

    pub async fn withdraw_unstake(
        &self,
        signer: &Account,
        validator_id: AccountId,
    ) -> anyhow::Result<ExecutionFinalResult> {
        signer
            .call(&self.contract_id, "withdraw_unstake")
            .args_json(json!({
                "validator_id": validator_id
            }))
            .max_gas()
            .transact()
            .await
            .map_err(|e| anyhow!("Failed to withdraw_unstake, error: {:?}", e))
    }

    pub async fn get_validator_detail(
        &self,
        signer: &Account,
        validator_id: AccountId,
    ) -> anyhow::Result<Option<ValidatorDetail>> {
        let result = signer
            .view(&self.contract_id, "get_validator_detail")
            .args_json(json!({
                "validator_id": validator_id
            }))
            .await?
            .json()?;
        Ok(result)
    }

    pub async fn undelegate_in_unstake(
        &self,
        signer: &Account,
        delegator_id: AccountId,
    ) -> anyhow::Result<ExecutionFinalResult> {
        signer
            .call(&self.contract_id, "undelegate_in_unstake")
            .args_json(json!({
                "delegator_id": delegator_id
            }))
            .max_gas()
            .transact()
            .await
            .map_err(|e| anyhow!("Failed to undelegate_in_unstake, error: {:?}", e))
    }

    pub async fn claim_reward_in_validator(
        &self,
        signer: &Account,
        account_id: AccountId,
        reward_token_id: AccountId,
    ) -> anyhow::Result<ExecutionFinalResult> {
        signer
            .call(&self.contract_id, "claim_reward_in_validator")
            .args_json(json!({
                "account_id": account_id,
                "reward_token_id": reward_token_id
            }))
            .max_gas()
            .transact()
            .await
            .map_err(|e| anyhow!("Failed to claim_reward_in_validator, error: {:?}", e))
    }

    pub async fn destroy(
        &self,
        signer: &Account,
        validator_id: AccountId,
    ) -> anyhow::Result<ExecutionFinalResult> {
        signer
            .call(&self.contract_id, "destroy")
            .args_json(json!({
                "validator_id": validator_id
            }))
            .max_gas()
            .transact()
            .await
            .map_err(|e| anyhow!("Failed to destroy, error: {:?}", e))
    }

    pub async fn distribute_latest_reward(
        &self,
        signer: &Account,
    ) -> anyhow::Result<ExecutionFinalResult> {
        signer
            .call(&self.contract_id, "distribute_latest_reward")
            .max_gas()
            .transact()
            .await
            .map_err(|e| anyhow!("Failed to distribute_latest_reward, error: {:?}", e))
    }

    pub async fn distribute_latest_reward_in_validator(
        &self,
        signer: &Account,
        validator_id: AccountId,
    ) -> anyhow::Result<ExecutionFinalResult> {
        signer
            .call(&self.contract_id, "distribute_latest_reward_in_validator")
            .max_gas()
            .args_json(json!({ "validator_id": validator_id }))
            .transact()
            .await
            .map_err(|e| {
                anyhow!(
                    "Failed to distribute_latest_reward_in_validator, error: {:?}",
                    e
                )
            })
    }

    pub async fn get_undistributed_rewards_count(&self, signer: &Account) -> anyhow::Result<u32> {
        let result: u32 = signer
            .view(&self.contract_id, "get_undistributed_rewards_count")
            .await?
            .json()?;

        tracing::info!("get_undistributed_rewards_count, count: {}", result);

        Ok(result)
    }

    pub async fn get_validators_undistributed_rewards(
        &self,
        signer: &Account,
    ) -> anyhow::Result<Vec<(AccountId, u32)>> {
        let result: Vec<(AccountId, u32)> = signer
            .view(&self.contract_id, "get_validators_undistributed_rewards")
            .args_json(json!({}))
            .await?
            .json()?;

        tracing::info!("get_validators_undistributed_rewards, result: {:?}", result);

        Ok(result)
    }

    pub async fn get_validators(&self, signer: &Account) -> anyhow::Result<Vec<ValidatorInfo>> {
        let result = signer
            .view(&self.contract_id, "get_validators")
            .args_json(json!({}))
            .await?
            .json()?;
        tracing::info!("get_validators, result: {:?}", result);
        Ok(result)
    }

    pub async fn ping(
        &self,
        signer: &Account,
        validator_id: AccountId,
    ) -> anyhow::Result<ExecutionFinalResult> {
        signer
            .call(&self.contract_id, "ping")
            .max_gas()
            .args_json(json!({ "validator_id": validator_id }))
            .transact()
            .await
            .map_err(|e| anyhow!("Failed to ping, error: {:?}", e))
    }
}
