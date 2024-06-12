use std::{collections::HashMap, vec};

use anyhow::bail;
use essential_rest_client::EssentialClient;
use essential_types::{
    convert::word_4_from_u8_32,
    solution::{Mutation, Solution, SolutionData},
    Hash, IntentAddress, Word,
};

pub struct Nft {
    client: EssentialClient,
    wallet: essential_wallet::Wallet,
    deployed_intents: HashMap<String, IntentAddress>,
}

impl Nft {
    pub fn new(
        addr: String,
        deployed_intents: HashMap<String, IntentAddress>,
        wallet: essential_wallet::Wallet,
    ) -> anyhow::Result<Self> {
        let client = EssentialClient::new(addr)?;
        Ok(Self {
            client,
            deployed_intents,
            wallet,
        })
    }

    pub fn create_account(&mut self, account_name: &str) -> anyhow::Result<()> {
        self.wallet
            .new_key_pair(account_name, essential_wallet::Scheme::Secp256k1)
    }

    pub async fn mint(&mut self, account_name: &str, hash: Hash) -> anyhow::Result<()> {
        let key = self.get_hashed_key(account_name)?;
        let hash = essential_types::convert::word_4_from_u8_32(hash);

        let mut state_key = vec![0];
        state_key.extend_from_slice(&hash);

        let solution = Solution {
            data: vec![SolutionData {
                intent_to_solve: self.deployed_intents["mint"].clone(),
                decision_variables: vec![
                    vec![hash[0]],
                    vec![hash[1]],
                    vec![hash[2]],
                    vec![hash[3]],
                    vec![key[0]],
                    vec![key[1]],
                    vec![key[2]],
                    vec![key[3]],
                ],
                transient_data: Default::default(),
                state_mutations: vec![Mutation {
                    key: state_key,
                    value: key.to_vec(),
                }],
            }],
        };
        self.client.submit_solution(solution).await?;
        Ok(())
    }

    pub async fn do_i_own(&mut self, account_name: &str, hash: Hash) -> anyhow::Result<bool> {
        let key = self.get_hashed_key(account_name)?;
        let hash = essential_types::convert::word_4_from_u8_32(hash);

        let mut state_key = vec![0];
        state_key.extend_from_slice(&hash);

        let state = self.query("transfer", &state_key).await?;
        Ok(state[..] == key[..])
    }

    pub async fn transfer(
        &mut self,
        account_name: &str,
        to: &str,
        hash: Hash,
    ) -> anyhow::Result<()> {
        let key = self.get_hashed_key(account_name)?;
        let to = self.get_hashed_key(to)?;
        let token = essential_types::convert::word_4_from_u8_32(hash);

        let mut state_key = vec![0];
        state_key.extend_from_slice(&key);

        let state = self.query("key", &state_key).await?;
        if state.is_empty() {
            // Init nonce

            // Sign key
            let mut decision_variables = vec![];
            let sig = self.wallet.sign_words(&key, account_name)?;
            let sig = match sig {
                essential_signer::Signature::Secp256k1(sig) => sig,
                _ => bail!("Invalid signature"),
            };
            let sig = essential_sign::encode::signature(&sig);

            // Currently dec vars are stored as one word each in pint.
            let iter = key.iter().map(|w| vec![*w]);
            decision_variables.extend(iter);
            let iter = sig.into_iter().map(|w| vec![w]);
            decision_variables.extend(iter);
            let k = self.get_key(account_name)?;
            let iter = k.iter().map(|w| vec![*w]);
            decision_variables.extend(iter);

            let solution = Solution {
                data: vec![SolutionData {
                    intent_to_solve: self.deployed_intents["init"].clone(),
                    decision_variables,
                    transient_data: Default::default(),
                    state_mutations: vec![Mutation {
                        key: state_key.clone(),
                        value: vec![0],
                    }],
                }],
            };
            self.client.submit_solution(solution).await?;
        }

        // Make key auth and transfer

        // Sign key, token, to
        let mut to_hash = key.to_vec();
        to_hash.extend_from_slice(&token);
        to_hash.extend_from_slice(&to);

        let sig = self.wallet.sign_words(&to_hash, account_name)?;
        let sig = match sig {
            essential_signer::Signature::Secp256k1(sig) => sig,
            _ => bail!("Invalid signature"),
        };
        let sig = essential_sign::encode::signature(&sig);

        let mut decision_variables = vec![];

        // Currently dec vars are stored as one word each in pint.
        let iter = sig.into_iter().map(|w| vec![w]);
        decision_variables.extend(iter);

        let k = self.get_key(account_name)?;
        let iter = k.iter().map(|w| vec![*w]);
        decision_variables.extend(iter);

        let transient_data = vec![
            Mutation {
                key: vec![0],
                value: key.to_vec(),
            },
            Mutation {
                key: vec![1],
                value: token.to_vec(),
            },
            Mutation {
                key: vec![2],
                value: to.to_vec(),
            },
        ];

        let mut nonce = loop {
            let nonce = self.query("key", &state_key).await?;
            if !nonce.is_empty() {
                break nonce;
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        };
        nonce[0] += 1;

        let key_auth = SolutionData {
            intent_to_solve: self.deployed_intents["key"].clone(),
            decision_variables,
            transient_data,
            state_mutations: vec![Mutation {
                key: state_key,
                value: nonce,
            }],
        };

        let mut state_key = vec![0];
        state_key.extend_from_slice(&token);

        let transfer_nft = SolutionData {
            intent_to_solve: self.deployed_intents["transfer"].clone(),
            decision_variables: vec![vec![0]],
            transient_data: vec![],
            state_mutations: vec![Mutation {
                key: state_key,
                value: to.to_vec(),
            }],
        };
        let solution = Solution {
            data: vec![key_auth, transfer_nft],
        };
        self.client.submit_solution(solution).await?;

        Ok(())
    }

    async fn query(&self, name: &str, key: &[Word]) -> anyhow::Result<Vec<Word>> {
        let state = self
            .client
            .query_state(&self.deployed_intents.get(name).unwrap().set, &key.to_vec())
            .await?;
        Ok(state)
    }

    fn get_hashed_key(&mut self, account_name: &str) -> anyhow::Result<[Word; 4]> {
        let public_key = self.wallet.get_public_key(account_name)?;
        let essential_signer::PublicKey::Secp256k1(public_key) = public_key else {
            bail!("Invalid public key")
        };
        let encoded = essential_sign::encode::public_key(&public_key);
        Ok(word_4_from_u8_32(essential_hash::hash_words(&encoded)))
    }

    fn get_key(&mut self, account_name: &str) -> anyhow::Result<[Word; 5]> {
        let public_key = self.wallet.get_public_key(account_name)?;
        let essential_signer::PublicKey::Secp256k1(public_key) = public_key else {
            bail!("Invalid public key")
        };
        Ok(essential_sign::encode::public_key(&public_key))
    }
}
