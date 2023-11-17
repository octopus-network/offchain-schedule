use anyhow::Ok;
use serde_json::json;

use crate::*;

use super::NearContract;

#[async_trait]
pub trait AppchainRegistry: NearContract {
    async fn get_appchain_ids(&self, signer: &Account) -> anyhow::Result<Vec<String>> {
        let result: Vec<String> = signer
            .view(self.get_contract_id(), "get_appchain_ids")
            .await?
            .json()?;
        Ok(result)
    }

    async fn get_appchain_status_of(
        &self,
        signer: &Account,
        appchain_id: String,
    ) -> anyhow::Result<AppchainStatus> {
        let result: AppchainStatus = signer
            .view(self.get_contract_id(), "get_appchain_status_of")
            .args_json(json!({
                "appchain_id": appchain_id
            }))
            .await?
            .json()?;
        Ok(result)
    }
}

/// The state of an appchain
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum AppchainState {
    Registered,
    Audited,
    Voting,
    Booting,
    Active,
    Closing,
    Closed,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum SubstrateTemplateType {
    Barnacle,
    BarnacleEvm,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum AppchainType {
    Cosmos,
    Substrate(SubstrateTemplateType),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AppchainMetadata {
    // pub description: String,
    pub appchain_type: AppchainType,
    // pub website_url: String,
    // pub function_spec_url: String,
    // pub github_address: String,
    // pub github_release: String,
    // pub contact_email: String,
    // pub premined_wrapped_appchain_token_beneficiary: Option<AccountId>,
    // pub premined_wrapped_appchain_token: U128,
    // pub initial_supply_of_wrapped_appchain_token: U128,
    // pub ido_amount_of_wrapped_appchain_token: U128,
    // pub initial_era_reward: U128,
    // pub fungible_token_metadata: FungibleTokenMetadata,
    // pub custom_metadata: HashMap<String, String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AppchainStatus {
    // pub appchain_id: AppchainId,
    // pub evm_chain_id: Option<U64>,
    pub appchain_metadata: AppchainMetadata,
    pub appchain_anchor: Option<AccountId>,
    // pub appchain_owner: AccountId,
    // pub register_deposit: U128,
    pub appchain_state: AppchainState,
    // pub upvote_deposit: U128,
    // pub downvote_deposit: U128,
    // pub voting_score: I128,
    // pub registered_time: U64,
    // pub go_live_time: U64,
    // pub validator_count: u32,
    // pub total_stake: U128,
    // pub dao_proposal_url: Option<String>,
}
