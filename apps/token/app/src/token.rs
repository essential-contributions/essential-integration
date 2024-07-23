use anyhow::bail;
use essential_app_utils::inputs::Encode;
use essential_rest_client::EssentialClient;
use essential_server_types::SolutionOutcome;
use essential_types::{convert::word_4_from_u8_32, solution::Solution, ContentAddress, Word};
use essential_wallet::Wallet;

/// Items generated from `token-abi.json`.
#[allow(clippy::module_inception)]
pub(crate) mod token;

/// Items generated from `signed-abi.json`.
pub(crate) mod signed;

pub struct Token {
    client: EssentialClient,
    wallet: Wallet,
}

impl Token {
    pub fn new(addr: String, wallet: essential_wallet::Wallet) -> anyhow::Result<Self> {
        let client = EssentialClient::new(addr)?;
        Ok(Self { client, wallet })
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

        const AUTH_PATH: Word = 0;
        const BURN_PATH: Word = 1;

        let mut data = key.to_vec();
        data.push(amount);
        data.push(new_nonce);
        data.extend(token::Burn::ADDRESS.contract.encode());
        data.extend(token::Burn::ADDRESS.predicate.encode());

        let mut solution = Solution {
            data: Default::default(),
        };

        let auth = signed::BurnData {
            predicate_to_solve: signed::Burn::ADDRESS,
            decision_variables: signed::Burn::Vars {
                ___I_pathway: BURN_PATH,
                sig: self.sign_data(account_name, data)?.encode(),
            },
            transient_data: signed::Burn::pub_vars::mutations().token(|addr| {
                let (c, p) = token::Burn::ADDRESS.encode();
                addr.contract(c).addr(p)
            }),
        };

        solution.data.insert(AUTH_PATH as usize, auth.into());

        let burn = token::BurnData {
            predicate_to_solve: token::Burn::ADDRESS,
            decision_variables: token::Burn::Vars {
                auth_addr: signed::Burn::ADDRESS.encode(),
                ___A_pathway: AUTH_PATH,
            },
            transient_data: token::Burn::pub_vars::mutations().key(key).amount(amount),
            state_mutations: token::storage::mutations()
                .balances(|map| map.entry(key, new_from_balance))
                .nonce(|nonces| nonces.entry(key, new_nonce)),
        };

        solution.data.insert(BURN_PATH as usize, burn.into());

        self.client.submit_solution(solution).await
    }

    pub async fn mint(
        &mut self,
        account_name: &str,
        balance: Word,
        token_name: &str,
        token_symbol: &str,
    ) -> anyhow::Result<ContentAddress> {
        let solution = self
            .mint_solution(account_name, balance, token_name, token_symbol)
            .await?;

        // Submit the solution
        self.client.submit_solution(solution).await
    }

