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
    pub signed_transfer_with: IntentAddress,
    pub signed_mint: IntentAddress,
    pub signed_burn: IntentAddress,
    pub signed_cancel: IntentAddress,
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

    /// Create and submit solution that solves the token burn intent
    /// and the signed burn intent
    pub async fn burn(
        &mut self,
        account_name: &str,
        amount: i64,
    ) -> anyhow::Result<ContentAddress> {
        // Get the hashed public key of the account
        let key = self.get_hashed_key(account_name)?;

        // Increment the nonce
        let nonce = self.increment_nonce(key).await?;

        // Solve the token burn intent
        let token_burn = self.create_token_burn(nonce, key, amount, 1.into()).await?;

        // Solve the signed authorization for the burn intent
        let signed_burn = self.create_signed_burn(account_name, nonce, key, amount, 0.into())?;

        let solution = Solution {
            data: vec![token_burn, signed_burn],
        };

        // Submit the solution
        self.client.submit_solution(solution).await
    }

    pub async fn mint(
        &mut self,
        account_name: &str,
        balance: Word,
    ) -> anyhow::Result<ContentAddress> {
        // Get the hashed public key of the account
        let key = self.get_hashed_key(account_name)?;

        // Increment the nonce
        let nonce = self.increment_nonce(key).await?;

        // Set the number of decimals for the token
        let decimals = 18;

        // Solve the token mint intent
        let mint = self.create_token_mint(nonce, key, balance, decimals, 1.into())?;

        // Solve the signed authorization for the mint intent
        let mint_auth =
            self.create_signed_mint(account_name, nonce, key, balance, decimals, 0.into())?;

        let solution = Solution {
            data: vec![mint, mint_auth],
        };

        // Submit the solution
        self.client.submit_solution(solution).await
    }

    pub async fn transfer(
        &mut self,
        from_name: &str,
        to_name: &str,
        amount: i64,
    ) -> anyhow::Result<ContentAddress> {
        // Get the hashed public key of the from account
        let key = self.get_hashed_key(from_name)?;

        // Get the hashed public key of the to account
        let to = self.get_hashed_key(to_name)?;

        // Increment the nonce
        let nonce = self.increment_nonce(key).await?;

        // Solve the token transfer intent
        let token_transfer = self
            .create_token_transfer(nonce, key, to, amount, 1.into())
            .await?;

        // Solve the signed authorization for the transfer intent
        let signed_transfer =
            self.create_signed_transfer(from_name, nonce, key, to, amount, 0.into())?;

        let solution = Solution {
            data: vec![token_transfer, signed_transfer],
        };

        // Submit the solution
        self.client.submit_solution(solution).await
    }

    /// Query the outcome of a solution
    pub async fn solution_outcome(
        &mut self,
        solution_address: ContentAddress,
    ) -> anyhow::Result<Vec<SolutionOutcome>> {
        self.client.solution_outcome(&solution_address.0).await
    }

    /// Query the balance of the account
    pub async fn balance(&mut self, account_name: &str) -> anyhow::Result<Option<i64>> {
        let key = self.get_hashed_key(account_name)?;
        let state = self
            .query(&self.deployed_intents.token, &query_balances(key.into()))
            .await?;
        Ok(state.first().copied())
    }

    /// Query the nonce of the account
    pub async fn nonce(&self, key: [Word; 4]) -> anyhow::Result<Word> {
        let nonce = self
            .query(
                &self.deployed_intents.token,
                &inputs::token::query_nonce((key).into()),
            )
            .await?;
        Ok(nonce.first().copied().unwrap_or_default())
    }

    // Query state
    async fn query(&self, set_address: &ContentAddress, key: &[Word]) -> anyhow::Result<Vec<Word>> {
        let state = self.client.query_state(set_address, &key.to_vec()).await?;
        Ok(state)
    }

    async fn create_token_burn(
        &self,
        nonce: Word,
        key: [Word; 4],
        amount: Word,
        auth_path: Word,
    ) -> anyhow::Result<SolutionData> {
        // Calculate the new balance of the account after the burn
        let new_from_balance = self.calculate_from_balance(key, amount).await?;

        // Set the key and amount to be burned
        let transient_data = inputs::token::burn::TransientData {
            key: key.into(),
            amount: amount.into(),
        };

        // Set the instance of the authentication intent
        let decision_variables = inputs::token::burn::DecVars {
            auth_addr: Instance {
                address: self.deployed_intents.signed_burn.clone(),
                path: auth_path,
            },
        };

        // Create the burn mutation which sets the balance of the account to the new balance.
        let burn_mutation = inputs::token::balances(key.into(), new_from_balance.into());

        // Create the nonce mutation which increments the nonce of the account
        let nonce_mutation = inputs::token::nonce(key.into(), nonce.into());

        let token_burn = SolutionData {
            intent_to_solve: self.deployed_intents.burn.clone(),
            decision_variables: decision_variables.encode(),
            transient_data: transient_data.encode(),
            state_mutations: vec![burn_mutation, nonce_mutation],
        };

        Ok(token_burn)
    }

    fn create_signed_burn(
        &mut self,
        account_name: &str,
        nonce: Word,
        key: [Word; 4],
        amount: Word,
        burn_path: Word,
    ) -> anyhow::Result<SolutionData> {
        // The instance of the token burn intent
        let instance = Instance {
            address: self.deployed_intents.burn.clone(),
            path: burn_path,
        };

        // Hash and sign the key and amount to be burned
        let mut data = key.to_vec();
        data.push(amount);

        let sig = self.sign_data(account_name, data, nonce, instance.address.clone())?;

        // Set the path of the token burn intent,
        // the signature of the key and amount to be burned,
        // and the public key of the account
        let decision_variables = inputs::signed::burn::DecVars {
            token_path: instance.path.into(),
            sig,
            public_key: self.get_pub_key(account_name)?,
        };

        // Set the address of the token to be burned
        let transient_data = inputs::signed::burn::TransientData {
            token_address: instance.address,
        };

        let signed_burn = SolutionData {
            intent_to_solve: self.deployed_intents.signed_burn.clone(),
            decision_variables: decision_variables.encode(),
            transient_data: transient_data.encode(),
            state_mutations: vec![],
        };

        Ok(signed_burn)
    }

    fn create_token_mint(
        &self,
        nonce: Word,
        key: [Word; 4],
        balance: Word,
        decimals: Word,
        auth_path: Word,
    ) -> anyhow::Result<SolutionData> {
        // Set the instance of the authentication intent
        let auth_instance = Instance {
            address: self.deployed_intents.signed_mint.clone(),
            path: auth_path,
        };

        // Set the name and symbol of the token.
        // Set the address of the authentication intent.
        let decision_variables = inputs::token::mint::DecVars {
            name: [0; 4].into(),
            symbol: [0; 4].into(),
            auth_addr: auth_instance,
        };

        // Set the key, balance, and decimals of the token to be minted.
        let transient_data = inputs::token::mint::TransientData {
            key: key.into(),
            amount: balance.into(),
            decimals: decimals.into(),
        };

        // Create the mutations.

        // Set the balance of the account to the new balance.
        let bal_mutation = inputs::token::balances(key.into(), balance.into());

        // Set the name of the token.
        let name_mutation = inputs::token::token_name(decision_variables.name);

        // Set the symbol of the token.
        let symbol_mutation = inputs::token::token_symbol(decision_variables.symbol);

        // Set the decimals of the token.
        let decimals_mutation = inputs::token::decimals(18.into());

        // Increment the nonce of the account.
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
        Ok(mint)
    }

    fn create_signed_mint(
        &mut self,
        account_name: &str,
        nonce: Word,
        key: [Word; 4],
        balance: Word,
        decimals: Word,
        mint_path: Word,
    ) -> anyhow::Result<SolutionData> {
        // Hash and sign the key, balance, and decimals of the token to be minted.
        let mut data = key.to_vec();
        data.push(balance);
        data.push(decimals);
        let sig = self.sign_data(
            account_name,
            data,
            nonce,
            self.deployed_intents.mint.clone(),
        )?;

        // Set the path of the token mint intent,
        // the signature of the key, balance, and decimals of the token to be minted,
        // and the public key of the account.
        let decision_variables = inputs::signed::mint::DecVars {
            token_path: mint_path.into(),
            sig,
            public_key: self.get_pub_key(account_name)?,
        };

        // Set the address of the token to be minted.
        let transient_data = inputs::signed::mint::TransientData {
            token_address: self.deployed_intents.mint.clone(),
        };

        let mint_auth = SolutionData {
            intent_to_solve: self.deployed_intents.signed_mint.clone(),
            decision_variables: decision_variables.encode(),
            transient_data: transient_data.encode(),
            state_mutations: vec![],
        };

        Ok(mint_auth)
    }

    async fn create_token_transfer(
        &self,
        nonce: Word,
        key: [Word; 4],
        to: [Word; 4],
        amount: Word,
        auth_path: Word,
    ) -> anyhow::Result<SolutionData> {
        // Set the instance of the authentication intent
        let decision_variables = inputs::token::transfer::DecVars {
            auth_addr: Instance {
                address: self.deployed_intents.signed_transfer.clone(),
                path: auth_path,
            },
        };

        // Calculate the new balances.
        let new_from_balance = self.calculate_from_balance(key, amount).await?;

        let state = self
            .query(&self.deployed_intents.token, &query_balances(to.into()))
            .await?;
        let to_balance = state.first().copied().unwrap_or_default();
        let Some(to_new_balance) = to_balance.checked_add(amount) else {
            bail!("Overflow error")
        };

        // Set the key to transfer from,
        // the address to transfer to
        // and amount to be transferred.
        let transient_data = inputs::token::transfer::TransientData {
            key: key.into(),
            to: to.into(),
            amount: amount.into(),
        };

        // Create the mutations.

        // Set the balance of the from account.
        let from_mutation = inputs::token::balances(key.into(), new_from_balance.into());

        // Set the balance of the to account.
        let to_mutation = inputs::token::balances(to.into(), to_new_balance.into());

        // Increment the nonce of the account.
        let nonce_mutation = inputs::token::nonce(key.into(), nonce.into());

        let token_transfer = SolutionData {
            intent_to_solve: self.deployed_intents.transfer.clone(),
            decision_variables: decision_variables.encode(),
            transient_data: transient_data.encode(),
            state_mutations: vec![from_mutation, to_mutation, nonce_mutation],
        };

        Ok(token_transfer)
    }

    fn create_signed_transfer(
        &mut self,
        account_name: &str,
        nonce: Word,
        key: [Word; 4],
        to: [Word; 4],
        amount: Word,
        token_path: Word,
    ) -> anyhow::Result<SolutionData> {
        // The instance of the token transfer intent
        let instance = Instance {
            address: self.deployed_intents.transfer.clone(),
            path: token_path,
        };

        // Hash and sign the key, address, and amount to be transferred
        let mut data = key.to_vec();
        data.extend(to);
        data.push(amount);

        let sig = self.sign_data(
            account_name,
            data,
            nonce,
            self.deployed_intents.transfer.clone(),
        )?;

        // Set the path of the token transfer intent,
        // the signature of the key, address, and amount to be transferred,
        // and the public key of the account.
        let decision_variables = inputs::signed::transfer::DecVars {
            sig,
            public_key: self.get_pub_key(account_name)?,
            token_path: instance.path.into(),
        };

        // Set the address of the token to be transferred
        let transient_data = inputs::signed::transfer::TransientData {
            token_address: instance.address,
        };

        let signed_transfer = SolutionData {
            intent_to_solve: self.deployed_intents.signed_transfer.clone(),
            decision_variables: decision_variables.encode(),
            transient_data: transient_data.encode(),
            state_mutations: vec![],
        };

        Ok(signed_transfer)
    }

    async fn calculate_from_balance(&self, key: [Word; 4], amount: Word) -> anyhow::Result<Word> {
        let state = self
            .query(&self.deployed_intents.token, &query_balances(key.into()))
            .await?;
        let from_balance = if state.is_empty() {
            0
        } else {
            let [from_balance] = &state[..] else {
                bail!("Invalid state");
            };
            *from_balance
        };
        let Some(new_from_balance) = from_balance.checked_sub(amount) else {
            bail!("Insufficient balance")
        };
        Ok(new_from_balance)
    }

    async fn increment_nonce(&self, key: [Word; 4]) -> anyhow::Result<Word> {
        let nonce = self.nonce(key).await?;
        Ok(nonce + 1)
    }

    fn sign_data(
        &mut self,
        account_name: &str,
        mut data: Vec<Word>,
        nonce: Word,
        address: IntentAddress,
    ) -> anyhow::Result<essential_signer::secp256k1::ecdsa::RecoverableSignature> {
        data.push(nonce);

        // Sign the token instance
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
