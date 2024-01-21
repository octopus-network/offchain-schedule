use crate::*;

use self::types::{PendingWithdrawal, StakingPoolDetail, StakingPoolInfo};

#[derive(Debug)]
pub struct RestakingBase {
    pub contract_id: AccountId,
}

impl RestakingBase {
    pub fn new(contract_id: AccountId) -> Self {
        Self { contract_id }
    }

    pub async fn is_withdrawable(
        &self,
        signer: &Account,
        staker_id: AccountId,
        certificate: String,
    ) -> anyhow::Result<bool> {
        let result: bool = signer
            .view(&self.contract_id, "is_withdrawable")
            .args_json(json!({
                "staker_id": staker_id,
                "certificate": certificate
            }))
            .await?
            .json()?;

        Ok(result)
    }

    pub async fn get_staking_pools(
        &self,
        signer: &Account,
    ) -> anyhow::Result<Vec<StakingPoolInfo>> {
        let result = signer
            .view(&self.contract_id, "get_staking_pools")
            .args_json(json!({}))
            .await?
            .json()?;

        Ok(result)
    }

    #[allow(unused)]
    pub async fn get_staking_pool(
        &self,
        signer: &Account,
        pool_id: AccountId,
    ) -> anyhow::Result<StakingPoolDetail> {
        let result = signer
            .view(&self.contract_id, "get_staking_pool")
            .args_json(json!({"pool_id": pool_id}))
            .await?
            .json()?;

        Ok(result)
    }

    pub async fn get_current_epoch_height(&self, signer: &Account) -> anyhow::Result<u64> {
        let result: String = signer
            .view(&self.contract_id, "get_current_epoch_height")
            .args_json(json!({}))
            .await?
            .json()?;
        let epoch_height: u64 = result.parse()?;
        Ok(epoch_height)
    }

    pub async fn get_pending_withdrawals(
        &self,
        signer: &Account,
        account_id: AccountId,
    ) -> anyhow::Result<Vec<PendingWithdrawal>> {
        let result = signer
            .view(&self.contract_id, "get_pending_withdrawals")
            .args_json(json!({"account_id": account_id}))
            .await?
            .json()?;

        Ok(result)
    }

    pub async fn withdraw_unstake_batch(
        &self,
        signer: &Account,
        pool_id: AccountId,
        unstake_batch_id: String,
    ) -> anyhow::Result<ExecutionFinalResult> {
        signer
            .call(&self.contract_id, "withdraw_unstake_batch")
            .max_gas()
            .args_json(json!({ "pool_id": pool_id, "unstake_batch_id": unstake_batch_id }))
            .transact()
            .await
            .map_err(|e| anyhow!("Failed to withdraw_unstake_batch, error: {:?}", e))
    }

    pub async fn withdraw(
        &self,
        signer: &Account,
        staker: AccountId,
        id: String,
    ) -> anyhow::Result<ExecutionFinalResult> {
        signer
            .call(&self.contract_id, "withdraw")
            .max_gas()
            .args_json(json!({ "staker": staker, "id": id}))
            .transact()
            .await
            .map_err(|e| anyhow!("Failed to withdraw, error: {:?}", e))
    }

    pub async fn submit_unstake_batch(
        &self,
        signer: &Account,
        pool_id: AccountId,
    ) -> anyhow::Result<ExecutionFinalResult> {
        signer
            .call(&self.contract_id, "submit_unstake_batch")
            .max_gas()
            .args_json(json!({ "pool_id": pool_id }))
            .transact()
            .await
            .map_err(|e| anyhow!("Failed to submit_unstake_batch, error: {:?}", e))
    }
}