    pub async fn mint_solution(
        &mut self,
        account_name: &str,
        balance: Word,
        token_name: &str,
        token_symbol: &str,
    ) -> anyhow::Result<Solution> {
        // Get the hashed public key of the account
        let key = self.get_hashed_key(account_name)?;

        // Increment the nonce
        let nonce = self.increment_nonce(key).await?;

        // Set the number of decimals for the token
        let decimals = 18;

        const AUTH_PATH: Word = 0;
        const MINT_PATH: Word = 1;

        let mut data = key.to_vec();
        data.push(balance);
        data.push(decimals);
        data.push(nonce);
        data.extend(token::Mint::ADDRESS.contract.encode());
        data.extend(token::Mint::ADDRESS.predicate.encode());

        let mut solution = Solution {
            data: Default::default(),
        };

        let auth = signed::MintData {
            predicate_to_solve: signed::Mint::ADDRESS.clone(),
            decision_variables: signed::Mint::Vars {
                ___I_pathway: MINT_PATH,
                sig: self.sign_data(account_name, data)?.encode(),
            },
            transient_data: signed::Mint::pub_vars::mutations().token(|addr| {
                let (c, p) = signed::Mint::ADDRESS.encode();
                addr.contract(c).addr(p)
            }),
        };

        solution.data.insert(AUTH_PATH as usize, auth.into());

        let mint = token::MintData {
            predicate_to_solve: token::Mint::ADDRESS.clone(),
            decision_variables: token::Mint::Vars {
                auth_addr: signed::Mint::ADDRESS.encode(),
                ___A_pathway: AUTH_PATH,
            },
            transient_data: token::Mint::pub_vars::mutations()
                .key(key)
                .decimals(decimals)
                .amount(balance),
            state_mutations: token::storage::mutations()
                .balances(|map| map.entry(key, balance))
                .token_name(word_4_from_u8_32(essential_hash::hash(&token_name)))
                .token_symbol(word_4_from_u8_32(essential_hash::hash(&token_symbol)))
                .decimals(decimals)
                .nonce(|nonces| nonces.entry(key, nonce)),
        };

        solution.data.insert(MINT_PATH as usize, mint.into());
        Ok(solution)
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
            .query(&token::ADDRESS, &token::query_balances(to.into()))
            .await?;
        let to_balance = state.first().copied().unwrap_or_default();
        let Some(new_to_balance) = to_balance.checked_add(amount) else {
            bail!("Overflow error")
        };

        const AUTH_PATH: Word = 0;
        const TRANSFER_PATH: Word = 1;

        let mut data = key.to_vec();
        data.extend(to);
        data.push(amount);
        data.push(new_nonce);
        data.extend(token::Transfer::ADDRESS.contract.encode());
        data.extend(token::Transfer::ADDRESS.predicate.encode());

        let mut solution = Solution {
            data: Default::default(),
        };

        let auth = signed::TransferData {
            predicate_to_solve: signed::Transfer::ADDRESS,
            decision_variables: signed::Transfer::Vars {
                ___I_pathway: TRANSFER_PATH,
                sig: self.sign_data(from_name, data)?.encode(),
            },
            transient_data: signed::Transfer::pub_vars::mutations().token(|addr| {
                let (c, p) = token::Transfer::ADDRESS.encode();
                addr.contract(c).addr(p)
            }),
        };

        solution.data.insert(AUTH_PATH as usize, auth.into());

        let transfer = token::TransferData {
            predicate_to_solve: token::Transfer::ADDRESS,
            decision_variables: token::Transfer::Vars {
                auth_addr: signed::Transfer::ADDRESS.encode(),
                ___A_pathway: AUTH_PATH,
            },
            transient_data: token::Transfer::pub_vars::mutations()
                .key(key)
                .to(to)
                .amount(amount),
            state_mutations: token::storage::mutations()
                .balances(|map| map.entry(key, new_from_balance))
                .balances(|map| map.entry(to, new_to_balance))
                .nonce(|nonces| nonces.entry(key, new_nonce)),
        };

        solution
            .data
            .insert(TRANSFER_PATH as usize, transfer.into());

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
            .query(&token::ADDRESS, &token::query_balances(key.into()))
            .await?;
        Ok(state.first().copied())
    }

    /// Query the nonce of the account
    pub async fn nonce(&self, key: [Word; 4]) -> anyhow::Result<Word> {
        let nonce = self
            .query(&token::ADDRESS, &token::query_nonce(key.into()))
            .await?;
        Ok(nonce.first().copied().unwrap_or_default())
    }

    // Query state
    async fn query(
        &self,
        set_address: &ContentAddress,
        key: &Vec<Word>,
    ) -> anyhow::Result<Vec<Word>> {
        let state = self.client.query_state(set_address, key).await?;
        Ok(state)
    }

    async fn calculate_from_balance(&self, key: [Word; 4], amount: Word) -> anyhow::Result<Word> {
        let state = self
            .query(&token::ADDRESS, &token::query_balances(key.into()))
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

    fn get_pub_key(
        &mut self,
        account_name: &str,
    ) -> anyhow::Result<essential_signer::secp256k1::PublicKey> {
        let public_key = self.wallet.get_public_key(account_name)?;
        let essential_signer::PublicKey::Secp256k1(public_key) = public_key else {
            anyhow::bail!("Invalid public key")
        };
        Ok(public_key)
    }

    fn get_hashed_key(&mut self, account_name: &str) -> anyhow::Result<[Word; 4]> {
        let public_key = self.get_pub_key(account_name)?;
        let encoded = essential_sign::encode::public_key(&public_key);
        Ok(word_4_from_u8_32(essential_hash::hash_words(&encoded)))
    }
    fn sign_data(
        &mut self,
        account_name: &str,
        data: Vec<Word>,
    ) -> anyhow::Result<essential_signer::secp256k1::ecdsa::RecoverableSignature> {
        let sig = self.wallet.sign_words(&data, account_name)?;
        let sig = match sig {
            essential_signer::Signature::Secp256k1(sig) => sig,
            _ => bail!("Invalid signature"),
        };
        Ok(sig)
    }
}
