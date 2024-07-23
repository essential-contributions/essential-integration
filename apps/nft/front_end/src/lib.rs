use anyhow::{bail, ensure};
use essential_app_utils::{
    addresses::contract_hash,
    compile::compile_pint_project,
    inputs::Encode,
    print::{print_contract_address, print_predicate_address},
};
use essential_rest_client::EssentialClient;
use essential_types::{
    contract::Contract,
    convert::word_4_from_u8_32,
    solution::{Solution, SolutionData},
    ContentAddress, PredicateAddress, Word,
};
use std::{path::Path, vec};

/// Items generated from `nft-abi.json`.
mod nft {
    pint_abi::gen_from_file! {
        abi: "../pint/nft/out/debug/nft-abi.json",
        contract: "../pint/nft/out/debug/nft.json",
    }

    use essential_app_utils::inputs::{Int, B256};
    use essential_types::Key;

    // Short-hand for retrieving the owner key for the given token.
    pub fn query_owners(token: Int) -> essential_types::Key {
        let mut k: Vec<Key> = storage::keys().owners(|map| map.entry(token.0)).into();
        k.pop().unwrap()
    }

    // Short-hand for retrieving the nonce key for the given b256 key.
    pub fn query_nonce(key: B256) -> essential_types::Key {
        let mut k: Vec<Key> = storage::keys().nonce(|map| map.entry(key.0)).into();
        k.pop().unwrap()
    }
}

/// Items generated from `signed-abi.json`.
mod signed {
    pint_abi::gen_from_file! {
        abi: "../pint/signed/out/debug/signed-abi.json",
        contract:  "../pint/signed/out/debug/signed.json",
    }
}

/// Items generated from `swap-any-abi.json`.
pub mod swap_any {
    use essential_types::solution::Mutation;

    pint_abi::gen_from_file! {
        abi: "../pint/swap_any/out/debug/swap_any-abi.json",
        contract:  "../pint/swap_any/out/debug/swap_any.json",
    }

    // TODO: Remove the following after `pint-abi-gen` adds `keys()` builder.
    pub(crate) fn query_token() -> essential_types::Key {
        let mut m: Vec<Mutation> = storage::mutations().token(Default::default()).into();
        m.pop().unwrap().key
    }
}

pub struct Nft {
    client: EssentialClient,
    wallet: essential_wallet::Wallet,
}

impl Nft {
    pub fn new(addr: String, wallet: essential_wallet::Wallet) -> anyhow::Result<Self> {
        let client = EssentialClient::new(addr)?;
        Ok(Self { client, wallet })
    }

    pub fn create_account(&mut self, account_name: &str) -> anyhow::Result<()> {
        self.wallet
            .new_key_pair(account_name, essential_wallet::Scheme::Secp256k1)
    }

    fn mint_inner(&mut self, key: [Word; 4], token: Word) -> anyhow::Result<Solution> {
        let decision_variables = nft::Mint::Vars {
            token,
            new_owner: key,
        };

        let state_mutations = nft::storage::mutations()
            .owners(|map| map.entry(token, key))
            .into();

        let solution = Solution {
            data: vec![SolutionData {
                predicate_to_solve: nft::Mint::ADDRESS,
                decision_variables: decision_variables.into(),
                transient_data: Default::default(),
                state_mutations,
            }],
        };
        Ok(solution)
    }

    pub async fn mint(&mut self, account_name: &str, token: Word) -> anyhow::Result<()> {
        let key = self.get_hashed_key(account_name)?;
        let solution = self.mint_inner(key, token)?;
        self.client.submit_solution(solution).await?;
        Ok(())
    }

    pub fn mint_solution(&mut self, account_name: &str, token: Word) -> anyhow::Result<Solution> {
        let key = self.get_hashed_key(account_name)?;
        self.mint_inner(key, token)
    }

    pub async fn mint_for_contract(
        &mut self,
        contract: &PredicateAddress,
        token: Word,
    ) -> anyhow::Result<()> {
        let key = contract_hash(contract);
        let solution = self.mint_inner(key, token)?;
        self.client.submit_solution(solution).await?;
        Ok(())
    }

    async fn do_i_own_inner(&mut self, key: [Word; 4], token: Word) -> anyhow::Result<bool> {
        let state = self
            .query(&nft::ADDRESS, &nft::query_owners(token.into()))
            .await?;
        Ok(state[..] == key[..])
    }

    pub async fn do_i_own(&mut self, account_name: &str, token: Word) -> anyhow::Result<bool> {
        let key = self.get_hashed_key(account_name)?;
        self.do_i_own_inner(key, token).await
    }

    pub async fn do_i_own_contract(
        &mut self,
        contract: &PredicateAddress,
        token: Word,
    ) -> anyhow::Result<bool> {
        let key = contract_hash(contract);
        self.do_i_own_inner(key, token).await
    }

