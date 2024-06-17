use anyhow::bail;
use app_utils::{
    addresses::{contract_hash, get_addresses, replace_intent_address, replace_set_address},
    compile::compile_pint_file,
    print::{print_intent_address, print_set_address},
};
use essential_rest_client::EssentialClient;
use essential_types::{
    convert::word_4_from_u8_32,
    solution::{Mutation, Solution, SolutionData},
    ContentAddress, Hash, IntentAddress, Word,
};
use inputs::nft::query_owners;
use std::{path::PathBuf, vec};

mod inputs;

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
        let decision_variables = inputs::nft::mint::DecVars {
            token: token.into(),
            new_owner: key.into(),
        };

        let mutation = inputs::nft::owners(token.into(), key.into());

        let solution = Solution {
            data: vec![SolutionData {
                intent_to_solve: self.deployed_intents.nft_mint.clone(),
                decision_variables: decision_variables.encode(),
                transient_data: Default::default(),
                state_mutations: vec![mutation],
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
        let state = self
            .query(&self.deployed_intents.nft, &query_owners(hash.into()))
            .await?;
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
        let solution = Solution {
            data: vec![SolutionData {
                intent_to_solve: self.deployed_intents.swap_any_init.clone(),
                decision_variables: Default::default(),
                transient_data: Default::default(),
                state_mutations: vec![inputs::swap_any::token(token.into())],
            }],
        };
        self.client.submit_solution(solution).await?;

        Ok(())
    }

    pub async fn swap_any_owns(&mut self) -> anyhow::Result<Option<Hash>> {
        let state = self
            .query(
                &self.deployed_intents.swap_any,
                &inputs::swap_any::query_token(),
            )
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

        // Make key auth and transfer
        let solution = self
            .make_transfer_solution(account_name, key, to, token)
            .await?;

        self.client.submit_solution(solution).await?;

        Ok(())
    }

    async fn make_transfer_solution(
        &mut self,
        account_name: &str,
        key: [Word; 4],
        to: [Word; 4],
        token: [Word; 4],
    ) -> anyhow::Result<Solution> {
        let nonce = self
            .query(
                &self.deployed_intents.key,
                &inputs::key::query_nonce(key.into()),
            )
            .await?;
        let mut nonce = nonce.first().copied().unwrap_or_default();
        nonce += 1;

        // Sign key, token, to
        let mut to_hash = key.to_vec();
        to_hash.extend_from_slice(&token);
        to_hash.extend_from_slice(&to);
        to_hash.push(nonce);
        to_hash.extend(word_4_from_u8_32(self.deployed_intents.nft.0));
        to_hash.extend(word_4_from_u8_32(
            self.deployed_intents.nft_transfer.intent.0,
        ));
        to_hash.push(2);

        let sig = self.wallet.sign_words(&to_hash, account_name)?;
        let sig = match sig {
            essential_signer::Signature::Secp256k1(sig) => sig,
            _ => bail!("Invalid signature"),
        };

        let decision_variables = inputs::key::key::DecVars {
            new_nonce: nonce.into(),
            sig,
            public_key: self.get_pub_key(account_name)?,
        };

        let transient_data = inputs::key::key::TransientData {
            key: key.into(),
            token: token.into(),
            to: to.into(),
            set: self.deployed_intents.nft.clone().into(),
            intent_addr: self.deployed_intents.nft_transfer.intent.clone().into(),
            path: 2.into(),
        };

        let key_auth = SolutionData {
            intent_to_solve: self.deployed_intents.key_key.clone(),
            decision_variables: decision_variables.encode(),
            transient_data: transient_data.encode(),
            state_mutations: vec![inputs::key::nonce(key.into(), nonce.into())],
        };

        let decision_variables = inputs::auth::DecVars {
            auth_addr: self.deployed_intents.key_key.clone(),
            authi_auth_pathway: 0.into(),
        };

        let auth = SolutionData {
            intent_to_solve: self.deployed_intents.auth_auth.clone(),
            decision_variables: decision_variables.encode(),
            transient_data: transient_data.encode(),
            state_mutations: vec![],
        };

        let transfer_nft = SolutionData {
            intent_to_solve: self.deployed_intents.nft_transfer.clone(),
            // Pathway to the auth intent
            decision_variables: vec![vec![1]],
            transient_data: vec![],
            state_mutations: vec![inputs::nft::owners(token.into(), to.into())],
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

        let mut solution = self
            .make_transfer_solution(account_name, key, to, token)
            .await?;

        // Get existing token
        let current_token = self
            .query(
                &self.deployed_intents.swap_any,
                &inputs::swap_any::query_token(),
            )
            .await?;

        let Ok(current_token): Result<[Word; 4], _> = current_token.try_into() else {
            bail!("Bad token state")
        };

        let transient_data = inputs::swap_any::swap::TransientData {
            key: to.into(),
            token: current_token.into(),
            to: key.into(),
            set: self.deployed_intents.nft.clone().into(),
            intent_addr: self.deployed_intents.nft_transfer.intent.clone().into(),
            path: 5.into(),
        };

        let swap_any_swap = SolutionData {
            intent_to_solve: self.deployed_intents.swap_any_swap.clone(),
            decision_variables: Default::default(),
            transient_data: transient_data.encode(),
            state_mutations: vec![Mutation {
                key: vec![0],
                value: token.to_vec(),
            }],
        };

        let decision_variables = inputs::auth::DecVars {
            auth_addr: self.deployed_intents.swap_any_swap.clone(),
            // Pathway to swap_any_swap intent
            authi_auth_pathway: 3.into(),
        };

        let auth = SolutionData {
            intent_to_solve: self.deployed_intents.auth_auth.clone(),
            decision_variables: decision_variables.encode(),
            transient_data: transient_data.encode(),
            state_mutations: vec![],
        };

        // Transfer existing token from swap_any to user
        let transfer_nft = SolutionData {
            intent_to_solve: self.deployed_intents.nft_transfer.clone(),
            // Pathway to the auth intent
            decision_variables: inputs::nft::transfer::DecVars {
                auth_auth_pathway: 4.into(),
            }
            .encode(),
            transient_data: vec![],
            state_mutations: vec![inputs::nft::owners(current_token.into(), key.into())],
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

    fn get_pub_key(
        &mut self,
        account_name: &str,
    ) -> anyhow::Result<essential_signer::secp256k1::PublicKey> {
        let public_key = self.wallet.get_public_key(account_name)?;
        let essential_signer::PublicKey::Secp256k1(public_key) = public_key else {
            bail!("Invalid public key")
        };
        Ok(public_key)
    }
}

pub async fn deploy_app(
    addr: String,
    wallet: &mut essential_wallet::Wallet,
    account_name: &str,
    pint_directory: PathBuf,
) -> anyhow::Result<Addresses> {
    let client = EssentialClient::new(addr)?;
    let key_intents = compile_pint_file(pint_directory.clone(), "key.pnt").await?;
    let key_addresses = get_addresses(&key_intents);

    let nft_intents = compile_pint_file(pint_directory.clone(), "nft.pnt").await?;
    let nft_addresses = get_addresses(&nft_intents);

    let auth_intents = compile_pint_file(pint_directory.clone(), "auth.pnt").await?;
    let auth_addresses = get_addresses(&auth_intents);

    let swap_any_intents = compile_pint_file(pint_directory.clone(), "swap_any.pnt").await?;
    let swap_any_addresses = get_addresses(&swap_any_intents);

    let addresses = Addresses {
        nft: nft_addresses.0.clone(),
        nft_mint: nft_addresses.1[0].clone(),
        nft_transfer: nft_addresses.1[1].clone(),
        auth: auth_addresses.0.clone(),
        auth_auth: auth_addresses.1[0].clone(),
        key: key_addresses.0.clone(),
        key_key: key_addresses.1[0].clone(),
        swap_any: swap_any_addresses.0.clone(),
        swap_any_init: swap_any_addresses.1[0].clone(),
        swap_any_swap: swap_any_addresses.1[1].clone(),
    };

    let intents = wallet.sign_intent_set(nft_intents, account_name)?;
    client.deploy_intent_set(intents).await?;
    let intents = wallet.sign_intent_set(key_intents, account_name)?;
    client.deploy_intent_set(intents).await?;
    let intents = wallet.sign_intent_set(auth_intents, account_name)?;
    client.deploy_intent_set(intents).await?;
    let intents = wallet.sign_intent_set(swap_any_intents, account_name)?;
    client.deploy_intent_set(intents).await?;

    Ok(addresses)
}

pub async fn compile_addresses(pint_directory: PathBuf) -> anyhow::Result<Addresses> {
    let key_intents = compile_pint_file(pint_directory.clone(), "key.pnt").await?;
    let key_addresses = get_addresses(&key_intents);

    let nft_intents = compile_pint_file(pint_directory.clone(), "nft.pnt").await?;
    let nft_addresses = get_addresses(&nft_intents);

    let auth_intents = compile_pint_file(pint_directory.clone(), "auth.pnt").await?;
    let auth_addresses = get_addresses(&auth_intents);

    let swap_any_intents = compile_pint_file(pint_directory.clone(), "swap_any.pnt").await?;
    let swap_any_addresses = get_addresses(&swap_any_intents);

    let addresses = Addresses {
        nft: nft_addresses.0.clone(),
        nft_mint: nft_addresses.1[0].clone(),
        nft_transfer: nft_addresses.1[1].clone(),
        auth: auth_addresses.0.clone(),
        auth_auth: auth_addresses.1[0].clone(),
        key: key_addresses.0.clone(),
        key_key: key_addresses.1[0].clone(),
        swap_any: swap_any_addresses.0.clone(),
        swap_any_init: swap_any_addresses.1[0].clone(),
        swap_any_swap: swap_any_addresses.1[1].clone(),
    };

    Ok(addresses)
}

pub fn print_addresses(addresses: &Addresses) {
    let Addresses {
        nft,
        nft_mint,
        nft_transfer,
        auth,
        auth_auth,
        key,
        key_key,
        swap_any,
        swap_any_init,
        swap_any_swap,
    } = addresses;
    print_set_address("nft", nft);
    print_intent_address("nft_mint", nft_mint);
    print_intent_address("nft_transfer", nft_transfer);
    print_set_address("auth", auth);
    print_intent_address("auth_auth", auth_auth);
    print_set_address("key", key);
    print_intent_address("key_key", key_key);
    print_set_address("swap_any", swap_any);
    print_intent_address("swap_any_init", swap_any_init);
    print_intent_address("swap_any_swap", swap_any_swap);
}

pub async fn update_addresses(pint_directory: PathBuf) -> anyhow::Result<()> {
    let addresses = compile_addresses(pint_directory.clone()).await?;

    replace_intent_address(pint_directory.clone(), "auth.pnt", &addresses.key_key).await?;

    replace_intent_address(pint_directory.clone(), "nft.pnt", &addresses.auth_auth).await?;

    replace_set_address(pint_directory, "swap_any.pnt", &addresses.nft).await?;

    Ok(())
}
