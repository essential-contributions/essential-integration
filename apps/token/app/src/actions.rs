use crate::token::Addresses;
use app_utils::{
    addresses::get_addresses,
    compile::compile_pint_file,
    print::{print_intent_address, print_set_address},
};
use std::path::PathBuf;

pub async fn compile_addresses(pint_directory: PathBuf) -> anyhow::Result<Addresses> {
    let token_intents = compile_pint_file(pint_directory.clone(), "token.pnt").await?;
    let token_addresses = get_addresses(&token_intents);

    let addresses = Addresses {
        token: token_addresses.0.clone(),
        burn: token_addresses.1[0].clone(),
        init: token_addresses.1[1].clone(),
        mint: token_addresses.1[2].clone(),
        transfer: token_addresses.1[3].clone(),
    };

    Ok(addresses)
}

pub fn print_addresses(addresses: &Addresses) {
    let Addresses {
        token,
        burn,
        init,
        mint,
        transfer,
    } = addresses;
    print_set_address("token", token);
    print_intent_address("burn", burn);
    print_intent_address("init", init);
    print_intent_address("mint", mint);
    print_intent_address("transfer", transfer);
}
