use ic_agent::{Agent, Identity};

pub fn create_agent(identity: impl Identity + 'static) -> Result<Agent, String> {
    Agent::builder()
        .with_url("https://ic0.app")
        .with_identity(identity)
        .build()
        .map_err(|e| format!("{:?}", e))
}
