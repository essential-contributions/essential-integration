use essential_rest_client::EssentialClient;
use essential_types::{
    intent::{Intent, SignedSet},
    ContentAddress,
};

/// Deploy a set of signed intents to the server.
/// The signed intents are expected to be a JSON serialized `SignedSet`.
pub async fn deploy_signed_bytes(
    addr: String,
    signed_intents: Vec<u8>,
) -> anyhow::Result<ContentAddress> {
    let signed_intents: SignedSet = serde_json::from_slice(&signed_intents)?;
    deploy_signed(addr, signed_intents).await
}

/// Sign and deploy a set of unsigned intents to the server.
/// The unsigned intents are expected to be a JSON serialized `Vec<Intent>`.
pub async fn deploy_bytes(
    addr: String,
    account_name: &str,
    wallet: &mut essential_wallet::Wallet,
    intents: Vec<u8>,
) -> anyhow::Result<ContentAddress> {
    let intents: Vec<Intent> = serde_json::from_slice(&intents)?;
    sign_and_deploy(addr, account_name, wallet, intents).await
}

/// Deploy a set of signed intents to the server.
pub async fn deploy_signed(
    addr: String,
    signed_intents: SignedSet,
) -> anyhow::Result<ContentAddress> {
    let client = EssentialClient::new(addr)?;
    client.deploy_intent_set(signed_intents).await
}

/// Sign and deploy a set of unsigned intents to the server.
pub async fn sign_and_deploy(
    addr: String,
    account_name: &str,
    wallet: &mut essential_wallet::Wallet,
    intents: Vec<Intent>,
) -> anyhow::Result<ContentAddress> {
    let signed_intents = wallet.sign_intent_set(intents, account_name)?;
    deploy_signed(addr, signed_intents).await
}