    pub async fn init_swap_any(&mut self, token: Word) -> anyhow::Result<()> {
        let solution = Solution {
            data: vec![SolutionData {
                predicate_to_solve: swap_any::Init::ADDRESS,
                decision_variables: swap_any::Init::Vars {
                    contract: nft::ADDRESS.into(),
                }
                .into(),
                transient_data: Default::default(),
                state_mutations: swap_any::storage::mutations().token(token).into(),
            }],
        };
        self.client.submit_solution(solution).await?;

        Ok(())
    }

    pub async fn swap_any_owns(&mut self) -> anyhow::Result<Option<Word>> {
        let state = self
            .query(&swap_any::ADDRESS, &swap_any::query_token())
            .await?;

        if state.is_empty() {
            return Ok(None);
        }

        let Ok([token]): Result<[Word; 1], _> = state.try_into() else {
            bail!("Bad token state")
        };

        Ok(Some(token))
    }

    pub async fn transfer(
        &mut self,
        account_name: &str,
        to: &str,
        token: Word,
    ) -> anyhow::Result<()> {
        let solution = self.transfer_solution(account_name, to, token).await?;

        self.client.submit_solution(solution).await?;

        Ok(())
    }

    pub async fn transfer_solution(
        &mut self,
        account_name: &str,
        to: &str,
        token: Word,
    ) -> anyhow::Result<Solution> {
        let key = self.get_hashed_key(account_name)?;
        let to = self.get_hashed_key(to)?;

        // Make key auth and transfer
        let solution = self
            .make_transfer_solution(account_name, key, to, token)
            .await?;

        Ok(solution)
    }

    async fn make_transfer_solution(
        &mut self,
        account_name: &str,
        key: [Word; 4],
        to: [Word; 4],
        token: Word,
    ) -> anyhow::Result<Solution> {
        let nonce = self
            .query(&nft::ADDRESS, &nft::query_nonce(key.into()))
            .await?;
        let mut nonce = nonce.first().copied().unwrap_or_default();
        nonce += 1;

        // Sign key, to, token
        let mut to_hash = key.to_vec();
        to_hash.extend_from_slice(&to);
        to_hash.push(token);
        to_hash.push(nonce);
        to_hash.extend(word_4_from_u8_32(nft::ADDRESS.0));
        to_hash.extend(word_4_from_u8_32(nft::Transfer::ADDRESS.predicate.0));

        let sig = self.wallet.sign_words(&to_hash, account_name)?;
        let sig = match sig {
            essential_signer::Signature::Secp256k1(sig) => sig,
            _ => bail!("Invalid signature"),
        };
        let pk = self.get_pub_key(account_name)?;

        let decision_variables = signed::Transfer::Vars {
            ___I_pathway: 1.into(),
            sig: sig.encode(),
            public_key: pk.encode(),
        };

        let transient_data = signed::Transfer::pub_vars::mutations()
            .nft(|tup| {
                tup.contract(nft::Transfer::ADDRESS.contract.into())
                    .addr(nft::Transfer::ADDRESS.predicate.into())
            })
            .into();

        let signed_transfer = SolutionData {
            predicate_to_solve: signed::Transfer::ADDRESS,
            decision_variables: decision_variables.into(),
            transient_data,
            state_mutations: vec![],
        };

        let transient_data = nft::Transfer::pub_vars::mutations()
            .key(key)
            .to(to)
            .token(token)
            .into();

        let decision_variables = nft::Transfer::Vars {
            auth_addr: (
                signed::Transfer::ADDRESS.contract.into(),
                signed::Transfer::ADDRESS.predicate.into(),
            ),
            ___A_pathway: 0,
        };

        let state_mutations = nft::storage::mutations()
            .owners(|map| map.entry(token, to))
            .nonce(|map| map.entry(key, nonce))
            .into();

        let transfer_nft = SolutionData {
            predicate_to_solve: nft::Transfer::ADDRESS,
            decision_variables: decision_variables.into(),
            transient_data,
            state_mutations,
        };

        Ok(Solution {
            data: vec![signed_transfer, transfer_nft],
        })
    }

    pub async fn swap_with_contract(
        &mut self,
        account_name: &str,
        token: Word,
    ) -> anyhow::Result<()> {
        let solution = self
            .swap_with_contract_solution(account_name, token)
            .await?;
        self.client.submit_solution(solution).await?;
        Ok(())
    }

