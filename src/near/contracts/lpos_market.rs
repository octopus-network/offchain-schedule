use crate::{cmd_args::NearNetwork, *};
use anyhow::{anyhow, Ok};
use near_workspaces::{result::ExecutionFinalResult, Account, AccountId};
use serde_json::json;

use crate::types::ConsumerChainId;

#[derive(Debug)]
pub struct LposMarket {
    pub contract_id: AccountId,
}

impl LposMarket {
    pub fn new(contract_id: AccountId) -> Self {
        Self { contract_id }
    }

    pub fn new_from_network(network: &NearNetwork) -> Self {
        let contract_id = match network {
            NearNetwork::Mainnet => {
                unimplemented!()
            }
            NearNetwork::Testnet => "contract-4.lpos-market.testnet".parse().unwrap(),
        };
        Self { contract_id }
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
    ) -> ExecutionFinalResult {
        signer
            .call(&self.contract_id, "distribute_latest_reward_in_validator")
            .max_gas()
            .args_json(json!({ "validator_id": validator_id }))
            .transact()
            .await
            .unwrap()
    }

    pub async fn distribute_latest_sponsor_reward_in_consumer_chain(
        &self,
        signer: &Account,
        consumer_chain_id: ConsumerChainId,
    ) -> ExecutionFinalResult {
        signer
            .call(
                &self.contract_id,
                "distribute_latest_sponsor_reward_in_consumer_chain",
            )
            .args_json(json!({ "consumer_chain_id": consumer_chain_id }))
            .max_gas()
            .transact()
            .await
            .unwrap()
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
}
