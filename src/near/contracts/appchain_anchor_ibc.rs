use crate::*;

use super::NearContract;

#[async_trait]
pub trait AppchainAnchorIbc: NearContract {
    async fn fetch_validator_set_from_restaking_base(
        &self,
        signer: &Account,
    ) -> anyhow::Result<ExecutionFinalResult> {
        signer
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
            })
    }

    async fn send_vsc_packet_to_appchain(
        &self,
        signer: &Account,
    ) -> anyhow::Result<ExecutionFinalResult> {
        signer
            .call(self.get_contract_id(), "send_vsc_packet_to_appchain")
            .max_gas()
            .transact()
            .await
            .map_err(|e| anyhow!("Failed to send_vsc_packet_to_appchain, error: {:?}", e))
    }
}
