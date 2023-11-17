use serde::Serialize;

use crate::*;

pub async fn transfer_for_cross_chain() -> anyhow::Result<()> {
    info!("distribute_lpos_market_reward");

    let otto_token = OTTO_TOKEN
        .get()
        .ok_or(anyhow::anyhow!("Failed to get OTTO_TOKEN"))?;
    let sys_env = SYS_ENV
        .get()
        .ok_or(anyhow::anyhow!("Failed to get SYS_ENV"))?;
    let signer = SIGNER
        .get()
        .ok_or(anyhow::anyhow!("Failed to get SIGNER"))?;

    let transfer_call_msg = serde_json::to_string(&TransferCallMsg {
        receiver: sys_env.dst_chain_transfer_receiver.clone(),
        timeout_seconds: "300".to_string(),
    })?;

    otto_token
        .ft_transfer_call(
            signer,
            sys_env.cross_chain_transfer_receiver.clone(),
            "1".to_string(),
            transfer_call_msg,
            None,
        )
        .await?
        .into_result()?;

    Ok(())
}

#[derive(Serialize)]
struct TransferCallMsg {
    // an address in cosmos, eg: "cosmos1sqkln20dw9a5779nyjw0eka4xsqrk9hdgrueql"
    pub receiver: String,
    // eg: "300"
    pub timeout_seconds: String,
}
