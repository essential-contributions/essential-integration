use std::collections::HashMap;

use anyhow::bail;
use essential_rest_client::EssentialClient;
use essential_types::{
    solution::{Mutation, Solution, SolutionData},
    Hash, IntentAddress,
};

pub struct Nft {
    client: EssentialClient,
    deployed_intents: HashMap<String, IntentAddress>,
}

impl Nft {
    pub fn new(
        addr: String,
        deployed_intents: HashMap<String, IntentAddress>,
    ) -> anyhow::Result<Self> {
        let client = EssentialClient::new(addr)?;
        Ok(Self {
            client,
            deployed_intents,
        })
    }

    pub fn create_account(&self, account_name: &str) -> anyhow::Result<()> {
        essential_wallet::new_key_pair(
            account_name.to_string(),
            essential_wallet::Scheme::Secp256k1,
        )
    }

    pub async fn mint(&self, account_name: &str, hash: Hash) -> anyhow::Result<()> {
        let public_key = essential_wallet::get_public_key(account_name)?;
        let essential_signer::PublicKey::Secp256k1(public_key) = public_key else {
            bail!("Invalid public key")
        };
        let key = essential_signer::hash_bytes(public_key.serialize().as_ref())?;
        let key = essential_types::convert::word_4_from_u8_32(key);
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

    pub async fn do_i_own(&self, account_name: &str, hash: Hash) -> anyhow::Result<bool> {
        let public_key = essential_wallet::get_public_key(account_name)?;
        let essential_signer::PublicKey::Secp256k1(public_key) = public_key else {
            bail!("Invalid public key")
        };
        let key = essential_signer::hash_bytes(public_key.serialize().as_ref())?;
        let key = essential_types::convert::word_4_from_u8_32(key);
        let hash = essential_types::convert::word_4_from_u8_32(hash);

        let mut state_key = vec![0];
        state_key.extend_from_slice(&hash);

        let state = self
            .client
            .query_state(
                &self.deployed_intents.values().next().unwrap().set,
                &state_key,
            )
            .await?;
        Ok(state[..] == key[..])
    }
}
