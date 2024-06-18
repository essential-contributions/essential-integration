use crate::inputs::{self, token::query_balances};
use anyhow::bail;
use app_utils::inputs::Instance;
use essential_rest_client::EssentialClient;
use essential_server_types::SolutionOutcome;
use essential_types::{
    convert::word_4_from_u8_32,
    solution::{Solution, SolutionData},
    ContentAddress, IntentAddress, Word,
};
use essential_wallet::Wallet;

pub struct Token {
    client: EssentialClient,
    wallet: Wallet,
    deployed_intents: Addresses,
}

#[derive(Debug, Clone)]
pub struct Addresses {
    pub token: ContentAddress,
    pub burn: IntentAddress,
    pub mint: IntentAddress,
    pub transfer: IntentAddress,
    pub signed: ContentAddress,
    pub signed_cancel: IntentAddress,
    pub signed_transfer: IntentAddress,
    pub signed_transfer_from_to: IntentAddress,
    pub signed_transfer_from: IntentAddress,
}

impl Token {
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

    pub async fn burn(
        &mut self,
        account_name: &str,
        amount: i64,
    ) -> anyhow::Result<ContentAddress> {
        let key = self.get_hashed_key(account_name)?;
        self.burn_inner(account_name, key, amount).await
    }

    pub async fn mint(
        &mut self,
        account_name: &str,
        amount: Word,
    ) -> anyhow::Result<ContentAddress> {
        let key = self.get_hashed_key(account_name)?;
        self.mint_inner(key, amount).await
    }

    pub async fn transfer(
        &mut self,
        from_name: &str,
        to_name: &str,
        amount: i64,
    ) -> anyhow::Result<ContentAddress> {
        let from_key = self.get_hashed_key(from_name)?;
        let to_key = self.get_hashed_key(to_name)?;
        self.transfer_inner(from_name, from_key, to_key, amount)
            .await
    }

    pub async fn solution_outcome(
        &mut self,
        solution_address: ContentAddress,
    ) -> anyhow::Result<Vec<SolutionOutcome>> {
        self.client.solution_outcome(&solution_address.0).await
    }

    pub async fn balance(&mut self, account_name: &str) -> anyhow::Result<Option<i64>> {
        let key = self.get_hashed_key(account_name)?;
        let state = self
            .query(&self.deployed_intents.token, &query_balances(key.into()))
            .await?;
        Ok(state.first().copied())
    }

    pub async fn nonce(&mut self, key: &[Word; 4]) -> anyhow::Result<Word> {
        let nonce = self
            .query(
                &self.deployed_intents.signed,
                &inputs::signed::query_nonce((*key).into()),
            )
            .await?;
        Ok(nonce.first().copied().unwrap_or_default())
    }

    async fn query(&self, set_address: &ContentAddress, key: &[Word]) -> anyhow::Result<Vec<Word>> {
        let state = self.client.query_state(set_address, &key.to_vec()).await?;
        Ok(state)
    }

    async fn burn_inner(
        &mut self,
        account_name: &str,
        key: [Word; 4],
        amount: Word,
    ) -> anyhow::Result<ContentAddress> {
        let mut nonce = self.nonce(&key).await?;
        nonce += 1;

        let state = self
            .query(&self.deployed_intents.token, &query_balances(key.into()))
            .await?;
        let from_balance = state.first().copied().unwrap_or_default();
        let Some(from_new_balance) = from_balance.checked_sub(amount) else {
            bail!("Insufficient balance")
        };

        // Burn doesn't send to anyone
        let to = [0; 4];

        let decision_variables = inputs::token::burn::DecVars {
            auth_addr: Instance {
                address: self.deployed_intents.signed_transfer.clone(),
                path: 1.into(),
            },
        };
        let mutation = inputs::token::balances(key.into(), from_new_balance.into());
        let token_burn = SolutionData {
            intent_to_solve: self.deployed_intents.burn.clone(),
            decision_variables: decision_variables.encode(),
            transient_data: Default::default(),
            state_mutations: vec![mutation],
        };

        let instance = Instance {
            address: self.deployed_intents.burn.clone(),
            path: 0.into(),
        };
        let sig = self.hash_transfer(account_name, &key, &to, amount, nonce, instance.clone())?;

        let decision_variables = inputs::signed::transfer::DecVars {
            sig,
            public_key: self.get_pub_key(account_name)?,
        };

        let transient_data = inputs::signed::transfer::TransientData {
            key: key.into(),
            to: to.into(),
            amount: amount.into(),
            set: instance.address.set.clone().into(),
            intent_addr: instance.address.intent.clone().into(),
            path: instance.path.into(),
        };

        let mutation = inputs::signed::nonce(key.into(), nonce.into());
        let signed_transfer = SolutionData {
            intent_to_solve: self.deployed_intents.signed_transfer.clone(),
            decision_variables: decision_variables.encode(),
            transient_data: transient_data.encode(),
            state_mutations: vec![mutation],
        };

        let solution = Solution {
            data: vec![token_burn, signed_transfer],
        };
        self.client.submit_solution(solution).await
    }

