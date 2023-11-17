use itertools::Itertools;

use crate::{
    near::contracts::{
        appchain_anchor_ibc::AppchainAnchorIbc,
        appchain_registry::{AppchainRegistry, AppchainState, AppchainType},
        AppchainAnchorIbcContract,
    },
    *,
};

// use crate::{near::contracts::{appchain_anchor_ibc::{AppchainAnchorIbc, self}, AppchainAnchorIbcContract}, global::SIGNER};

pub async fn fetch_validator_set_from_restaking_base_and_send_vsc_packet_to_appchain_in_anchors(
) -> anyhow::Result<()> {
    let appchain_anchor_ibc_list = get_all_anchor_ibc().await?;
    let signer = SIGNER.get().ok_or(anyhow!("Failed to get signer"))?;
    for appchain_anchor_ibc in appchain_anchor_ibc_list {
        appchain_anchor_ibc
            .fetch_validator_set_from_restaking_base(signer)
            .await?
            .into_result()?;
        appchain_anchor_ibc
            .send_vsc_packet_to_appchain(signer)
            .await?
            .into_result()?;
    }
    Ok(())
}

async fn get_all_anchor_ibc() -> anyhow::Result<Vec<AppchainAnchorIbcContract>> {
    let signer = SIGNER.get().ok_or(anyhow!("Failed to get signer"))?;
    let registry = APPCHAIN_REGISTRY_CONTRACT
        .get()
        .ok_or(anyhow!("Failed to get signer"))?;
    let appchain_ids = registry.get_appchain_ids(signer).await?;

    let mut active_cosmos_appchain_ids: Vec<AccountId> = vec![];
    for appchain_id in appchain_ids {
        let status = registry
            .get_appchain_status_of(signer, appchain_id.clone())
            .await?;

        if matches!(status.appchain_state, AppchainState::Active)
            && matches!(status.appchain_metadata.appchain_type, AppchainType::Cosmos)
        {
            active_cosmos_appchain_ids
                .push(format!("{}.{}", appchain_id, registry.contract_id).parse()?);
        }
    }

    Ok(active_cosmos_appchain_ids
        .into_iter()
        .map(|id| AppchainAnchorIbcContract { contract_id: id })
        .collect_vec())
}