    pub async fn swap_with_contract_solution(
        &mut self,
        account_name: &str,
        token: Word,
    ) -> anyhow::Result<Solution> {
        let key = self.get_hashed_key(account_name)?;
        let to = contract_hash(&swap_any::Swap::ADDRESS);
        let nonce = self
            .query(&nft::ADDRESS, &nft::query_nonce(to.into()))
            .await?;
        let mut nonce = nonce.first().copied().unwrap_or_default();
        nonce += 1;

        let mut solution = self
            .make_transfer_solution(account_name, key, to, token)
            .await?;

        // Get existing token
        let current_token = self
            .query(&swap_any::ADDRESS, &swap_any::query_token())
            .await?;

        let Ok([current_token]): Result<[Word; 1], _> = current_token.try_into() else {
            bail!("Bad token state")
        };

        let transient_data = swap_any::Swap::pub_vars::mutations()
            .nft(|tup| {
                tup.contract(nft::Transfer::ADDRESS.contract.into())
                    .addr(nft::Transfer::ADDRESS.predicate.into())
            })
            .into();

        let decision_variables = swap_any::Swap::Vars { ___I_pathway: 3 }.into();

        let state_mutations = swap_any::storage::mutations().token(token).into();

        let swap_any_swap = SolutionData {
            predicate_to_solve: swap_any::Swap::ADDRESS,
            decision_variables,
            transient_data,
            state_mutations,
        };

        let transient_data = nft::Transfer::pub_vars::mutations()
            .key(to)
            .to(key)
            .token(current_token)
            .into();

        let decision_variables = nft::Transfer::Vars {
            auth_addr: (
                swap_any::Swap::ADDRESS.contract.into(),
                swap_any::Swap::ADDRESS.predicate.into(),
            ),
            // Pathway to the auth predicate
            ___A_pathway: 2,
        }
        .into();

        let state_mutations = nft::storage::mutations()
            .owners(|map| map.entry(current_token, key))
            .nonce(|map| map.entry(to, nonce))
            .into();

        // Transfer existing token from swap_any to user
        let transfer_nft = SolutionData {
            predicate_to_solve: nft::Transfer::ADDRESS,
            decision_variables,
            transient_data,
            state_mutations,
        };

        solution.data.push(swap_any_swap);
        solution.data.push(transfer_nft);

        Ok(solution)
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
    pint_directory: &Path,
) -> anyhow::Result<()> {
    let client = EssentialClient::new(addr)?;
    let (nft_contract, signed_contract, swap_any_predicates) =
        compile_contracts(pint_directory).await?;

    let predicates = wallet.sign_contract(nft_contract, account_name)?;
    client.deploy_contract(predicates).await?;
    let predicates = wallet.sign_contract(signed_contract, account_name)?;
    client.deploy_contract(predicates).await?;
    let predicates = wallet.sign_contract(swap_any_predicates, account_name)?;
    client.deploy_contract(predicates).await?;

    Ok(())
}

/// Compiles the signed, nft and swap_any contracts and returns them in a tuple of that order.
///
/// Returns an `Err` in the case that a newly compiled contract address differs
/// from the ABI-provided address.
pub async fn compile_contracts(
    pint_directory: &Path,
) -> anyhow::Result<(Contract, Contract, Contract)> {
    let signed_contract = compile_pint_project(pint_directory.join("signed")).await?;
    let nft_contract = compile_pint_project(pint_directory.join("nft")).await?;
    let swap_any_predicates = compile_pint_project(pint_directory.join("swap_any")).await?;

    // Check that the newly compiled addresses match the ABI-generated addresses.
    ensure!(signed::ADDRESS == essential_hash::contract_addr::from_contract(&signed_contract));
    ensure!(nft::ADDRESS == essential_hash::contract_addr::from_contract(&nft_contract));
    ensure!(
        swap_any::ADDRESS == essential_hash::contract_addr::from_contract(&swap_any_predicates)
    );

    Ok((signed_contract, nft_contract, swap_any_predicates))
}

pub fn print_addresses() {
    print_contract_address("nft", &nft::ADDRESS);
    print_predicate_address("nft_cancel", &nft::Cancel::ADDRESS);
    print_predicate_address("nft_mint", &nft::Mint::ADDRESS);
    print_predicate_address("nft_transfer", &nft::Transfer::ADDRESS);
    print_contract_address("signed", &signed::ADDRESS);
    print_predicate_address("signed_cancel", &signed::Cancel::ADDRESS);
    print_predicate_address("signed_transfer", &signed::Transfer::ADDRESS);
    print_contract_address("swap_any", &swap_any::ADDRESS);
    print_predicate_address("swap_any_init", &swap_any::Init::ADDRESS);
    print_predicate_address("swap_any_swap", &swap_any::Swap::ADDRESS);
}

pub fn find_address(predicate: &str, num: usize) -> Option<&str> {
    predicate
        .split("0x")
        .nth(num)
        .and_then(|s| s.split(&[' ', ')', ',']).next())
        .map(|s| s.trim())
}
