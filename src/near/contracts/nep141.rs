use crate::*;

#[derive(Debug)]
pub struct Nep141 {
    pub contract_id: AccountId,
}

impl Nep141 {
    pub fn new(contract_id: AccountId) -> Self {
        Nep141 { contract_id }
    }

    pub async fn ft_balance_of(
        &self,
        signer: &Account,
        account_id: AccountId,
    ) -> anyhow::Result<String> {
        let result = signer
            .view(&self.contract_id, "ft_balance_of")
            .args_json(json!({
                "account_id": account_id
            }))
            .await?
            .json()?;
        Ok(result)
    }
}
