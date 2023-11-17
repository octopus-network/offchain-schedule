use anyhow::{anyhow, Ok};
use tracing::info;

use crate::global::{LPOS_MARKET, SIGNER};

pub async fn distribute_lpos_market_reward() -> anyhow::Result<()> {
    info!("distribute_lpos_market_reward");

    let lpos_market = LPOS_MARKET
        .get()
        .ok_or_else(|| anyhow!("Failed to get LPOS_MARKET."))?;

    let signer = SIGNER
        .get()
        .ok_or_else(|| anyhow!("Failed to get SIGNER."))?;

    let count = lpos_market.get_undistributed_rewards_count(signer).await?;

    for _ in 0..count {
        lpos_market
            .distribute_latest_reward(signer)
            .await?
            .into_result()?;
    }

    let undistributed_validators = lpos_market
        .get_validators_undistributed_rewards(signer)
        .await?;

    for (validator_id, count) in undistributed_validators {
        for _ in 0..count {
            lpos_market
                .distribute_latest_reward_in_validator(signer, validator_id.clone())
                .await
                .into_result()?;
        }
    }

    Ok(())
}
