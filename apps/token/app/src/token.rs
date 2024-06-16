use essential_rest_client::EssentialClient;
use essential_server_types::SolutionOutcome;
use essential_types::{
    convert::word_4_from_u8_32,
    solution::{Solution, SolutionData},
    ContentAddress, IntentAddress, Word,
};
use essential_wallet::Wallet;

use crate::inputs::{self, token::query_balances};

pub struct Token {
    client: EssentialClient,
    wallet: Wallet,
    deployed_intents: Addresses,
}

#[derive(Debug, Clone)]
pub struct Addresses {
    pub token: ContentAddress,
    pub mint: IntentAddress,
    pub transfer: IntentAddress,
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

    pub async fn mint(
        &mut self,
        account_name: &str,
        amount: i64,
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
        let from_balance = self.balance(from_name).await.unwrap().unwrap_or_default();
        let to_balance = self.balance(to_name).await.unwrap().unwrap_or_default();
        self.transfer_inner(
            from_key,
            to_key,
            amount,
            from_balance - amount,
            to_balance + amount,
        )
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

    async fn query(&self, set_address: &ContentAddress, key: &[Word]) -> anyhow::Result<Vec<Word>> {
        let state = self.client.query_state(set_address, &key.to_vec()).await?;
        Ok(state)
    }

    async fn mint_inner(&mut self, key: [Word; 4], amount: Word) -> anyhow::Result<ContentAddress> {
        let decision_variables = inputs::token::mint::DecVars {
            owner: key.into(),
            amount: amount.into(),
        };
        let mutation = inputs::token::balances(key.into(), amount.into());
        let solution = Solution {
            data: vec![SolutionData {
                intent_to_solve: self.deployed_intents.mint.clone(),
                decision_variables: decision_variables.encode(),
                transient_data: Default::default(),
                state_mutations: vec![mutation],
            }],
        };
        self.client.submit_solution(solution).await
    }

    async fn transfer_inner(
        &mut self,
        from: [Word; 4],
        to: [Word; 4],
        amount: Word,
        from_balance: i64,
        to_balance: i64,
    ) -> anyhow::Result<ContentAddress> {
        let decision_variables = inputs::token::transfer::DecVars {
            receiver: to.into(),
            sender: from.into(),
            amount: amount.into(),
        };
        let mutation_from = inputs::token::balances(from.into(), from_balance.into());
        let mutation_to = inputs::token::balances(to.into(), to_balance.into());
        let solution = Solution {
            data: vec![SolutionData {
                intent_to_solve: self.deployed_intents.transfer.clone(),
                decision_variables: decision_variables.encode(),
                transient_data: Default::default(),
                state_mutations: vec![mutation_from, mutation_to],
            }],
        };
        self.client.submit_solution(solution).await
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
