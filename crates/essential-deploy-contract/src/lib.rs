use essential_rest_client::EssentialClient;
use essential_types::{
    contract::{Contract, SignedContract},
    ContentAddress,
};

/// Deploy a contract to the server.
/// The signed contract is expected to be a JSON serialized `SignedContract`.
pub async fn deploy_signed_bytes(
    addr: String,
    signed_predicates: Vec<u8>,
) -> anyhow::Result<ContentAddress> {
    let signed_predicates: SignedContract = serde_json::from_slice(&signed_predicates)?;
    deploy_signed(addr, signed_predicates).await
}

/// Sign and deploy a unsigned contract to the server.
/// The unsigned contract is expected to be a JSON serialized `Contract`.
pub async fn deploy_bytes(
    addr: String,
    account_name: &str,
    wallet: &mut essential_wallet::Wallet,
    contract: Vec<u8>,
) -> anyhow::Result<ContentAddress> {
    let contract: Contract = serde_json::from_slice(&contract)?;
    sign_and_deploy(addr, account_name, wallet, contract).await
}

/// Deploy a signed contract to the server.
pub async fn deploy_signed(
    addr: String,
    signed_predicates: SignedContract,
) -> anyhow::Result<ContentAddress> {
    let client = EssentialClient::new(addr)?;
    client.deploy_contract(signed_predicates).await
}

/// Sign and deploy a unsigned contract to the server.
pub async fn sign_and_deploy(
    addr: String,
    account_name: &str,
    wallet: &mut essential_wallet::Wallet,
    contract: Contract,
) -> anyhow::Result<ContentAddress> {
    let signed_predicates = wallet.sign_contract(contract, account_name)?;
    deploy_signed(addr, signed_predicates).await
}
