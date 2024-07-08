use anyhow::bail;
use essential_app_utils::inputs::Instance;
use essential_types::{
    convert::word_4_from_u8_32,
    solution::{Solution, SolutionData},
    ContentAddress, PredicateAddress, Word,
};
use essential_wallet::Wallet;

pub mod signed;
pub mod token;

const AUTH_PATH: Word = 0;
const TOKEN_PATH: Word = 1;
const BURN_PATH: Word = 1;
const MINT_PATH: Word = 1;

pub struct SignedTransfer {
    pub auth_address: PredicateAddress,
    pub token_address: PredicateAddress,
    pub from_account_name: String,
    pub to_account_name: String,
    pub new_nonce: Word,
    pub amount: Word,
    pub new_from_balance: Word,
    pub new_to_balance: Word,
}

pub struct SignedBurn {
    pub auth_address: PredicateAddress,
    pub burn_address: PredicateAddress,
    pub from_account_name: String,
    pub new_nonce: Word,
    pub amount: Word,
    pub new_from_balance: Word,
}

pub struct SignedMint {
    pub auth_address: PredicateAddress,
    pub mint_address: PredicateAddress,
    pub account_name: String,
    pub new_nonce: Word,
    pub amount: Word,
    pub decimals: Word,
    pub name: [Word; 4],
    pub symbol: [Word; 4],
}

impl SignedTransfer {
    pub fn build(&self, wallet: &mut Wallet) -> anyhow::Result<Solution> {
        let mut solution = blank_solution();
        solution.data[TOKEN_PATH as usize] = self.create_token_transfer(wallet)?;
        solution.data[AUTH_PATH as usize] = self.create_signed_transfer(wallet)?;
        Ok(solution)
    }

    fn create_token_transfer(&self, wallet: &mut Wallet) -> anyhow::Result<SolutionData> {
        let key = get_hashed_key(wallet, &self.from_account_name)?;
        let to = get_hashed_key(wallet, &self.to_account_name)?;
        let amount = self.amount;
        // Set the instance of the authentication predicate
        let decision_variables = token::transfer::DecVars {
            auth_addr: Instance {
                address: self.auth_address.clone(),
                path: AUTH_PATH,
            },
        };

        // Calculate the new balances.

        // Set the key to transfer from,
        // the address to transfer to
        // and amount to be transferred.
        let transient_data = token::transfer::TransientData {
            key: key.into(),
            to: to.into(),
            amount: amount.into(),
        };

        // Create the mutations.

        // Set the balance of the from account.
        let from_mutation = token::balances(key.into(), self.new_from_balance.into());

        // Set the balance of the to account.
        let to_mutation = token::balances(to.into(), self.new_to_balance.into());

        // Increment the nonce of the account.
        let nonce_mutation = token::nonce(key.into(), self.new_nonce.into());

        let token_transfer = SolutionData {
            predicate_to_solve: self.token_address.clone(),
            decision_variables: decision_variables.encode(),
            transient_data: transient_data.encode(),
            state_mutations: vec![from_mutation, to_mutation, nonce_mutation],
        };

        Ok(token_transfer)
    }

    fn create_signed_transfer(&self, wallet: &mut Wallet) -> anyhow::Result<SolutionData> {
        let key = get_hashed_key(wallet, &self.from_account_name)?;
        let to = get_hashed_key(wallet, &self.to_account_name)?;
        // The instance of the token transfer predicate
        let instance = Instance {
            address: self.token_address.clone(),
            path: TOKEN_PATH,
        };

        // Hash and sign the key, address, and amount to be transferred
        let mut data = key.to_vec();
        data.extend(to);
        data.push(self.amount);

        let sig = sign_data(
            wallet,
            &self.from_account_name,
            data,
            self.new_nonce,
            self.token_address.clone(),
        )?;

        // Set the path of the token transfer predicate,
        // the signature of the key, address, and amount to be transferred,
        // and the public key of the account.
        let decision_variables = signed::transfer::DecVars {
            sig,
            public_key: get_pub_key(wallet, &self.from_account_name)?,
            token_path: instance.path.into(),
        };

        // Set the address of the token to be transferred
        let transient_data = signed::transfer::TransientData {
            token_address: instance.address,
        };

        let signed_transfer = SolutionData {
            predicate_to_solve: self.auth_address.clone(),
            decision_variables: decision_variables.encode(),
            transient_data: transient_data.encode(),
            state_mutations: vec![],
        };

        Ok(signed_transfer)
    }
}

