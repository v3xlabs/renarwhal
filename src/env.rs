use alloy_provider::Provider;

pub struct Environment {
    pub RPC_URL: String,
    pub provider: Box<dyn Provider>
}
