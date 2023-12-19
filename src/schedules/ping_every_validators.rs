use crate::global::LPOS_MARKET;
use crate::types::ValidatorStatus;
use crate::*;

pub async fn ping_every_validators() -> anyhow::Result<()> {
    info!("ping every validators");

    let lpos_market = LPOS_MARKET
        .get()
        .ok_or_else(|| anyhow!("Failed to get LPOS_MARKET."))?;

    let signer = SIGNER
        .get()
        .ok_or_else(|| anyhow!("Failed to get SIGNER."))?;

    let validator_infos = lpos_market.get_validators(signer).await?;
    for validator_info in validator_infos {
        if matches!(validator_info.status, ValidatorStatus::Staking) {
            lpos_market
                .ping(signer, validator_info.validator_id)
                .await?
                .into_result()?;
        }
    }

    Ok(())
}
