use crate::global::LPOS_MARKET;
use crate::types::ValidatorStatus;
use crate::*;

pub async fn ping_every_validators() -> anyhow::Result<()> {
    let lpos_market = LPOS_MARKET
        .get()
        .ok_or_else(|| anyhow!("Failed to get LPOS_MARKET."))?;

    let restaking_base = RESTAKING_BASE
        .get()
        .ok_or_else(|| anyhow!("Failed to get LPOS_MARKET."))?;

    let signer = SIGNER
        .get()
        .ok_or_else(|| anyhow!("Failed to get SIGNER."))?;

    let current_epoch = restaking_base.get_current_epoch_height(signer).await?;
    let validator_infos = lpos_market.get_validators(signer).await?;
    for validator_info in validator_infos {
        if matches!(validator_info.status, ValidatorStatus::Staking)
            && validator_info.last_ping_epoch < current_epoch
        {
            lpos_market
                .ping(signer, validator_info.validator_id)
                .await?
                .into_result()?;
        }
    }

    Ok(())
}
