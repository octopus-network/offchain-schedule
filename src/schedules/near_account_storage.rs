use crate::*;

pub async fn check_near_account_storage() -> anyhow::Result<()> {
    let sys_env = SYS_ENV
        .get()
        .ok_or(anyhow::anyhow!("Failed to get SYS_ENV"))?;

    for (account_id, storage_maximum_value) in sys_env.near_account_storage_check_list.iter() {
        let account_details = get_account_details_by_sys_env(account_id).await?;
        info!(
            "Storage usage of {:?}: {:?}",
            account_id, account_details.storage_usage
        );
        if account_details.storage_usage > storage_maximum_value.clone() as u64 {
            info!(
                "OCTOPUS_ALERT: Storage usage of {:?} is {:?}, exceed maximum_value {:?}",
                account_id, account_details.storage_usage, storage_maximum_value
            );
        }
    }

    Ok(())
}
