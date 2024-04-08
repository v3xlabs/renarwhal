use alloy::{
    hex::FromHex,
    network::Ethereum,
    primitives::{keccak256, Address, U256},
};
use alloy_provider::ProviderBuilder;
use alloy_sol_types::sol;
use std::str::FromStr;

use super::ETHName;

sol! {
    #[sol(rpc)]
    contract ETHBaseRegistrar {
        #[derive(Debug)]
        function nameExpires(uint256 id) public returns (uint256);
    }
}

impl ETHName {
    pub async fn get_price(&self, /* provider: &dyn Provider */ /* env: &Environment */) -> U256 {
        let base_registrar_address =
            Address::from_hex("0x57f1887a8BF19b14fC0dF6Fd9B2acc9Af147eA85")
                .expect("Invalid ETHBaseRegistrar Address");

        let provider = ProviderBuilder::new()
            .with_gas_estimation()
            .network::<Ethereum>()
            .on_http("https://rpc.ankr.com/eth".try_into().unwrap())
            .unwrap();

        let base_registrar = ETHBaseRegistrar::new(base_registrar_address, provider);

        let hash = keccak256("lucemans").to_string();

        let hash = U256::from_str(&hash).unwrap();

        let x = base_registrar.nameExpires(hash).call().await.unwrap();

        x._0
    }
}

#[cfg(test)]
mod tests {
    // use alloy::{
    //     network::Ethereum,
    //     providers::ProviderBuilder,
    // };

    use super::*;

    #[tokio::test]
    async fn test_get_price() {
        // let provider = ProviderBuilder::new()
        //     .with_gas_estimation()
        //     .network::<Ethereum>()
        //     .on_http("https://rpc.ankr.com/eth".try_into().unwrap())
        //     .unwrap();

        let estimate = ETHName {
            name: "lucemans".to_string(),
        }
        .get_price()
        .await;

        println!("Expires: {:?}", estimate);
    }
}
