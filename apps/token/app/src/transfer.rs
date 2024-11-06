//! # Transfer
//! Contains functionality for transferring tokens between accounts in the token contract.

use essential_app_utils::inputs::Encode;
use essential_sign::secp256k1::ecdsa::RecoverableSignature;
use essential_types::{
    solution::{Solution, SolutionData},
    Word,
};

use crate::{balance, nonce, Query};

/// Represents the initial data required for transferring tokens.
pub struct Init {
    /// The hashed key of the sender.
    pub hashed_from_key: [Word; 4],
    /// The hashed key of the recipient.
    pub hashed_to_key: [Word; 4],
    /// The amount of tokens to transfer.
    pub amount: Word,
    /// The current nonce of the sender.
    pub nonce: Query,
}

/// Represents the data to be signed for a transfer solution.
pub struct ToSign {
    /// The hashed key of the sender.
    pub hashed_from_key: [Word; 4],
    /// The hashed key of the recipient.
    pub hashed_to_key: [Word; 4],
    /// The amount of tokens to transfer.
    pub amount: Word,
    /// The new nonce of the sender.
    pub new_nonce: Word,
}

/// Contains all necessary information to build a transfer solution.
pub struct BuildSolution {
    /// The hashed key of the sender.
    pub hashed_from_key: [Word; 4],
    /// The hashed key of the recipient.
    pub hashed_to_key: [Word; 4],
    /// The new nonce of the sender.
    pub new_nonce: Word,
    /// The amount of tokens to transfer.
    pub amount: Word,
    /// The current balance of the sender.
    pub current_from_balance: Query,
    /// The current balance of the recipient.
    pub current_to_balance: Query,
    /// The signature over the data.
    pub signature: RecoverableSignature,
}

impl ToSign {
    /// Converts the ToSign struct to a vector of Words for signing.
    pub fn to_words(&self) -> Vec<Word> {
        vec![
            self.hashed_from_key[0],
            self.hashed_from_key[1],
            self.hashed_from_key[2],
            self.hashed_from_key[3],
            self.hashed_to_key[0],
            self.hashed_to_key[1],
            self.hashed_to_key[2],
            self.hashed_to_key[3],
            self.amount,
            self.new_nonce,
        ]
    }
}

/// Prepares the data to be signed for a transfer solution.
pub fn data_to_sign(account: Init) -> anyhow::Result<ToSign> {
    let Init {
        hashed_from_key,
        hashed_to_key,
        amount,
        nonce: current_nonce,
    } = account;
    let new_nonce = increment_nonce(nonce(current_nonce)?);
    Ok(ToSign {
        amount,
        new_nonce,
        hashed_from_key,
        hashed_to_key,
    })
}

/// Builds a transfer solution based on the provided data.
pub fn build_solution(build: BuildSolution) -> anyhow::Result<Solution> {
    let BuildSolution {
        hashed_from_key,
        hashed_to_key,
        new_nonce,
        amount,
        current_from_balance,
        current_to_balance,
        signature,
    } = build;
    let from_balance = calculate_from_balance(balance(current_from_balance)?, amount)?;
    let to_balance = calculate_to_balance(balance(current_to_balance)?, amount)?;
    let signature = signature.encode();
    let auth =
        super::token::TransferAuthMode::Signed((signature, super::token::TransferSignedMode::All));
    let vars = super::token::Transfer::Vars {
        key: hashed_from_key,
        to: hashed_to_key,
        amount,
        auth: (auth, super::token::ExtraConstraints::None),
    };
    let mutations = super::token::storage::mutations()
        .balances(|map| map.entry(hashed_from_key, from_balance))
        .balances(|map| map.entry(hashed_to_key, to_balance))
        .nonce(|nonces| nonces.entry(hashed_from_key, new_nonce));
    let solution = SolutionData {
        predicate_to_solve: super::token::Transfer::ADDRESS,
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

/// Calculates the new balance for the sender after transferring tokens.
fn calculate_from_balance(from_balance: Word, amount: Word) -> anyhow::Result<Word> {
    from_balance
        .checked_sub(amount)
        .ok_or(anyhow::anyhow!("Insufficient balance"))
}

/// Calculates the new balance for the recipient after receiving tokens.
fn calculate_to_balance(to_balance: Word, amount: Word) -> anyhow::Result<Word> {
    to_balance
        .checked_add(amount)
        .ok_or(anyhow::anyhow!("Insufficient balance"))
}
