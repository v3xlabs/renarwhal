use alloy::primitives::U256;
use alloy_contract::SolCallBuilder;
use alloy_sol_types::sol;

use crate::env::Environment;

use super::ETHName;

impl ETHName {
    pub async fn get_expiry(env: &Environment) -> U256 {
        U256::from(0)
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_get_expiry() {
    }
}
