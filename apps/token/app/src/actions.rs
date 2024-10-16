use crate::token::{signed, token};
use anyhow::ensure;
use essential_app_utils::{
    compile::compile_pint_project,
    print::{print_contract_address, print_predicate_address},
};
use essential_types::contract::Contract;
use std::path::{Path, PathBuf};

/// Compiles the token and signed contracts and returns them in a tuple in that order.
///
/// Returns an `Err` in the case that a newly compiled contract address differs
/// from the ABI-provided address.
pub async fn compile_contracts(pint_directory: &Path) -> anyhow::Result<(Contract, Contract)> {
    let token_contract = compile_pint_project(pint_directory.join("token")).await?;
    let signed_contract = compile_pint_project(pint_directory.join("signed")).await?;
    ensure!(token::ADDRESS == essential_hash::contract_addr::from_contract(&token_contract));
    ensure!(signed::ADDRESS == essential_hash::contract_addr::from_contract(&signed_contract));
    Ok((token_contract, signed_contract))
}

pub fn print_addresses() {
    print_contract_address("token", &token::ADDRESS);
    print_predicate_address("burn", &token::Burn::ADDRESS);
    print_predicate_address("cancel", &token::Cancel::ADDRESS);
    print_predicate_address("mint", &token::Mint::ADDRESS);
    print_predicate_address("transfer", &token::Transfer::ADDRESS);
    print_contract_address("signed", &signed::ADDRESS);
    print_predicate_address("signed_transfer", &signed::Transfer::ADDRESS);
    print_predicate_address("signed_transfer_with", &signed::TransferWith::ADDRESS);
    print_predicate_address("signed_burn", &signed::Burn::ADDRESS);
    print_predicate_address("signed_mint", &signed::Mint::ADDRESS);
    print_predicate_address("signed_cancel", &signed::Cancel::ADDRESS);
}

// pub async fn deploy_app(
//     addr: String,
//     wallet: &mut essential_wallet::Wallet,
//     account_name: &str,
//     pint_directory: &Path,
// ) -> anyhow::Result<()> {
//     let client = EssentialClient::new(addr)?;
//     let (token_contract, signed_contract) = compile_contracts(pint_directory).await?;

//     let predicates = wallet.sign_contract(token_contract, account_name)?;
//     client.deploy_contract(predicates).await?;
//     let predicates = wallet.sign_contract(signed_contract, account_name)?;
//     client.deploy_contract(predicates).await?;

//     Ok(())
// }

pub async fn get_contracts(pint_directory: PathBuf) -> anyhow::Result<Vec<Contract>> {
    let token_contract = compile_pint_project(pint_directory.clone().join("token")).await?;
    let signed_contract = compile_pint_project(pint_directory.clone().join("signed")).await?;
    Ok(vec![token_contract, signed_contract])
}