impl SignedBurn {
    pub fn build(&self, wallet: &mut Wallet) -> anyhow::Result<Solution> {
        let mut solution = blank_solution();
        solution.data[BURN_PATH as usize] = self.create_token_burn(wallet)?;
        solution.data[AUTH_PATH as usize] = self.create_signed_burn(wallet)?;
        Ok(solution)
    }

    fn create_token_burn(&self, wallet: &mut Wallet) -> anyhow::Result<SolutionData> {
        let key = get_hashed_key(wallet, &self.from_account_name)?;

        // Set the key and amount to be burned
        let transient_data = token::burn::TransientData {
            key: key.into(),
            amount: self.amount.into(),
        };

        // Set the instance of the authentication predicate
        let decision_variables = token::burn::DecVars {
            auth_addr: Instance {
                address: self.auth_address.clone(),
                path: AUTH_PATH,
            },
        };

        // Create the burn mutation which sets the balance of the account to the new balance.
        let burn_mutation = token::balances(key.into(), self.new_from_balance.into());

        // Create the nonce mutation which increments the nonce of the account
        let nonce_mutation = token::nonce(key.into(), self.new_nonce.into());

        let token_burn = SolutionData {
            predicate_to_solve: self.burn_address.clone(),
            decision_variables: decision_variables.encode(),
            transient_data: transient_data.encode(),
            state_mutations: vec![burn_mutation, nonce_mutation],
        };

        Ok(token_burn)
    }

    fn create_signed_burn(&self, wallet: &mut Wallet) -> anyhow::Result<SolutionData> {
        let key = get_hashed_key(wallet, &self.from_account_name)?;
        // The instance of the token burn predicate
        let instance = Instance {
            address: self.burn_address.clone(),
            path: BURN_PATH,
        };

        // Hash and sign the key and amount to be burned
        let mut data = key.to_vec();
        data.push(self.amount);

        let sig = sign_data(
            wallet,
            &self.from_account_name,
            data,
            self.new_nonce,
            instance.address.clone(),
        )?;

        // Set the path of the token burn predicate,
        // the signature of the key and amount to be burned,
        // and the public key of the account
        let decision_variables = signed::burn::DecVars {
            token_path: instance.path.into(),
            sig,
            public_key: get_pub_key(wallet, &self.from_account_name)?,
        };

        // Set the address of the token to be burned
        let transient_data = signed::burn::TransientData {
            token_address: instance.address,
        };

        let signed_burn = SolutionData {
            predicate_to_solve: self.auth_address.clone(),
            decision_variables: decision_variables.encode(),
            transient_data: transient_data.encode(),
            state_mutations: vec![],
        };

        Ok(signed_burn)
    }
}

impl SignedMint {
    pub fn build(&self, wallet: &mut Wallet) -> anyhow::Result<Solution> {
        let mut solution = blank_solution();
        solution.data[TOKEN_PATH as usize] = self.create_token_mint(wallet)?;
        solution.data[AUTH_PATH as usize] = self.create_signed_mint(wallet)?;
        Ok(solution)
    }

