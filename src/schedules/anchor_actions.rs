use anyhow::Ok;
use itertools::Itertools;

use crate::{
    near::contracts::{appchain_anchor_ibc::AppchainAnchorIbc, AppchainAnchorIbcContract},
    *,
};

use self::types::MultiTxsOperationProcessingResult;

pub async fn process_pending_slash_in_anchor_ibc() -> anyhow::Result<()> {
    let sys_env = SYS_ENV.get().unwrap();
    let appchain_anchor_ibc_list = sys_env
        .active_ibc_anchor_id_list
        .iter()
        .map(|id| AppchainAnchorIbcContract {
            contract_id: id.clone(),
        })
        .collect_vec();

    let signer = SIGNER.get().ok_or(anyhow!("Failed to get signer"))?;
    for appchain_anchor_ibc in appchain_anchor_ibc_list {
        let result = appchain_anchor_ibc
            .get_pending_slash_packets(signer)
            .await?;
        for _ in 0..result.len() {
            appchain_anchor_ibc
                .process_first_pending_slash_packet(signer)
                .await?
                .into_result()?;
        }
    }
    Ok(())
}

pub async fn distribute_pending_rewards_in_anchor_ibc() -> anyhow::Result<()> {
    let sys_env = SYS_ENV.get().unwrap();
    let appchain_anchor_ibc_list = sys_env
        .active_ibc_anchor_id_list
        .iter()
        .map(|id| AppchainAnchorIbcContract {
            contract_id: id.clone(),
        })
        .collect_vec();

    let signer = SIGNER.get().ok_or(anyhow!("Failed to get signer"))?;
    for appchain_anchor_ibc in appchain_anchor_ibc_list {
        let mut max_limit_times = 20;
        let mut result: MultiTxsOperationProcessingResult =
            MultiTxsOperationProcessingResult::NeedMoreGas;
        while max_limit_times > 0 {
            result = appchain_anchor_ibc
                .distribute_pending_rewards(signer)
                .await?
                .into_result()?
                .json()?;

            if matches!(result, MultiTxsOperationProcessingResult::Ok) {
                break;
            }

            max_limit_times -= 1
        }
        if matches!(result, MultiTxsOperationProcessingResult::NeedMoreGas) {
            return Err(anyhow!(
                "The times of distribute_pending_rewards jobs exceed than exception"
            ));
        }
    }

    Ok(())
}

pub async fn fetch_validator_set_from_restaking_base_and_send_vsc_packet_to_appchain_in_anchors(
) -> anyhow::Result<()> {
    let sys_env = SYS_ENV.get().unwrap();
    let appchain_anchor_ibc_list = sys_env
        .active_ibc_anchor_id_list
        .iter()
        .map(|id| AppchainAnchorIbcContract {
            contract_id: id.clone(),
        })
        .collect_vec();
    // let appchain_anchor_ibc_list = get_all_anchor_ibc().await?;
    tracing::info!("All anchor ibc list: {:?}", appchain_anchor_ibc_list);
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

// async fn get_all_anchor_ibc() -> anyhow::Result<Vec<AppchainAnchorIbcContract>> {
//     let signer = SIGNER.get().ok_or(anyhow!("Failed to get signer"))?;
//     let registry = APPCHAIN_REGISTRY_CONTRACT
//         .get()
//         .ok_or(anyhow!("Failed to get signer"))?;
//     let appchain_ids = registry.get_appchain_ids(signer).await?;

//     let mut active_cosmos_appchain_ids: Vec<AccountId> = vec![];
//     for appchain_id in appchain_ids {
//         let status = registry
//             .get_appchain_status_of(signer, appchain_id.clone())
//             .await?;

//         if matches!(status.appchain_metadata.appchain_type, AppchainType::Cosmos) {
//             active_cosmos_appchain_ids
//                 .push(format!("{}.{}", appchain_id, registry.contract_id).parse()?);
//         }
//     }

//     Ok(active_cosmos_appchain_ids
//         .into_iter()
//         .map(|id| AppchainAnchorIbcContract { contract_id: id })
//         .collect_vec())
// }
