use anyhow::Ok;

use crate::{
    types::{MultiTxsOperationProcessingResult, ValidatorStatus},
    *,
};

pub async fn handle_unstake_batch() -> anyhow::Result<()> {
    info!("handle_unstake_batch");

    let restaking_base = RESTAKING_BASE
        .get()
        .ok_or_else(|| anyhow!("Failed to get LPOS_MARKET."))?;

    let signer = SIGNER
        .get()
        .ok_or_else(|| anyhow!("Failed to get SIGNER."))?;

    let staking_pools = restaking_base.get_staking_pools(signer).await?;
    let current_epoch_height = restaking_base.get_current_epoch_height(signer).await?;

    for staking_pool_info in staking_pools {
        if staking_pool_info.last_unstake_batch_id.is_some()
            && staking_pool_info.last_unstake_epoch + 4 <= current_epoch_height
        {
            restaking_base
                .withdraw_unstake_batch(
                    signer,
                    staking_pool_info.pool_id.clone(),
                    staking_pool_info.last_unstake_batch_id.unwrap(),
                )
                .await?
                .into_result()?;
        }

        if staking_pool_info.batched_unstake_amount > 0 {
            restaking_base
                .submit_unstake_batch(signer, staking_pool_info.pool_id.clone())
                .await?
                .into_result()?;
        }
    }

    Ok(())
}

pub async fn process_unstake_for_validators() -> anyhow::Result<()> {
    info!("process_unstake_for_validators");

    let lpos_market = LPOS_MARKET
        .get()
        .ok_or_else(|| anyhow!("Failed to get LPOS_MARKET."))?;

    let restaking_base = RESTAKING_BASE
        .get()
        .ok_or_else(|| anyhow!("Failed to get LPOS_MARKET."))?;

    let signer = SIGNER
        .get()
        .ok_or_else(|| anyhow!("Failed to get SIGNER."))?;

    let validator_infos = lpos_market.get_validators(signer).await?;

    for validator_info in validator_infos {
        if !matches!(validator_info.status, ValidatorStatus::Unstaking)
            && !matches!(validator_info.status, ValidatorStatus::ToBeDestroyed)
        {
            continue;
        }

        if matches!(validator_info.status, ValidatorStatus::Unstaking) {
            // 1. check unstake withdrawal is withdrawable
            let is_withdrawable = restaking_base
                .is_withdrawable(
                    signer,
                    validator_info.escrow_id.clone(),
                    validator_info.unstake_withdraw_certificate.unwrap(),
                )
                .await?;
            if !is_withdrawable {
                continue;
            }
            lpos_market
                .withdraw_unstake(signer, validator_info.validator_id.clone())
                .await?
                .into_result()?;
        }

        loop_clean_validator_state(validator_info.validator_id.clone()).await?;

        let option_validator_detail = lpos_market
            .get_validator_detail(signer, validator_info.validator_id.clone())
            .await?;
        if let Some(validator_detail) = option_validator_detail {
            for (token_id, _) in validator_detail.rewards {
                lpos_market
                    .claim_reward_in_validator(
                        signer,
                        validator_info.validator_id.clone(),
                        token_id,
                    )
                    .await?
                    .into_result()?;
            }

            for delegator_id in validator_detail.delegators {
                lpos_market
                    .undelegate_in_unstake(signer, delegator_id)
                    .await?
                    .into_result()?;
            }
        }

        lpos_market
            .destroy(signer, validator_info.validator_id.clone())
            .await?
            .into_result()?;
    }

    Ok(())
}

async fn loop_clean_validator_state(validator_id: AccountId) -> anyhow::Result<()> {
    let lpos_market = LPOS_MARKET
        .get()
        .ok_or_else(|| anyhow!("Failed to get LPOS_MARKET."))?;
    let signer = SIGNER
        .get()
        .ok_or_else(|| anyhow!("Failed to get SIGNER."))?;
    let mut max_times = 10;
    while max_times > 0 {
        let result: MultiTxsOperationProcessingResult = lpos_market
            .clean_validator_state(signer, validator_id.clone())
            .await?
            .into_result()?
            .json()?;
        if matches!(result, MultiTxsOperationProcessingResult::Ok) {
            return Ok(());
        }
        max_times -= 1;
    }

    Err(anyhow!("Clean validator state exceed max times."))
}
