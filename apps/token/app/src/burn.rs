//! Contains functionality for burning tokens in the token contract.
use essential_app_utils::inputs::Encode;
use essential_sign::secp256k1::ecdsa::RecoverableSignature;
use essential_types::{solution::Solution, Word};

use crate::{balance, nonce, Query};

/// Represents the initial data required for burning tokens.
pub struct Init {
    /// The hashed key of the account.
    pub hashed_key: [Word; 4],
    /// The amount of tokens to burn.
    pub amount: Word,
    /// The current nonce of the account.
    pub nonce: Query,
}

/// Represents the data to be signed for a burn solutions.
pub struct ToSign {
    /// The hashed key of the account.
    pub hashed_key: [Word; 4],
    /// The amount of tokens to burn.
    pub amount: Word,
    /// The new nonce of the account.
    pub new_nonce: Word,
}

/// Contains all necessary information to build a burn solution.
pub struct BuildSolution {
    /// The new nonce of the account.
    pub new_nonce: Word,
    /// The current balance of the account.
    pub current_balance: Query,
    /// The hashed key of the account.
    pub hashed_key: [Word; 4],
    /// The amount of tokens to burn.
    pub amount: Word,
    /// The signature over the data.
    pub signature: RecoverableSignature,
}

/// Prepares the data to be signed for a burn transaction.
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

/// Builds a burn solution based on the provided data.
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
    let signature = signature.encode();
    let vars = super::token::Burn::Vars {
        key: hashed_key,
        amount,
        auth: super::token::BurnAuth::Signed(signature),
    };
    let mutations = super::token::storage::mutations()
        .balances(|map| map.entry(hashed_key, new_from_balance))
        .nonce(|nonces| nonces.entry(hashed_key, new_nonce));
    let solution = Solution {
        predicate_to_solve: super::token::Burn::ADDRESS,
        predicate_data: vars.into(),
        state_mutations: mutations.into(),
    };
    Ok(solution)
}

/// Increments the nonce by one.
fn increment_nonce(nonce: Word) -> Word {
    nonce + 1
}

/// Calculates the new balance after burning tokens.
fn calculate_from_balance(from_balance: Word, amount: Word) -> anyhow::Result<Word> {
    from_balance
        .checked_sub(amount)
        .ok_or(anyhow::anyhow!("Insufficient balance"))
}

impl ToSign {
    /// Converts the ToSign struct to a vector of Words for signing.
    pub fn to_words(&self) -> Vec<Word> {
        self.hashed_key
            .iter()
            .copied()
            .chain([self.amount, self.new_nonce].iter().copied())
            .collect()
    }
}
