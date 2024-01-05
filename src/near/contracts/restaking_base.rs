use crate::*;

#[derive(Debug)]
pub struct RestakingBase {
    pub contract_id: AccountId,
}

impl RestakingBase {
    pub fn new(contract_id: AccountId) -> Self {
        Self { contract_id }
    }

    pub async fn is_withdrawable(
        &self,
        signer: &Account,
        staker_id: AccountId,
        certificate: String,
    ) -> anyhow::Result<bool> {
        let result: bool = signer
            .view(&self.contract_id, "is_withdrawable")
            .args_json(json!({
                "staker_id": staker_id,
                "certificate": certificate
            }))
            .await?
            .json()?;

        Ok(result)
    }
}
