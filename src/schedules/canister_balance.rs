use crate::global::SYS_ENV;
use crate::ic::create_agent;
use ic_utils::{call::AsyncCall, interfaces::ManagementCanister};
use num_bigint::BigUint;
use tracing::info;

pub async fn check_canister_balance() -> anyhow::Result<()> {
    let sys_env = SYS_ENV
        .get()
        .ok_or(anyhow::anyhow!("Failed to get SYS_ENV"))?;

    for canister_info in &sys_env.canister_info_list {
        let agent = create_agent(canister_info.agent_identity.clone()).expect("");
        let ic00 = ManagementCanister::create(&agent);
        let result = ic00
            .canister_status(&canister_info.canister_id)
            .call_and_wait()
            .await?;
        info!(
            "canister_balance: {:?}: {:?}",
            canister_info.canister_id.to_string(),
            result.0.cycles
        );
        if result.0.cycles.0 < BigUint::from(1_000_000_000_000u64) {
            info!(
                "OCTOPUS_ALERT: insufficient canister balance: {:?}: {:?}",
                canister_info.canister_id.to_string(),
                result.0.cycles
            );
        }
    }

    Ok(())
}
