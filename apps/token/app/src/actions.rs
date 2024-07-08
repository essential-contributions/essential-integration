use crate::token::Addresses;
use essential_app_utils::{
    addresses::get_addresses,
    compile::compile_pint_project,
    print::{print_contract_address, print_predicate_address},
};
use essential_rest_client::EssentialClient;
use essential_types::contract::Contract;
use std::path::PathBuf;

pub async fn compile_addresses(pint_directory: PathBuf) -> anyhow::Result<Addresses> {
    let token_contract =
        compile_pint_project(pint_directory.clone().join("token"), "token").await?;
    let token_addresses = get_addresses(&token_contract);
    let signed_contract =
        compile_pint_project(pint_directory.clone().join("signed"), "signed").await?;
    let signed_addresses = get_addresses(&signed_contract);

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
        signed_transfer_with: signed_addresses.1[4].clone(),
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
        signed_transfer_with,
        signed_mint,
        signed_burn,
        signed_cancel,
    } = addresses;
    print_contract_address("token", token);
    print_predicate_address("burn", burn);
    print_predicate_address("cancel", cancel);
    print_predicate_address("mint", mint);
    print_predicate_address("transfer", transfer);
    print_contract_address("signed", signed);
    print_predicate_address("signed_transfer", signed_transfer);
    print_predicate_address("signed_transfer_with", signed_transfer_with);
    print_predicate_address("signed_burn", signed_burn);
    print_predicate_address("signed_mint", signed_mint);
    print_predicate_address("signed_cancel", signed_cancel);
}

pub async fn deploy_app(
    addr: String,
    wallet: &mut essential_wallet::Wallet,
    account_name: &str,
    pint_directory: PathBuf,
) -> anyhow::Result<Addresses> {
    let client = EssentialClient::new(addr)?;
    let token_contract =
        compile_pint_project(pint_directory.clone().join("token"), "token").await?;
    let token_addresses = get_addresses(&token_contract);
    let signed_contract =
        compile_pint_project(pint_directory.clone().join("signed"), "signed").await?;
    let signed_addresses = get_addresses(&signed_contract);

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
        signed_transfer_with: signed_addresses.1[4].clone(),
    };

    let predicates = wallet.sign_contract(token_contract, account_name)?;
    client.deploy_contract(predicates).await?;
    let predicates = wallet.sign_contract(signed_contract, account_name)?;
    client.deploy_contract(predicates).await?;

    Ok(addresses)
}

pub async fn get_contracts(pint_directory: PathBuf) -> anyhow::Result<Vec<Contract>> {
    let token_contract =
        compile_pint_project(pint_directory.clone().join("token"), "token").await?;
    let signed_contract =
        compile_pint_project(pint_directory.clone().join("signed"), "signed").await?;
    Ok(vec![token_contract, signed_contract])
}
