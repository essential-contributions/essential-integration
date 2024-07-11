use anyhow::bail;
use essential_app_utils::{
    addresses::{contract_hash, get_addresses},
    compile::compile_pint_project,
    inputs::Encode,
    print::{print_contract_address, print_predicate_address},
};
use essential_rest_client::EssentialClient;
use essential_types::{
    convert::word_4_from_u8_32,
    solution::{Solution, SolutionData},
    ContentAddress, PredicateAddress, Word,
};
use std::{path::PathBuf, vec};

/// Items generated from `nft-abi.json`.
mod nft {
    pint_abi::gen_from_file!("../pint/nft/out/debug/nft-abi.json");

    // TODO: Remove the following after `pint-abi-gen` adds `keys()` builder.
    use essential_app_utils::inputs::{Int, B256};
    use essential_types::solution::Mutation;

    pub fn query_owners(token: Int) -> essential_types::Key {
        let mut m: Vec<Mutation> = storage::mutations()
            .owners(|map| map.entry(token.0, Default::default()))
            .into();
        m.pop().unwrap().key
    }
    pub fn query_nonce(key: B256) -> essential_types::Key {
        let mut m: Vec<Mutation> = storage::mutations()
            .nonce(|map| map.entry(key.0, Default::default()))
            .into();
        m.pop().unwrap().key
    }
}

/// Items generated from `signed-abi.json`.
mod signed {
    pint_abi::gen_from_file!("../pint/signed/out/debug/signed-abi.json");
}

/// Items generated from `swap-any-abi.json`.
mod swap_any {
    use essential_types::solution::Mutation;

    pint_abi::gen_from_file!("../pint/swap_any/out/debug/swap_any-abi.json");

    // TODO: Remove the following after `pint-abi-gen` adds `keys()` builder.
    pub(crate) fn query_token() -> essential_types::Key {
        let mut m: Vec<Mutation> = storage::mutations().token(Default::default()).into();
        m.pop().unwrap().key
    }
}

pub struct Nft {
    client: EssentialClient,
    wallet: essential_wallet::Wallet,
    deployed_predicates: Addresses,
}

#[derive(Debug, Clone)]
pub struct Addresses {
    pub nft: ContentAddress,
    pub nft_cancel: PredicateAddress,
    pub nft_mint: PredicateAddress,
    pub nft_transfer: PredicateAddress,
    pub signed: ContentAddress,
    pub signed_cancel: PredicateAddress,
    pub signed_transfer: PredicateAddress,
    pub swap_any: ContentAddress,
    pub swap_any_init: PredicateAddress,
    pub swap_any_swap: PredicateAddress,
}