    async fn mint_inner(
        &mut self,
        key: [Word; 4],
        balance: Word,
    ) -> anyhow::Result<ContentAddress> {
        let decision_variables = inputs::token::mint::DecVars {
            token_name: [0; 4].into(),
            token_symbol: [0; 4].into(),
            decimals: 18.into(),
        };
        let bal_mutation = inputs::token::balances(key.into(), balance.into());
        let init_mutation = inputs::token::init();
        let solution = Solution {
            data: vec![SolutionData {
                intent_to_solve: self.deployed_intents.mint.clone(),
                decision_variables: decision_variables.encode(),
                transient_data: Default::default(),
                state_mutations: vec![bal_mutation, init_mutation],
            }],
        };
        self.client.submit_solution(solution).await
    }

    async fn transfer_inner(
        &mut self,
        account_name: &str,
        key: [Word; 4],
        to: [Word; 4],
        amount: Word,
    ) -> anyhow::Result<ContentAddress> {
        let mut nonce = self.nonce(&key).await?;
        nonce += 1;

        let decision_variables = inputs::token::transfer::DecVars {
            auth_addr: Instance {
                address: self.deployed_intents.signed_transfer.clone(),
                path: 1.into(),
            },
        };
        let state = self
            .query(&self.deployed_intents.token, &query_balances(key.into()))
            .await?;
        let from_balance = state.first().copied().unwrap_or_default();
        let Some(from_new_balance) = from_balance.checked_sub(amount) else {
            bail!("Insufficient balance")
        };
        let state = self
            .query(&self.deployed_intents.token, &query_balances(to.into()))
            .await?;
        let to_balance = state.first().copied().unwrap_or_default();
        let Some(to_new_balance) = to_balance.checked_add(amount) else {
            bail!("Overflow error")
        };

        let from_mutation = inputs::token::balances(key.into(), from_new_balance.into());
        let to_mutation = inputs::token::balances(to.into(), to_new_balance.into());

        let token_transfer = SolutionData {
            intent_to_solve: self.deployed_intents.transfer.clone(),
            decision_variables: decision_variables.encode(),
            transient_data: Default::default(),
            state_mutations: vec![from_mutation, to_mutation],
        };

        let instance = Instance {
            address: self.deployed_intents.transfer.clone(),
            path: 0.into(),
        };
        let sig = self.hash_transfer(account_name, &key, &to, amount, nonce, instance.clone())?;

        let decision_variables = inputs::signed::transfer::DecVars {
            sig,
            public_key: self.get_pub_key(account_name)?,
        };

        let transient_data = inputs::signed::transfer::TransientData {
            key: key.into(),
            to: to.into(),
            amount: amount.into(),
            set: instance.address.set.clone().into(),
            intent_addr: instance.address.intent.clone().into(),
            path: instance.path.into(),
        };

        let mutation = inputs::signed::nonce(key.into(), nonce.into());
        let signed_transfer = SolutionData {
            intent_to_solve: self.deployed_intents.signed_transfer.clone(),
            decision_variables: decision_variables.encode(),
            transient_data: transient_data.encode(),
            state_mutations: vec![mutation],
        };

        let solution = Solution {
            data: vec![token_transfer, signed_transfer],
        };
        self.client.submit_solution(solution).await
    }

    fn hash_transfer(
        &mut self,
        account_name: &str,
        key: &[Word; 4],
        to: &[Word; 4],
        amount: Word,
        nonce: Word,
        instance: Instance,
    ) -> anyhow::Result<essential_signer::secp256k1::ecdsa::RecoverableSignature> {
        // Sign key, to, anount
        let mut to_hash = key.to_vec();
        to_hash.extend_from_slice(to);
        to_hash.push(amount);
        to_hash.push(nonce);

        // Sign the token transfer instance
        to_hash.extend(word_4_from_u8_32(instance.address.set.0));
        to_hash.extend(word_4_from_u8_32(instance.address.intent.0));
        to_hash.push(instance.path);

        let sig = self.wallet.sign_words(&to_hash, account_name)?;
        let sig = match sig {
            essential_signer::Signature::Secp256k1(sig) => sig,
            _ => bail!("Invalid signature"),
        };
        Ok(sig)
    }

    fn get_hashed_key(&mut self, account_name: &str) -> anyhow::Result<[Word; 4]> {
        let public_key = self.wallet.get_public_key(account_name)?;
        let essential_signer::PublicKey::Secp256k1(public_key) = public_key else {
            anyhow::bail!("Invalid public key")
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
