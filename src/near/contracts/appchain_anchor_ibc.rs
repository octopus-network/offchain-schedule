use crate::{types::RewardDistribution, *};

use super::NearContract;

#[async_trait]
pub trait AppchainAnchorIbc: NearContract {
    async fn distribute_pending_rewards(
        &self,
        signer: &Account,
    ) -> anyhow::Result<ExecutionFinalResult> {
        let result = signer
            .call(self.get_contract_id(), "distribute_pending_rewards")
            .max_gas()
            .transact()
            .await
            .map_err(|e| {
                anyhow!(
                    "Failed to fetch_validator_set_from_restaking_base, error: {:?}",
                    e
                )
            });

        tracing::info!("distribute_pending_rewards, result: {:?}", result);
        result
    }

    async fn get_pending_rewards(
        &self,
        signer: &Account,
    ) -> anyhow::Result<Vec<RewardDistribution>> {
        let result = signer
            .view(self.get_contract_id(), "get_pending_rewards")
            .await?
            .json()?;

        Ok(result)
    }

    async fn fetch_validator_set_from_restaking_base(
        &self,
        signer: &Account,
    ) -> anyhow::Result<ExecutionFinalResult> {
        let result = signer
            .call(
                self.get_contract_id(),
                "fetch_validator_set_from_restaking_base",
            )
            .max_gas()
            .transact()
            .await
            .map_err(|e| {
                anyhow!(
                    "Failed to fetch_validator_set_from_restaking_base, error: {:?}",
                    e
                )
            });
        tracing::info!(
            "get_consumer_chains_undistributed_rewards, result: {:?}",
            result
        );
        result
    }

    async fn send_vsc_packet_to_appchain(
        &self,
        signer: &Account,
    ) -> anyhow::Result<ExecutionFinalResult> {
        let result = signer
            .call(self.get_contract_id(), "send_vsc_packet_to_appchain")
            .max_gas()
            .transact()
            .await
            .map_err(|e| anyhow!("Failed to send_vsc_packet_to_appchain, error: {:?}", e));
        tracing::info!(
            "get_consumer_chains_undistributed_rewards, result: {:?}",
            result
        );
        result
    }
}
