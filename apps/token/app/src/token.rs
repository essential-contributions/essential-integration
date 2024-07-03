use crate::inputs::{self, token::query_balances, SignedBurn, SignedMint, SignedTransfer};
use anyhow::bail;
use essential_rest_client::EssentialClient;
use essential_server_types::SolutionOutcome;
use essential_types::{
    convert::word_4_from_u8_32, solution::Solution, ContentAddress, PredicateAddress, Word,
};
use essential_wallet::Wallet;

pub struct Token {
    client: EssentialClient,
    wallet: Wallet,
    deployed_predicates: Addresses,
}

#[derive(Debug, Clone)]
pub struct Addresses {
    pub token: ContentAddress,
    pub burn: PredicateAddress,
    pub mint: PredicateAddress,
    pub transfer: PredicateAddress,
    pub cancel: PredicateAddress,
    pub signed: ContentAddress,
    pub signed_transfer: PredicateAddress,
    pub signed_transfer_with: PredicateAddress,
    pub signed_mint: PredicateAddress,
    pub signed_burn: PredicateAddress,
    pub signed_cancel: PredicateAddress,
}

impl Token {
    pub fn new(
        addr: String,
        deployed_predicates: Addresses,
        wallet: essential_wallet::Wallet,
    ) -> anyhow::Result<Self> {
        let client = EssentialClient::new(addr)?;
        Ok(Self {
            client,
            deployed_predicates,
            wallet,
        })
    }

    pub fn create_account(&mut self, account_name: &str) -> anyhow::Result<()> {
        self.wallet
            .new_key_pair(account_name, essential_wallet::Scheme::Secp256k1)
    }

    /// Create and submit solution that solves the token burn predicate
    /// and the signed burn predicate
    pub async fn burn(
        &mut self,
        account_name: &str,
        amount: i64,
    ) -> anyhow::Result<ContentAddress> {
        // Get the hashed public key of the account
        let key = self.get_hashed_key(account_name)?;

        // Increment the nonce
        let new_nonce = self.increment_nonce(key).await?;

        // Calculate the new balance of the account after the burn
        let new_from_balance = self.calculate_from_balance(key, amount).await?;

        let builder = SignedBurn {
            auth_address: self.deployed_predicates.signed_burn.clone(),
            burn_address: self.deployed_predicates.burn.clone(),
            from_account_name: account_name.to_string(),
            new_nonce,
            amount,
            new_from_balance,
        };

        let solution = builder.build(&mut self.wallet)?;

        // Submit the solution
        self.client.submit_solution(solution).await
    }

    pub async fn mint(
        &mut self,
        account_name: &str,
        balance: Word,
    ) -> anyhow::Result<ContentAddress> {
        let solution = self.mint_solution(account_name, balance).await?;

        // Submit the solution
        self.client.submit_solution(solution).await
    }

    pub async fn mint_solution(
        &mut self,
        account_name: &str,
        balance: Word,
    ) -> anyhow::Result<Solution> {
        // Get the hashed public key of the account
        let key = self.get_hashed_key(account_name)?;

        // Increment the nonce
        let nonce = self.increment_nonce(key).await?;

        // Set the number of decimals for the token
        let decimals = 18;

        let builder = SignedMint {
            auth_address: self.deployed_predicates.signed_mint.clone(),
            mint_address: self.deployed_predicates.mint.clone(),
            account_name: account_name.to_string(),
            new_nonce: nonce,
            amount: balance,
            decimals,
            name: [0; 4],
            symbol: [0; 4],
        };

        builder.build(&mut self.wallet)
    }

    pub async fn transfer(
        &mut self,
        from_name: &str,
        to_name: &str,
        amount: Word,
    ) -> anyhow::Result<ContentAddress> {
        // Get the hashed public key of the from account
        let key = self.get_hashed_key(from_name)?;

        // Get the hashed public key of the to account
        let to = self.get_hashed_key(to_name)?;

        // Increment the nonce
        let new_nonce = self.increment_nonce(key).await?;

        let new_from_balance = self.calculate_from_balance(key, amount).await?;

        let state = self
            .query(&self.deployed_predicates.token, &query_balances(to.into()))
            .await?;
        let to_balance = state.first().copied().unwrap_or_default();
        let Some(new_to_balance) = to_balance.checked_add(amount) else {
            bail!("Overflow error")
        };

        let builder = SignedTransfer {
            auth_address: self.deployed_predicates.signed_transfer.clone(),
            token_address: self.deployed_predicates.transfer.clone(),
            from_account_name: from_name.to_string(),
            to_account_name: to_name.to_string(),
            new_nonce,
            amount,
            new_from_balance,
            new_to_balance,
        };

        let solution = builder.build(&mut self.wallet)?;

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
            .query(&self.deployed_predicates.token, &query_balances(key.into()))
            .await?;
        Ok(state.first().copied())
    }

    /// Query the nonce of the account
    pub async fn nonce(&self, key: [Word; 4]) -> anyhow::Result<Word> {
        let nonce = self
            .query(
                &self.deployed_predicates.token,
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

    async fn calculate_from_balance(&self, key: [Word; 4], amount: Word) -> anyhow::Result<Word> {
        let state = self
            .query(&self.deployed_predicates.token, &query_balances(key.into()))
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

    fn get_hashed_key(&mut self, account_name: &str) -> anyhow::Result<[Word; 4]> {
        let public_key = self.wallet.get_public_key(account_name)?;
        let essential_signer::PublicKey::Secp256k1(public_key) = public_key else {
            anyhow::bail!("Invalid public key")
        };
        let encoded = essential_sign::encode::public_key(&public_key);
        Ok(word_4_from_u8_32(essential_hash::hash_words(&encoded)))
    }
}
