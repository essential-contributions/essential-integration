//! # Mint
//! Contains functionality for minting new tokens in the token contract.

use essential_app_utils::inputs::Encode;
use essential_sign::secp256k1::ecdsa::RecoverableSignature;
use essential_types::{
    convert::word_4_from_u8_32,
    solution::{Solution, SolutionData},
    Word,
};

use crate::{balance, nonce, Query};

/// Represents the initial data required for minting tokens.
pub struct Init {
    /// The hashed key of the account.
    pub hashed_key: [Word; 4],
    /// The amount of tokens to mint.
    pub amount: Word,
    /// The number of decimals of the token.
    pub decimals: Word,
    /// The current nonce of the account.
    pub nonce: Query,
}

/// Represents the data to be signed for a mint solution.
pub struct ToSign {
    /// The hashed key of the account.
    pub hashed_key: [Word; 4],
    /// The amount of tokens to mint.
    pub amount: Word,
    /// The number of decimals of the token.
    pub decimals: Word,
    /// The new nonce of the account.
    pub new_nonce: Word,
}

/// Contains all necessary information to build a mint solution.
pub struct BuildSolution {
    /// The new nonce of the account.
    pub new_nonce: Word,
    /// The current balance of the account.
    pub current_balance: Query,
    /// The hashed key of the account.
    pub hashed_key: [Word; 4],
    /// The amount of tokens to mint.
    pub amount: Word,
    /// The number of decimals of the token.
    pub decimals: Word,
    /// The signature over the data.
    pub signature: RecoverableSignature,
    /// The name of the token.
    pub token_name: String,
    /// The symbol of the token.
    pub token_symbol: String,
}

impl ToSign {
    /// Converts the ToSign struct to a vector of Words for signing.
    pub fn to_words(&self) -> Vec<Word> {
        vec![
            self.hashed_key[0],
            self.hashed_key[1],
            self.hashed_key[2],
            self.hashed_key[3],
            self.amount,
            self.decimals,
            self.new_nonce,
        ]
    }
}

/// Prepares the data to be signed for a mint transaction.
pub fn data_to_sign(account: Init) -> anyhow::Result<ToSign> {
    let Init {
        hashed_key,
        nonce: current_nonce,
        amount,
        decimals,
    } = account;
    let new_nonce = increment_nonce(nonce(current_nonce)?);
    Ok(ToSign {
        hashed_key,
        amount,
        new_nonce,
        decimals,
    })
}

/// Builds a mint solution based on the provided data.
pub fn build_solution(build: BuildSolution) -> anyhow::Result<Solution> {
    let BuildSolution {
        new_nonce,
        current_balance,
        hashed_key,
        amount,
        signature,
        decimals,
        token_name,
        token_symbol,
    } = build;
    let balance = calculate_new_balance(balance(current_balance)?, amount)?;
    let signature = signature.encode();
    let vars = super::token::Mint::Vars {
        key: hashed_key,
        amount,
        decimals,
        auth: super::token::MintAuth::Signed(signature),
    };
    let mutations = super::token::storage::mutations()
        .balances(|map| map.entry(hashed_key, balance))
        .token_name(word_4_from_u8_32(essential_hash::hash(&token_name)))
        .token_symbol(word_4_from_u8_32(essential_hash::hash(&token_symbol)))
        .decimals(decimals)
        .nonce(|nonces| nonces.entry(hashed_key, new_nonce));
    let solution = SolutionData {
        predicate_to_solve: super::token::Mint::ADDRESS,
        decision_variables: vars.into(),
        state_mutations: mutations.into(),
    };
    Ok(Solution {
        data: vec![solution],
    })
}

/// Increments the nonce by 1.
fn increment_nonce(nonce: Word) -> Word {
    nonce + 1
}

/// Calculates the new balance after minting tokens.
fn calculate_new_balance(balance: Word, amount: Word) -> anyhow::Result<Word> {
    balance
        .checked_add(amount)
        .ok_or(anyhow::anyhow!("Insufficient balance"))
}
