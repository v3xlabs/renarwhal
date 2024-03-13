use ethers::abi::Token;
use ethers::providers::Http;
use ethers::providers::Middleware as _;
use ethers::providers::Provider;
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::Address;
use ethers::types::Bytes;
use ethers::types::U256;
use ethers_core::abi;
use hex_literal::hex;

fn main() {
    println!("Hello, world!");
}

pub async fn calculate_transaction() {
    println!("Calculating transaction...");
}

pub async fn calculate_renewal_price(names: &[&str]) -> Option<U256> {
    let provider = Provider::<Http>::try_from("https://rpc.ankr.com/eth_goerli/").unwrap();

    let mut transaction = TypedTransaction::default();

    transaction.set_to("0x5938C0A94e59631A74e6413985bDF5Fc6D0abAD1".parse::<Address>().unwrap());

    let price = 3125000000003490u64.into(); // dynamic

    let payload = {
        let function_signature = hex!("2c7e9b39");

        let duration = 31536000.into();

        let args = abi::encode(&[
            Token::Array(
                names
                    .iter()
                    .map(|name| Token::String(name.to_string()))
                    .collect(),
            ),
            Token::Uint(duration),
            Token::Uint(price),
        ]);

        Bytes::from([function_signature.to_vec(), args].concat())
    };

    transaction.set_data(payload);
    transaction.set_value(price * names.len());

    let transaction = provider.estimate_gas(&transaction, None).await.unwrap();

    print_gas_report(transaction, names.len());

    Some(transaction)
}

pub fn print_gas_report(gas: U256, names: usize) {
    println!("====================");
    println!("Gas estimate for renewing {} names: {}", names, gas);
    let per = gas / names;
    println!("Gas per name: {}", per);
    println!("====================");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_calculate_renewal_price() {
        let names = vec!["lucemans", "willbreak"];
        let result = calculate_renewal_price(&names).await;
        assert_eq!(result, Some(138006.into()));
        let names = vec!["lucemans", "willbreak", "antony"];
        let result = calculate_renewal_price(&names).await;
        // assert_eq!(result, Some(138006.into()));
        let names = vec!["lucemans", "willbreak", "antony", "foundation"];
        let result = calculate_renewal_price(&names).await;
        // assert_eq!(result, Some(138006.into()));
        let names = vec!["lucemans", "willbreak", "antony", "foundation", "rackspace"];
        let result = calculate_renewal_price(&names).await;
        // assert_eq!(result, Some(138006.into()));
        let names = vec!["lucemans", "willbreak", "antony", "foundation", "rackspace", "vercel"];
        let result = calculate_renewal_price(&names).await;
        // assert_eq!(result, Some(138006.into()));
        let names = vec!["lucemans", "willbreak", "antony", "foundation", "rackspace", "vercel", "snapshot"];
        let result = calculate_renewal_price(&names).await;
        // assert_eq!(result, Some(138006.into()));
        let names = vec!["lucemans", "willbreak", "antony", "foundation", "rackspace", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "lucemans", "willbreak", "antony", "foundation", "rackspace", "vercel", "lucemans", "willbreak", "antony", "foundation", "rackspace", "vercel"];
        let result = calculate_renewal_price(&names).await;
        // assert_eq!(result, Some(138006.into()));
        let names = vec!["lucemans", "willbreak", "antony", "foundation", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "lucemans", "willbreak", "antony", "foundation", "rackspace", "vercel", "lucemans", "willbreak", "antony", "foundation", "rackspace", "vercel"];
        let result = calculate_renewal_price(&names).await;
        // assert_eq!(result, Some(138006.into()));
        let names = vec!["lucemans", "willbreak", "antony", "foundation", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "lucemans", "willbreak", "antony", "foundation", "rackspace", "vercel", "lucemans", "willbreak"];
        let result = calculate_renewal_price(&names).await;
        // assert_eq!(result, Some(138006.into()));
        let names = vec!["lucemans", "willbreak", "antony", "foundation", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "lucemans", "willbreak", "antony", "foundation"];
        let result = calculate_renewal_price(&names).await;
        // assert_eq!(result, Some(138006.into()));
        let names = vec!["lucemans", "willbreak", "antony", "foundation", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "lucemans"];
        let result = calculate_renewal_price(&names).await;
        // assert_eq!(result, Some(138006.into()));
        let names = vec!["lucemans", "willbreak", "antony", "foundation", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot"];
        let result = calculate_renewal_price(&names).await;
        // assert_eq!(result, Some(138006.into()));
        let names = vec!["lucemans", "willbreak", "antony", "foundation", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot"];
        let result = calculate_renewal_price(&names).await;
        // assert_eq!(result, Some(138006.into()));
        let names = vec!["lucemans", "willbreak", "antony", "foundation", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot"];
        let result = calculate_renewal_price(&names).await;
        // assert_eq!(result, Some(138006.into()));
        let names = vec!["lucemans", "willbreak", "antony", "foundation", "rackspace", "vercel", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "lucemans", "willbreak", "antony", "foundation", "rackspace", "vercel", "lucemans", "willbreak", "antony", "foundation", "rackspace", "vercel"];
        let result = calculate_renewal_price(&names).await;
        // assert_eq!(result, Some(138006.into()));
    }

    #[tokio::test]
    async fn test_calculate_renewal_price2() {
        let names = vec!["lucemans", "willbreak", "antony", "foundation", "rackspace", "vercel", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "lucemans", "willbreak", "antony", "foundation", "rackspace", "vercel", "lucemans", "willbreak", "antony", "foundation", "rackspace", "vercel", "lucemans", "willbreak", "antony", "foundation", "rackspace", "vercel", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "lucemans", "willbreak", "antony", "foundation", "rackspace", "vercel", "lucemans", "willbreak", "antony", "foundation", "rackspace", "vercel"];
        let result = calculate_renewal_price(&names).await;
        // assert_eq!(result, Some(138006.into()));
        let names = vec!["lucemans", "willbreak", "antony", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "lucemans", "willbreak", "antony", "foundation", "rackspace", "vercel", "lucemans", "willbreak", "antony", "foundation", "rackspace", "vercel", "lucemans", "willbreak", "antony", "foundation", "rackspace", "vercel", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "lucemans", "willbreak", "antony", "foundation", "rackspace", "vercel", "lucemans", "willbreak", "antony", "foundation", "rackspace", "vercel"];
        let result = calculate_renewal_price(&names).await;
        // assert_eq!(result, Some(138006.into()));
        let names = vec!["lucemans", "willbreak", "antony", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "lucemans", "willbreak", "antony", "foundation", "rackspace", "vercel", "lucemans", "willbreak", "antony", "foundation", "rackspace", "vercel", "lucemans", "willbreak", "antony", "foundation", "rackspace", "vercel", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "snapshot", "lucemans", "willbreak", "antony"];
        let result = calculate_renewal_price(&names).await;
        // assert_eq!(result, Some(138006.into()));
    }
}
