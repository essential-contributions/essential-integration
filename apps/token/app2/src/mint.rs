use anyhow::bail;
use essential_app_utils::inputs::Encode;
use essential_sign::secp256k1::ecdsa::RecoverableSignature;
use essential_types::{
    convert::word_4_from_u8_32, solution::{Solution, SolutionData}, Value, Word
};

pub struct Query(pub Option<Value>);

pub struct Account {
    pub hashed_key: [Word; 4],
    pub amount: Word,
    pub decimals: Word,
    pub nonce: Query,
}

pub struct ToSign {
    pub hashed_key: [Word; 4],
    pub amount: Word,
    pub decimals: Word,
    pub new_nonce: Word,
}

pub struct BuildSolution {
    pub new_nonce: Word,
    pub current_balance: Query,
    pub hashed_key: [Word; 4],
    pub amount: Word,
    pub decimals: Word,
    pub signature: RecoverableSignature,
    pub token_name: String,
    pub token_symbol: String,
}

pub struct Submit(pub Solution);

impl ToSign {
    pub fn to_words(&self) -> Vec<Word> {
        vec![
            self.hashed_key[0],
            self.hashed_key[1],
            self.hashed_key[2],
            self.hashed_key[3],
            self.amount,
            self.new_nonce,
            self.decimals,
        ]
    }
}

pub fn data_to_sign(account: Account) -> anyhow::Result<ToSign> {
    let Account {
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
    let balance = balance(current_balance)?;
    let pub_vars = super::token::Mint::PubVars {
        key: hashed_key,
        amount,
        decimals,
    };
    let signature = signature.encode();
    let vars = super::token::Mint::Vars {
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