    fn create_token_mint(&self, wallet: &mut Wallet) -> anyhow::Result<SolutionData> {
        let key = get_hashed_key(wallet, &self.account_name)?;

        // Set the instance of the authentication predicate
        let auth_instance = Instance {
            address: self.auth_address.clone(),
            path: AUTH_PATH,
        };

        // Set the name and symbol of the token.
        // Set the address of the authentication predicate.
        let decision_variables = token::mint::DecVars {
            name: self.name.into(),
            symbol: self.symbol.into(),
            auth_addr: auth_instance,
        };

        // Set the key, balance, and decimals of the token to be minted.
        let transient_data = token::mint::TransientData {
            key: key.into(),
            amount: self.amount.into(),
            decimals: self.decimals.into(),
        };

        // Create the mutations.

        // Set the balance of the account to the new balance.
        let bal_mutation = token::balances(key.into(), self.amount.into());

        // Set the name of the token.
        let name_mutation = token::token_name(decision_variables.name);

        // Set the symbol of the token.
        let symbol_mutation = token::token_symbol(decision_variables.symbol);

        // Set the decimals of the token.
        let decimals_mutation = token::decimals(self.decimals.into());

        // Increment the nonce of the account.
        let nonce_mutation = token::nonce(key.into(), self.new_nonce.into());

        let mint = SolutionData {
            predicate_to_solve: self.mint_address.clone(),
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

    fn create_signed_mint(&self, wallet: &mut Wallet) -> anyhow::Result<SolutionData> {
        let key = get_hashed_key(wallet, &self.account_name)?;

        // Hash and sign the key, balance, and decimals of the token to be minted.
        let mut data = key.to_vec();
        data.push(self.amount);
        data.push(self.decimals);
        let sig = sign_data(
            wallet,
            &self.account_name,
            data,
            self.new_nonce,
            self.mint_address.clone(),
        )?;

        // Set the path of the token mint predicate,
        // the signature of the key, balance, and decimals of the token to be minted,
        // and the public key of the account.
        let decision_variables = signed::mint::DecVars {
            token_path: MINT_PATH.into(),
            sig,
            public_key: get_pub_key(wallet, &self.account_name)?,
        };

        // Set the address of the token to be minted.
        let transient_data = signed::mint::TransientData {
            token_address: self.mint_address.clone(),
        };

        let mint_auth = SolutionData {
            predicate_to_solve: self.auth_address.clone(),
            decision_variables: decision_variables.encode(),
            transient_data: transient_data.encode(),
            state_mutations: vec![],
        };

        Ok(mint_auth)
    }
}

fn sign_data(
    wallet: &mut Wallet,
    account_name: &str,
    mut data: Vec<Word>,
    nonce: Word,
    address: PredicateAddress,
) -> anyhow::Result<essential_signer::secp256k1::ecdsa::RecoverableSignature> {
    data.push(nonce);

    // Sign the token instance
    data.extend(word_4_from_u8_32(address.contract.0));
    data.extend(word_4_from_u8_32(address.predicate.0));

    let sig = wallet.sign_words(&data, account_name)?;
    let sig = match sig {
        essential_signer::Signature::Secp256k1(sig) => sig,
        _ => bail!("Invalid signature"),
    };
    Ok(sig)
}

fn get_hashed_key(wallet: &mut Wallet, account_name: &str) -> anyhow::Result<[Word; 4]> {
    let public_key = wallet.get_public_key(account_name)?;
    let essential_signer::PublicKey::Secp256k1(public_key) = public_key else {
        anyhow::bail!("Invalid public key")
    };
    let encoded = essential_sign::encode::public_key(&public_key);
    Ok(word_4_from_u8_32(essential_hash::hash_words(&encoded)))
}

fn get_pub_key(
    wallet: &mut Wallet,
    account_name: &str,
) -> anyhow::Result<essential_signer::secp256k1::PublicKey> {
    let public_key = wallet.get_public_key(account_name)?;
    let essential_signer::PublicKey::Secp256k1(public_key) = public_key else {
        bail!("Invalid public key")
    };
    Ok(public_key)
}

fn blank_solution() -> Solution {
    let data = SolutionData {
        predicate_to_solve: PredicateAddress {
            contract: ContentAddress([0; 32]),
            predicate: ContentAddress([0; 32]),
        },
        decision_variables: Default::default(),
        transient_data: Default::default(),
        state_mutations: Default::default(),
    };
    Solution {
        data: vec![data.clone(), data.clone()],
    }
}
