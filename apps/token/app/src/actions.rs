use crate::token::Addresses;
use app_utils::{
    addresses::get_addresses,
    compile::compile_pint_file,
    print::{print_intent_address, print_set_address},
};
use essential_rest_client::EssentialClient;
use std::path::PathBuf;

pub async fn compile_addresses(pint_directory: PathBuf) -> anyhow::Result<Addresses> {
    let token_intents = compile_pint_file(pint_directory.clone(), "token.pnt").await?;
    let token_addresses = get_addresses(&token_intents);
    let signed_intents = compile_pint_file(pint_directory.clone(), "signed.pnt").await?;
    let signed_addresses = get_addresses(&signed_intents);

    let addresses = Addresses {
        token: token_addresses.0.clone(),
        burn: token_addresses.1[0].clone(),
        cancel: token_addresses.1[1].clone(),
        mint: token_addresses.1[2].clone(),
        transfer: token_addresses.1[3].clone(),
        signed: signed_addresses.0.clone(),
        signed_burn: signed_addresses.1[0].clone(),
        signed_cancel: signed_addresses.1[1].clone(),
        signed_mint: signed_addresses.1[2].clone(),
        signed_transfer: signed_addresses.1[3].clone(),
        signed_transfer_from: signed_addresses.1[4].clone(),
    };

    Ok(addresses)
}

pub fn print_addresses(addresses: &Addresses) {
    let Addresses {
        token,
        burn,
        mint,
        transfer,
        cancel,
        signed,
        signed_transfer,
        signed_transfer_from,
        signed_mint,
        signed_burn,
        signed_cancel,
    } = addresses;
    print_set_address("token", token);
    print_intent_address("burn", burn);
    print_intent_address("cancel", cancel);
    print_intent_address("mint", mint);
    print_intent_address("transfer", transfer);
    print_set_address("signed", signed);
    print_intent_address("signed_transfer", signed_transfer);
    print_intent_address("signed_transfer_from", signed_transfer_from);
    print_intent_address("signed_burn", signed_burn);
    print_intent_address("signed_mint", signed_mint);
    print_intent_address("signed_cancel", signed_cancel);
}

pub async fn deploy_app(
    addr: String,
    wallet: &mut essential_wallet::Wallet,
    account_name: &str,
    pint_directory: PathBuf,
) -> anyhow::Result<Addresses> {
    let client = EssentialClient::new(addr)?;
    let token_intents = compile_pint_file(pint_directory.clone(), "token.pnt").await?;
    let token_addresses = get_addresses(&token_intents);
    let signed_intents = compile_pint_file(pint_directory.clone(), "signed.pnt").await?;
    let signed_addresses = get_addresses(&signed_intents);

    let addresses = Addresses {
        token: token_addresses.0.clone(),
        burn: token_addresses.1[0].clone(),
        cancel: token_addresses.1[1].clone(),
        mint: token_addresses.1[2].clone(),
        transfer: token_addresses.1[3].clone(),
        signed: signed_addresses.0.clone(),
        signed_burn: signed_addresses.1[0].clone(),
        signed_cancel: signed_addresses.1[1].clone(),
        signed_mint: signed_addresses.1[2].clone(),
        signed_transfer: signed_addresses.1[3].clone(),
        signed_transfer_from: signed_addresses.1[4].clone(),
    };

    let intents = wallet.sign_intent_set(token_intents, account_name)?;
    client.deploy_intent_set(intents).await?;
    let intents = wallet.sign_intent_set(signed_intents, account_name)?;
    client.deploy_intent_set(intents).await?;

    Ok(addresses)
}
