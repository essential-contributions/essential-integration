use std::vec;

use anyhow::bail;
use essential_rest_client::EssentialClient;
use essential_types::{
    convert::word_4_from_u8_32,
    solution::{Mutation, Solution, SolutionData},
    ContentAddress, Hash, IntentAddress, Word,
};

pub struct Nft {
    client: EssentialClient,
    wallet: essential_wallet::Wallet,
    deployed_intents: Addresses,
}

#[derive(Debug, Clone)]
pub struct Addresses {
    pub nft: ContentAddress,
    pub nft_mint: IntentAddress,
    pub nft_transfer: IntentAddress,
    pub auth: ContentAddress,
    pub auth_auth: IntentAddress,
    pub key: ContentAddress,
    pub key_init: IntentAddress,
    pub key_key: IntentAddress,
    pub swap_any: ContentAddress,
    pub swap_any_init: IntentAddress,
    pub swap_any_swap: IntentAddress,
}

impl Nft {
    pub fn new(
        addr: String,
        deployed_intents: Addresses,
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

    async fn mint_inner(&mut self, key: [Word; 4], token: Hash) -> anyhow::Result<()> {
        let token = essential_types::convert::word_4_from_u8_32(token);

        let mut state_key = vec![0];
        state_key.extend_from_slice(&token);

        let solution = Solution {
            data: vec![SolutionData {
                intent_to_solve: self.deployed_intents.nft_mint.clone(),
                decision_variables: vec![
                    vec![token[0]],
                    vec![token[1]],
                    vec![token[2]],
                    vec![token[3]],
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

    pub async fn mint(&mut self, account_name: &str, token: Hash) -> anyhow::Result<()> {
        let key = self.get_hashed_key(account_name)?;
        self.mint_inner(key, token).await
    }

    pub async fn mint_for_contract(
        &mut self,
        contract: &IntentAddress,
        token: Hash,
    ) -> anyhow::Result<()> {
        let key = contract_hash(contract);
        self.mint_inner(key, token).await
    }

    async fn do_i_own_inner(&mut self, key: [Word; 4], hash: Hash) -> anyhow::Result<bool> {
        let hash = essential_types::convert::word_4_from_u8_32(hash);

        let mut state_key = vec![0];
        state_key.extend_from_slice(&hash);

        let state = self.query(&self.deployed_intents.nft, &state_key).await?;
        Ok(state[..] == key[..])
    }

    pub async fn do_i_own(&mut self, account_name: &str, hash: Hash) -> anyhow::Result<bool> {
        let key = self.get_hashed_key(account_name)?;
        self.do_i_own_inner(key, hash).await
    }

    pub async fn do_i_own_contract(
        &mut self,
        contract: &IntentAddress,
        hash: Hash,
    ) -> anyhow::Result<bool> {
        let key = contract_hash(contract);
        self.do_i_own_inner(key, hash).await
    }

    pub async fn init_swap_any(&mut self, token: Hash) -> anyhow::Result<()> {
        let token = essential_types::convert::word_4_from_u8_32(token);

        let state_key = vec![0];

        let solution = Solution {
            data: vec![SolutionData {
                intent_to_solve: self.deployed_intents.swap_any_init.clone(),
                decision_variables: Default::default(),
                transient_data: Default::default(),
                state_mutations: vec![Mutation {
                    key: state_key.clone(),
                    value: token.to_vec(),
                }],
            }],
        };
        self.client.submit_solution(solution).await?;

        Ok(())
    }

    pub async fn swap_any_owns(&mut self) -> anyhow::Result<Option<Hash>> {
        let state_key = vec![0];

        let state = self
            .query(&self.deployed_intents.swap_any, &state_key)
            .await?;

        if state.is_empty() {
            return Ok(None);
        }

        let token = essential_types::convert::u8_32_from_word_4(
            state
                .try_into()
                .map_err(|_| anyhow::anyhow!("Bad token state"))?,
        );

        Ok(Some(token))
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

        self.initialize_nonce(account_name, key).await?;

        // Make key auth and transfer
        let solution = self
            .make_transfer_solution(account_name, key, to, token)
            .await?;

        self.client.submit_solution(solution).await?;

        Ok(())
    }

    async fn initialize_nonce(&mut self, account_name: &str, key: [Word; 4]) -> anyhow::Result<()> {
        let mut state_key = vec![0];
        state_key.extend_from_slice(&key);

        let state = self.query(&self.deployed_intents.key, &state_key).await?;
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
            let iter = sig.into_iter().map(|w| vec![w]);
            decision_variables.extend(iter);
            let iter = key.iter().map(|w| vec![*w]);
            decision_variables.extend(iter);
            let k = self.get_key(account_name)?;
            let iter = k.iter().map(|w| vec![*w]);
            decision_variables.extend(iter);

            let solution = Solution {
                data: vec![SolutionData {
                    intent_to_solve: self.deployed_intents.key_init.clone(),
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
        Ok(())
    }

    async fn make_transfer_solution(
        &mut self,
        account_name: &str,
        key: [Word; 4],
        to: [Word; 4],
        token: [Word; 4],
    ) -> anyhow::Result<Solution> {
        let mut state_key = vec![0];
        state_key.extend_from_slice(&key);

        let mut nonce = loop {
            let nonce = self.query(&self.deployed_intents.key, &state_key).await?;
            if !nonce.is_empty() {
                break nonce;
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        };
        nonce[0] += 1;

        // Sign key, token, to
        let mut to_hash = key.to_vec();
        to_hash.extend_from_slice(&token);
        to_hash.extend_from_slice(&to);
        to_hash.push(nonce[0]);

        let sig = self.wallet.sign_words(&to_hash, account_name)?;
        let sig = match sig {
            essential_signer::Signature::Secp256k1(sig) => sig,
            _ => bail!("Invalid signature"),
        };
        let sig = essential_sign::encode::signature(&sig);

        let mut decision_variables = vec![];

        decision_variables.push(vec![nonce[0]]);

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

        let key_auth = SolutionData {
            intent_to_solve: self.deployed_intents.key_key.clone(),
            decision_variables,
            transient_data: transient_data.clone(),
            state_mutations: vec![Mutation {
                key: state_key,
                value: nonce,
            }],
        };

        let mut decision_variables = word_4_from_u8_32(self.deployed_intents.key.0)
            .iter()
            .copied()
            .chain(
                word_4_from_u8_32(self.deployed_intents.key_key.intent.0)
                    .iter()
                    .copied(),
            )
            .map(|w| vec![w])
            .collect::<Vec<_>>();

        // Pathway to key intent
        decision_variables.push(vec![0]);

        let auth = SolutionData {
            intent_to_solve: self.deployed_intents.auth_auth.clone(),
            decision_variables,
            transient_data,
            state_mutations: vec![],
        };

        let mut state_key = vec![0];
        state_key.extend_from_slice(&token);

        let transfer_nft = SolutionData {
            intent_to_solve: self.deployed_intents.nft_transfer.clone(),
            // Pathway to the auth intent
            decision_variables: vec![vec![1]],
            transient_data: vec![],
            state_mutations: vec![Mutation {
                key: state_key,
                value: to.to_vec(),
            }],
        };
        Ok(Solution {
            data: vec![key_auth, auth, transfer_nft],
        })
    }

    pub async fn swap_with_contract(
        &mut self,
        account_name: &str,
        token: Hash,
    ) -> anyhow::Result<()> {
        let key = self.get_hashed_key(account_name)?;
        let to = contract_hash(&self.deployed_intents.swap_any_swap);
        let token = essential_types::convert::word_4_from_u8_32(token);

        self.initialize_nonce(account_name, key).await?;

        let mut solution = self
            .make_transfer_solution(account_name, key, to, token)
            .await?;

        // Get existing token
        let current_token = self.query(&self.deployed_intents.swap_any, &[0]).await?;

        let transient_data = vec![
            Mutation {
                key: vec![0],
                value: to.to_vec(),
            },
            Mutation {
                key: vec![1],
                value: current_token.to_vec(),
            },
            Mutation {
                key: vec![2],
                value: key.to_vec(),
            },
        ];

        let swap_any_swap = SolutionData {
            intent_to_solve: self.deployed_intents.swap_any_swap.clone(),
            decision_variables: Default::default(),
            transient_data: transient_data.clone(),
            state_mutations: vec![Mutation {
                key: vec![0],
                value: token.to_vec(),
            }],
        };

        let mut decision_variables = word_4_from_u8_32(self.deployed_intents.swap_any.0)
            .iter()
            .copied()
            .chain(
                word_4_from_u8_32(self.deployed_intents.swap_any_swap.intent.0)
                    .iter()
                    .copied(),
            )
            .map(|w| vec![w])
            .collect::<Vec<_>>();

        // Pathway to swap_any_swap intent
        decision_variables.push(vec![3]);

        let auth = SolutionData {
            intent_to_solve: self.deployed_intents.auth_auth.clone(),
            decision_variables,
            transient_data,
            state_mutations: vec![],
        };

        // Transfer existing token from swap_any to user

        let mut state_key = vec![0];
        state_key.extend(current_token);

        let transfer_nft = SolutionData {
            intent_to_solve: self.deployed_intents.nft_transfer.clone(),
            // Pathway to the auth intent
            decision_variables: vec![vec![4]],
            transient_data: vec![],
            state_mutations: vec![Mutation {
                key: state_key,
                value: key.to_vec(),
            }],
        };

        solution.data.push(swap_any_swap);
        solution.data.push(auth);
        solution.data.push(transfer_nft);

        self.client.submit_solution(solution).await?;
        Ok(())
    }

    async fn query(&self, set_address: &ContentAddress, key: &[Word]) -> anyhow::Result<Vec<Word>> {
        let state = self.client.query_state(set_address, &key.to_vec()).await?;
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

fn contract_hash(contract: &IntentAddress) -> [Word; 4] {
    let set_hash = essential_types::convert::word_4_from_u8_32(contract.set.0);
    let intent_hash = essential_types::convert::word_4_from_u8_32(contract.intent.0);
    let mut words = set_hash.to_vec();
    words.extend_from_slice(&intent_hash);

    let contract_hash = essential_hash::hash_words(&words);

    essential_types::convert::word_4_from_u8_32(contract_hash)
}
