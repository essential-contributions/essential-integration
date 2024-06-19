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
    pub cancel: IntentAddress,
    pub signed: ContentAddress,
    pub signed_transfer: IntentAddress,
    pub signed_transfer_from: IntentAddress,
    pub signed_mint: IntentAddress,
    pub signed_burn: IntentAddress,
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
        self.mint_inner(account_name, key, amount).await
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
                &self.deployed_intents.token,
                &inputs::token::query_nonce((*key).into()),
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

        let transient_data = inputs::token::burn::TransientData {
            key: key.into(),
            amount: amount.into(),
        };

        let decision_variables = inputs::token::burn::DecVars {
            auth_addr: Instance {
                address: self.deployed_intents.signed_burn.clone(),
                path: 1.into(),
            },
        };
        let burn_mutation = inputs::token::balances(key.into(), from_new_balance.into());
        let nonce_mutation = inputs::token::nonce(key.into(), nonce.into());
        let token_burn = SolutionData {
            intent_to_solve: self.deployed_intents.burn.clone(),
            decision_variables: decision_variables.encode(),
            transient_data: transient_data.encode(),
            state_mutations: vec![burn_mutation, nonce_mutation],
        };

        let instance = Instance {
            address: self.deployed_intents.burn.clone(),
            path: 0.into(),
        };

        let mut data = key.to_vec();
        data.push(amount);
        let sig = self.hash_data(account_name, data, nonce, instance.address.clone())?;

        let decision_variables = inputs::signed::burn::DecVars {
            token_path: instance.path.into(),
            sig,
            public_key: self.get_pub_key(account_name)?,
        };

        let transient_data = inputs::signed::burn::TransientData {
            token_address: instance.address,
        };

        let signed_transfer = SolutionData {
            intent_to_solve: self.deployed_intents.signed_burn.clone(),
            decision_variables: decision_variables.encode(),
            transient_data: transient_data.encode(),
            state_mutations: vec![],
        };

        let solution = Solution {
            data: vec![token_burn, signed_transfer],
        };
        self.client.submit_solution(solution).await
    }

    async fn mint_inner(
        &mut self,
        account_name: &str,
        key: [Word; 4],
        balance: Word,
    ) -> anyhow::Result<ContentAddress> {
        let mut nonce = self.nonce(&key).await?;
        nonce += 1;

        let auth_instance = Instance {
            address: self.deployed_intents.signed_mint.clone(),
            path: 1.into(),
        };

        let decision_variables = inputs::token::mint::DecVars {
            name: [0; 4].into(),
            symbol: [0; 4].into(),
            auth_addr: auth_instance.clone(),
        };

        let transient_data = inputs::token::mint::TransientData {
            key: key.into(),
            amount: balance.into(),
            decimals: 18.into(),
        };

        let bal_mutation = inputs::token::balances(key.into(), balance.into());
        let name_mutation = inputs::token::token_name(decision_variables.name);
        let symbol_mutation = inputs::token::token_symbol(decision_variables.symbol);
        let decimals_mutation = inputs::token::decimals(18.into());
        let nonce_mutation = inputs::token::nonce(key.into(), nonce.into());
        let mint = SolutionData {
            intent_to_solve: self.deployed_intents.mint.clone(),
            decision_variables: decision_variables.encode(),
            transient_data: transient_data.encode(),
            state_mutations: vec![
                bal_mutation,
                name_mutation,
                symbol_mutation,
                nonce_mutation,
                decimals_mutation,
            ],
        };

        let mut data = key.to_vec();
        data.push(balance);
        data.push(transient_data.decimals.0);
        let sig = self.hash_data(
            account_name,
            data,
            nonce,
            self.deployed_intents.mint.clone(),
        )?;

        let decision_variables = inputs::signed::mint::DecVars {
            token_path: 0.into(),
            sig,
            public_key: self.get_pub_key(account_name)?,
        };

        let transient_data = inputs::signed::mint::TransientData {
            token_address: self.deployed_intents.mint.clone(),
        };

        let mint_auth = SolutionData {
            intent_to_solve: self.deployed_intents.signed_mint.clone(),
            decision_variables: decision_variables.encode(),
            transient_data: transient_data.encode(),
            state_mutations: vec![],
        };

        let solution = Solution {
            data: vec![mint, mint_auth],
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

        let transient_data = inputs::token::transfer::TransientData {
            key: key.into(),
            to: to.into(),
            amount: amount.into(),
        };

        let from_mutation = inputs::token::balances(key.into(), from_new_balance.into());
        let to_mutation = inputs::token::balances(to.into(), to_new_balance.into());
        let nonce_mutation = inputs::token::nonce(key.into(), nonce.into());

        let token_transfer = SolutionData {
            intent_to_solve: self.deployed_intents.transfer.clone(),
            decision_variables: decision_variables.encode(),
            transient_data: transient_data.encode(),
            state_mutations: vec![from_mutation, to_mutation, nonce_mutation],
        };

        let instance = Instance {
            address: self.deployed_intents.transfer.clone(),
            path: 0.into(),
        };

        let mut data = key.to_vec();
        data.extend(to);
        data.push(amount);

        let sig = self.hash_data(
            account_name,
            data,
            nonce,
            self.deployed_intents.transfer.clone(),
        )?;

        let decision_variables = inputs::signed::transfer::DecVars {
            sig,
            public_key: self.get_pub_key(account_name)?,
            token_path: instance.path.into(),
        };

        let transient_data = inputs::signed::transfer::TransientData {
            token_address: instance.address,
        };

        let signed_transfer = SolutionData {
            intent_to_solve: self.deployed_intents.signed_transfer.clone(),
            decision_variables: decision_variables.encode(),
            transient_data: transient_data.encode(),
            state_mutations: vec![],
        };

        let solution = Solution {
            data: vec![token_transfer, signed_transfer],
        };
        self.client.submit_solution(solution).await
    }

    fn hash_data(
        &mut self,
        account_name: &str,
        mut data: Vec<Word>,
        nonce: Word,
        address: IntentAddress,
    ) -> anyhow::Result<essential_signer::secp256k1::ecdsa::RecoverableSignature> {
        data.push(nonce);

        // Sign the token transfer instance
        data.extend(word_4_from_u8_32(address.set.0));
        data.extend(word_4_from_u8_32(address.intent.0));

        let sig = self.wallet.sign_words(&data, account_name)?;
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
