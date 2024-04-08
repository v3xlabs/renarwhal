use alloy::{
    hex::FromHex,
    network::Ethereum,
    primitives::{Address, U256},
};
use alloy_provider::ProviderBuilder;
use alloy_sol_types::sol;

use super::ETHName;

// 0x253553366Da8546fC250F225fe3d25d0C782303b
sol! {
    #[sol(rpc)]
    contract ETHRegistrarController {
        #[derive(Debug)]
        function rentPrice(string name, uint256 duration) public returns (uint256);
    }
}

impl ETHName {
    pub async fn get_price(&self /* env: &Environment */) -> U256 {
        let registrar_controller_address =
            Address::from_hex("0x253553366Da8546fC250F225fe3d25d0C782303b")
                .expect("Invalid ETHRegistrarController Address");

        let provider = ProviderBuilder::new()
            .with_gas_estimation()
            .network::<Ethereum>()
            .on_http("https://rpc.ankr.com/eth".try_into().unwrap())
            .unwrap();

        let registrar_controller =
            ETHRegistrarController::new(registrar_controller_address, provider);

        let duration = U256::from(24 * 365 * 60 * 60);

        let x = registrar_controller
            .rentPrice(self.name.to_string(), duration)
            .call()
            .await
            .unwrap();

        x._0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_price() {
        let estimate = ETHName {
            name: "lucemans".to_string(),
        }
        .get_price()
        .await;

        println!("Expires: {:?}", estimate);
    }
}
