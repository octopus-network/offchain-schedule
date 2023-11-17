use std::time::Duration;

use crate::schedules::anchor_actions::fetch_validator_set_from_restaking_base_and_send_vsc_packet_to_appchain_in_anchors;
use crate::schedules::transfer_for_cross_chain::transfer_for_cross_chain;
use anyhow::anyhow;
use async_trait::async_trait;
use clokwerk::{AsyncScheduler, Interval::*, Job};
use clokwerk::{Scheduler, TimeUnits};
use cmd_args::CmdArgs;
use global::*;
use near_workspaces::AccountId;
use near_workspaces::{result::ExecutionFinalResult, Account};
use schedules::distribute_rewards::distribute_lpos_market_reward;
use serde::{Deserialize, Serialize};
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};

mod cmd_args;
mod global;
mod near;
mod schedules;
mod types;
mod util;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_log();

    init_env_config().await?;

    info!("Completed init env config. {:?}", SYS_ENV.get().unwrap());

    let mut scheduler = AsyncScheduler::new();

    scheduler.every(1.hours()).run(|| async {
        let result = distribute_lpos_market_reward().await;
        info!("distribute_lpos_market_reward result: {:?}", result);
    });

    // todo is 1 hours suitable?
    scheduler.every(1.hours()).run(|| async {
        let result = transfer_for_cross_chain().await;
        info!("transfer_for_cross_chain result: {:?}", result);
    });

    scheduler.every(1.days()).run(|| async {
        let result =
            fetch_validator_set_from_restaking_base_and_send_vsc_packet_to_appchain_in_anchors()
                .await;
        info!("fetch_validator_set_from_restaking_base_and_send_vsc_packet_to_appchain_in_anchors result: {:?}", result);
    });

    loop {
        scheduler.run_pending().await;
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

pub fn init_log() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    // 输出到控制台中
    let formatting_layer = fmt::layer().pretty().with_writer(std::io::stderr);

    Registry::default()
        .with(env_filter)
        .with(formatting_layer)
        .init();
}
