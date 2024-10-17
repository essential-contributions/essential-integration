use anyhow::bail;
use essential_app_utils::inputs::Encode;
use essential_sign::secp256k1::ecdsa::RecoverableSignature;
use essential_types::{
    solution::{Solution, SolutionData},
    Word,
};

use crate::Query;

pub struct Init {
    pub hashed_key: [Word; 4],
    pub nonce: Query,
    pub amount: Word,
}

pub struct ToSign {
    pub hashed_key: [Word; 4],
    pub amount: Word,
    pub new_nonce: Word,
}

pub struct BuildSolution {
    pub new_nonce: Word,
    pub current_balance: Query,
    pub hashed_key: [Word; 4],
    pub amount: Word,
    pub signature: RecoverableSignature,
}

pub struct Submit(pub Solution);

pub fn data_to_sign(account: Init) -> anyhow::Result<ToSign> {
    let Init {
        hashed_key,
        nonce: current_nonce,
        amount,
    } = account;
    let new_nonce = increment_nonce(nonce(current_nonce)?);
    Ok(ToSign {
        hashed_key,
        amount,
        new_nonce,
    })
}

pub fn build_solution(build: BuildSolution) -> anyhow::Result<Solution> {
    let BuildSolution {
        new_nonce,
        current_balance,
        hashed_key,
        amount,
        signature,
    } = build;
    let from_balance = balance(current_balance)?;
    let new_from_balance = calculate_from_balance(from_balance, amount)?;
    let pub_vars = super::token::Burn::PubVars {
        key: hashed_key,
        amount,
    };
    let signature = signature.encode();
    let vars = super::token::Burn::Vars {
        auth: super::token::BurnAuth::Signed(signature),
    };
    let mutations = super::token::storage::mutations()
        .balances(|map| map.entry(hashed_key, new_from_balance))
        .nonce(|nonces| nonces.entry(hashed_key, new_nonce));
    let solution = SolutionData {
        predicate_to_solve: super::token::Burn::ADDRESS,
        decision_variables: vars.into(),
        transient_data: pub_vars.into(),
        state_mutations: mutations.into(),
    };
    Ok(Solution {
        data: vec![solution],
    })
}

fn nonce(nonce: Query) -> anyhow::Result<Word> {
    let r = match nonce.0 {
        Some(nonce) => match &nonce[..] {
            [] => 0,
            [nonce] => *nonce,
            _ => bail!("Expected single word, got: {:?}", nonce),
        },
        None => 0,
    };
    Ok(r)
}

fn balance(balance: Query) -> anyhow::Result<Word> {
    let r = match balance.0 {
        Some(balance) => match &balance[..] {
            [] => 0,
            [balance] => *balance,
            _ => bail!("Expected single word, got: {:?}", balance),
        },
        None => 0,
    };
    Ok(r)
}

fn increment_nonce(nonce: Word) -> Word {
    nonce + 1
}

fn calculate_from_balance(from_balance: Word, amount: Word) -> anyhow::Result<Word> {
    from_balance
        .checked_sub(amount)
        .ok_or(anyhow::anyhow!("Insufficient balance"))
}