impl Nft {
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
                predicate_to_solve: self.deployed_predicates.nft_mint.clone(),
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
            .query(
                &self.deployed_predicates.nft,
                &nft::query_owners(token.into()),
            )
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
                predicate_to_solve: self.deployed_predicates.swap_any_init.clone(),
                decision_variables: swap_any::Init::Vars {
                    contract: self.deployed_predicates.nft.clone().into(),
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
            .query(&self.deployed_predicates.swap_any, &swap_any::query_token())
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
            .query(&self.deployed_predicates.nft, &nft::query_nonce(key.into()))
            .await?;
        let mut nonce = nonce.first().copied().unwrap_or_default();
        nonce += 1;

        // Sign key, to, token
        let mut to_hash = key.to_vec();
        to_hash.extend_from_slice(&to);
        to_hash.push(token);
        to_hash.push(nonce);
        to_hash.extend(word_4_from_u8_32(self.deployed_predicates.nft.0));
        to_hash.extend(word_4_from_u8_32(
            self.deployed_predicates.nft_transfer.predicate.0,
        ));

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

        let nft_transfer = self.deployed_predicates.nft_transfer.clone();
        let transient_data = signed::Transfer::pub_vars::mutations()
            .nft(|tup| {
                tup.contract(nft_transfer.contract.into())
                    .addr(nft_transfer.predicate.into())
            })
            .into();

        let signed_transfer = SolutionData {
            predicate_to_solve: self.deployed_predicates.signed_transfer.clone(),
            decision_variables: decision_variables.into(),
            transient_data,
            state_mutations: vec![],
        };

        let transient_data = nft::Transfer::pub_vars::mutations()
            .key(key)
            .to(to)
            .token(token)
            .into();

        let signed_tx_addr = self.deployed_predicates.signed_transfer.clone();
        let decision_variables = nft::Transfer::Vars {
            auth_addr: (
                signed_tx_addr.contract.into(),
                signed_tx_addr.predicate.into(),
            ),
            ___A_pathway: 0,
        };

        let state_mutations = nft::storage::mutations()
            .owners(|map| map.entry(token, to))
            .nonce(|map| map.entry(key, nonce))
            .into();

        let transfer_nft = SolutionData {
            predicate_to_solve: self.deployed_predicates.nft_transfer.clone(),
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
        let to = contract_hash(&self.deployed_predicates.swap_any_swap);
        let nonce = self
            .query(&self.deployed_predicates.nft, &nft::query_nonce(to.into()))
            .await?;
        let mut nonce = nonce.first().copied().unwrap_or_default();
        nonce += 1;

        let mut solution = self
            .make_transfer_solution(account_name, key, to, token)
            .await?;

        // Get existing token
        let current_token = self
            .query(&self.deployed_predicates.swap_any, &swap_any::query_token())
            .await?;

        let Ok([current_token]): Result<[Word; 1], _> = current_token.try_into() else {
            bail!("Bad token state")
        };

        let nft_tx_addr = self.deployed_predicates.nft_transfer.clone();
        let transient_data = swap_any::Swap::pub_vars::mutations()
            .nft(|tup| {
                tup.contract(nft_tx_addr.contract.into())
                    .addr(nft_tx_addr.predicate.into())
            })
            .into();

        let decision_variables = swap_any::Swap::Vars { ___I_pathway: 3 }.into();

        let state_mutations = swap_any::storage::mutations().token(token).into();

        let swap_any_swap = SolutionData {
            predicate_to_solve: self.deployed_predicates.swap_any_swap.clone(),
            decision_variables,
            transient_data,
            state_mutations,
        };

        let transient_data = nft::Transfer::pub_vars::mutations()
            .key(to)
            .to(key)
            .token(current_token)
            .into();

        let swap_any_swap_addr = self.deployed_predicates.swap_any_swap.clone();
        let decision_variables = nft::Transfer::Vars {
            auth_addr: (
                swap_any_swap_addr.contract.into(),
                swap_any_swap_addr.predicate.into(),
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
            predicate_to_solve: self.deployed_predicates.nft_transfer.clone(),
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
    pint_directory: PathBuf,
) -> anyhow::Result<Addresses> {
    let client = EssentialClient::new(addr)?;
    let signed_contract = compile_pint_project(pint_directory.clone().join("signed")).await?;
    let signed_addresses = get_addresses(&signed_contract);

    let nft_contract = compile_pint_project(pint_directory.clone().join("nft")).await?;
    let nft_addresses = get_addresses(&nft_contract);

    let swap_any_predicates = compile_pint_project(pint_directory.clone().join("swap_any")).await?;
    let swap_any_addresses = get_addresses(&swap_any_predicates);

    let addresses = Addresses {
        nft: nft_addresses.0.clone(),
        nft_cancel: nft_addresses.1[0].clone(),
        nft_mint: nft_addresses.1[1].clone(),
        nft_transfer: nft_addresses.1[2].clone(),
        signed: signed_addresses.0.clone(),
        signed_cancel: signed_addresses.1[0].clone(),
        signed_transfer: signed_addresses.1[1].clone(),
        swap_any: swap_any_addresses.0.clone(),
        swap_any_init: swap_any_addresses.1[0].clone(),
        swap_any_swap: swap_any_addresses.1[1].clone(),
    };

    let predicates = wallet.sign_contract(nft_contract, account_name)?;
    client.deploy_contract(predicates).await?;
    let predicates = wallet.sign_contract(signed_contract, account_name)?;
    client.deploy_contract(predicates).await?;
    let predicates = wallet.sign_contract(swap_any_predicates, account_name)?;
    client.deploy_contract(predicates).await?;

    Ok(addresses)
}

pub async fn compile_addresses(pint_directory: PathBuf) -> anyhow::Result<Addresses> {
    let signed_contract = compile_pint_project(pint_directory.clone().join("signed")).await?;
    let signed_addresses = get_addresses(&signed_contract);

    let nft_contract = compile_pint_project(pint_directory.clone().join("nft")).await?;
    let nft_addresses = get_addresses(&nft_contract);

    let swap_any_predicates = compile_pint_project(pint_directory.clone().join("swap_any")).await?;
    let swap_any_addresses = get_addresses(&swap_any_predicates);

    let addresses = Addresses {
        nft: nft_addresses.0.clone(),
        nft_cancel: nft_addresses.1[0].clone(),
        nft_mint: nft_addresses.1[1].clone(),
        nft_transfer: nft_addresses.1[2].clone(),
        signed: signed_addresses.0.clone(),
        signed_cancel: signed_addresses.1[0].clone(),
        signed_transfer: signed_addresses.1[1].clone(),
        swap_any: swap_any_addresses.0.clone(),
        swap_any_init: swap_any_addresses.1[0].clone(),
        swap_any_swap: swap_any_addresses.1[1].clone(),
    };

    Ok(addresses)
}

pub fn print_addresses(addresses: &Addresses) {
    let Addresses {
        nft,
        nft_cancel,
        nft_mint,
        nft_transfer,
        signed,
        signed_cancel,
        signed_transfer,
        swap_any,
        swap_any_init,
        swap_any_swap,
    } = addresses;
    print_contract_address("nft", nft);
    print_predicate_address("nft_cancel", nft_cancel);
    print_predicate_address("nft_mint", nft_mint);
    print_predicate_address("nft_transfer", nft_transfer);
    print_contract_address("signed", signed);
    print_predicate_address("signed_cancel", signed_cancel);
    print_predicate_address("signed_transfer", signed_transfer);
    print_contract_address("swap_any", swap_any);
    print_predicate_address("swap_any_init", swap_any_init);
    print_predicate_address("swap_any_swap", swap_any_swap);
}

pub fn find_address(predicate: &str, num: usize) -> Option<&str> {
    predicate
        .split("0x")
        .nth(num)
        .and_then(|s| s.split(&[' ', ')', ',']).next())
        .map(|s| s.trim())
}
