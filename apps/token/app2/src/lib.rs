pub mod token {
    pint_abi::gen_from_file! {
        abi: "../pint/token/out/debug/token-abi.json",
        contract:  "../pint/token/out/debug/token.json",
    }
}

pub mod burn;
pub mod mint;
