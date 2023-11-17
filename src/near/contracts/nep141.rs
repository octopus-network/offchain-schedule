use near_gas::NearGas;
use near_workspaces::{result::ExecutionFinalResult, Account, AccountId};
use serde_json::json;

use crate::*;

#[derive(Debug)]
pub struct Nep141 {
    pub contract_id: AccountId,
}

impl Nep141 {
    pub fn new(contract_id: AccountId) -> Self {
        Nep141 { contract_id }
    }

    pub async fn ft_transfer_call(
        &self,
        signer: &Account,
        receiver_id: String,
        amount: String,
        msg: String,
        memo: Option<String>,
    ) -> anyhow::Result<ExecutionFinalResult> {
        signer
            .call(&self.contract_id, "ft_transfer")
            .args_json(json!({
                "receiver_id": receiver_id,
                "amount": amount,
                "msg": msg,
                "memo": memo
            }))
            .gas(NearGas::from_tgas(80))
            .deposit(1)
            .transact()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to distribute_latest_reward, error: {:?}", e))
    }
}
