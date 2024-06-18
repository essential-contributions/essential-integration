use anyhow::bail;
use app_utils::{
    addresses::{contract_hash, get_addresses},
    compile::compile_pint_file,
    inputs::Instance,
    print::{print_intent_address, print_set_address},
    read::read_pint_file,
};
use essential_rest_client::EssentialClient;
use essential_types::{
    convert::word_4_from_u8_32,
    solution::{Solution, SolutionData},
    ContentAddress, IntentAddress, Word,
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
    pub signed: ContentAddress,
    pub signed_cancel: IntentAddress,
    pub signed_transfer: IntentAddress,
    pub signed_transfer_from_to: IntentAddress,
    pub signed_transfer_from: IntentAddress,
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

    async fn mint_inner(&mut self, key: [Word; 4], token: Word) -> anyhow::Result<()> {
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

    pub async fn mint(&mut self, account_name: &str, token: Word) -> anyhow::Result<()> {
        let key = self.get_hashed_key(account_name)?;
        self.mint_inner(key, token).await
    }

    pub async fn mint_for_contract(
        &mut self,
        contract: &IntentAddress,
        token: Word,
    ) -> anyhow::Result<()> {
        let key = contract_hash(contract);
        self.mint_inner(key, token).await
    }

    async fn do_i_own_inner(&mut self, key: [Word; 4], token: Word) -> anyhow::Result<bool> {
        let state = self
            .query(&self.deployed_intents.nft, &query_owners(token.into()))
            .await?;
        Ok(state[..] == key[..])
    }

    pub async fn do_i_own(&mut self, account_name: &str, token: Word) -> anyhow::Result<bool> {
        let key = self.get_hashed_key(account_name)?;
        self.do_i_own_inner(key, token).await
    }

    pub async fn do_i_own_contract(
        &mut self,
        contract: &IntentAddress,
        token: Word,
    ) -> anyhow::Result<bool> {
        let key = contract_hash(contract);
        self.do_i_own_inner(key, token).await
    }

    pub async fn init_swap_any(&mut self, token: Word) -> anyhow::Result<()> {
        let solution = Solution {
            data: vec![SolutionData {
                intent_to_solve: self.deployed_intents.swap_any_init.clone(),
                decision_variables: inputs::swap_any::init::DecVars {
                    set: self.deployed_intents.nft.clone().into(),
                }
                .encode(),
                transient_data: Default::default(),
                state_mutations: vec![inputs::swap_any::token(token.into())],
            }],
        };
        self.client.submit_solution(solution).await?;

        Ok(())
    }

    pub async fn swap_any_owns(&mut self) -> anyhow::Result<Option<Word>> {
        let state = self
            .query(
                &self.deployed_intents.swap_any,
                &inputs::swap_any::query_token(),
            )
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
        let key = self.get_hashed_key(account_name)?;
        let to = self.get_hashed_key(to)?;

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
        token: Word,
    ) -> anyhow::Result<Solution> {
        let nonce = self
            .query(
                &self.deployed_intents.signed,
                &inputs::signed::query_nonce(key.into()),
            )
            .await?;
        let mut nonce = nonce.first().copied().unwrap_or_default();
        nonce += 1;

        // Sign key, token, to
        let mut to_hash = key.to_vec();
        to_hash.extend_from_slice(&to);
        to_hash.push(token);
        to_hash.push(nonce);
        to_hash.extend(word_4_from_u8_32(self.deployed_intents.nft.0));
        to_hash.extend(word_4_from_u8_32(
            self.deployed_intents.nft_transfer.intent.0,
        ));
        to_hash.push(1);

        let sig = self.wallet.sign_words(&to_hash, account_name)?;
        let sig = match sig {
            essential_signer::Signature::Secp256k1(sig) => sig,
            _ => bail!("Invalid signature"),
        };

        let decision_variables = inputs::signed::transfer::DecVars {
            sig,
            public_key: self.get_pub_key(account_name)?,
        };

        let transient_data = inputs::signed::transfer::TransientData {
            key: key.into(),
            to: to.into(),
            token: token.into(),
            set: self.deployed_intents.nft.clone().into(),
            intent_addr: self.deployed_intents.nft_transfer.intent.clone().into(),
            path: 1.into(),
        };

        let signed_transfer = SolutionData {
            intent_to_solve: self.deployed_intents.signed_transfer.clone(),
            decision_variables: decision_variables.encode(),
            transient_data: transient_data.encode(),
            state_mutations: vec![inputs::signed::nonce(key.into(), nonce.into())],
        };

        let transfer_nft = SolutionData {
            intent_to_solve: self.deployed_intents.nft_transfer.clone(),
            decision_variables: inputs::nft::transfer::DecVars {
                auth_addr: Instance {
                    address: self.deployed_intents.signed_transfer.clone(),
                    path: 0,
                },
            }
            .encode(),
            transient_data: vec![],
            state_mutations: vec![inputs::nft::owners(token.into(), to.into())],
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
        let key = self.get_hashed_key(account_name)?;
        let to = contract_hash(&self.deployed_intents.swap_any_swap);

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

        let Ok([current_token]): Result<[Word; 1], _> = current_token.try_into() else {
            bail!("Bad token state")
        };

        let transient_data = inputs::swap_any::swap::TransientData {
            key: to.into(),
            token: current_token.into(),
            to: key.into(),
            set: self.deployed_intents.nft.clone().into(),
            intent_addr: self.deployed_intents.nft_transfer.intent.clone().into(),
            path: 3.into(),
        };

        let swap_any_swap = SolutionData {
            intent_to_solve: self.deployed_intents.swap_any_swap.clone(),
            decision_variables: Default::default(),
            transient_data: transient_data.encode(),
            state_mutations: vec![inputs::swap_any::token(token.into())],
        };

        // Transfer existing token from swap_any to user
        let transfer_nft = SolutionData {
            intent_to_solve: self.deployed_intents.nft_transfer.clone(),
            // Pathway to the auth intent
            decision_variables: inputs::nft::transfer::DecVars {
                auth_addr: Instance {
                    address: self.deployed_intents.swap_any_swap.clone(),
                    path: 2,
                },
            }
            .encode(),
            transient_data: vec![],
            state_mutations: vec![inputs::nft::owners(current_token.into(), key.into())],
        };

        solution.data.push(swap_any_swap);
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
    let signed_intents = compile_pint_file(pint_directory.clone(), "signed.pnt").await?;
    let signed_addresses = get_addresses(&signed_intents);

    let nft_intents = compile_pint_file(pint_directory.clone(), "nft.pnt").await?;
    let nft_addresses = get_addresses(&nft_intents);

    let swap_any_intents = compile_pint_file(pint_directory.clone(), "swap_any.pnt").await?;
    let swap_any_addresses = get_addresses(&swap_any_intents);

    let addresses = Addresses {
        nft: nft_addresses.0.clone(),
        nft_mint: nft_addresses.1[0].clone(),
        nft_transfer: nft_addresses.1[1].clone(),
        signed: signed_addresses.0.clone(),
        signed_cancel: signed_addresses.1[0].clone(),
        signed_transfer: signed_addresses.1[1].clone(),
        signed_transfer_from: signed_addresses.1[2].clone(),
        signed_transfer_from_to: signed_addresses.1[3].clone(),
        swap_any: swap_any_addresses.0.clone(),
        swap_any_init: swap_any_addresses.1[0].clone(),
        swap_any_swap: swap_any_addresses.1[1].clone(),
    };

    let intents = wallet.sign_intent_set(nft_intents, account_name)?;
    client.deploy_intent_set(intents).await?;
    let intents = wallet.sign_intent_set(signed_intents, account_name)?;
    client.deploy_intent_set(intents).await?;
    let intents = wallet.sign_intent_set(swap_any_intents, account_name)?;
    client.deploy_intent_set(intents).await?;

    Ok(addresses)
}

pub async fn compile_addresses(pint_directory: PathBuf) -> anyhow::Result<Addresses> {
    let signed_intents = compile_pint_file(pint_directory.clone(), "signed.pnt").await?;
    let signed_addresses = get_addresses(&signed_intents);

    let nft_intents = compile_pint_file(pint_directory.clone(), "nft.pnt").await?;
    let nft_addresses = get_addresses(&nft_intents);

    let swap_any_intents = compile_pint_file(pint_directory.clone(), "swap_any.pnt").await?;
    let swap_any_addresses = get_addresses(&swap_any_intents);

    let addresses = Addresses {
        nft: nft_addresses.0.clone(),
        nft_mint: nft_addresses.1[0].clone(),
        nft_transfer: nft_addresses.1[1].clone(),
        signed: signed_addresses.0.clone(),
        signed_cancel: signed_addresses.1[0].clone(),
        signed_transfer: signed_addresses.1[1].clone(),
        signed_transfer_from: signed_addresses.1[2].clone(),
        signed_transfer_from_to: signed_addresses.1[3].clone(),
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
        signed,
        signed_cancel,
        signed_transfer,
        signed_transfer_from,
        signed_transfer_from_to,
        swap_any,
        swap_any_init,
        swap_any_swap,
    } = addresses;
    print_set_address("nft", nft);
    print_intent_address("nft_mint", nft_mint);
    print_intent_address("nft_transfer", nft_transfer);
    print_set_address("signed", signed);
    print_intent_address("signed_cancel", signed_cancel);
    print_intent_address("signed_transfer", signed_transfer);
    print_intent_address("signed_transfer_from", signed_transfer_from);
    print_intent_address("signed_transfer_from_to", signed_transfer_from_to);
    print_set_address("swap_any", swap_any);
    print_intent_address("swap_any_init", swap_any_init);
    print_intent_address("swap_any_swap", swap_any_swap);
}

pub async fn update_addresses(pint_directory: PathBuf) -> anyhow::Result<()> {
    let addresses = compile_addresses(pint_directory.clone()).await?;

    replace_intent_address(
        pint_directory.join("allowed"),
        "signed.pnt",
        1,
        &addresses.signed_transfer,
    )
    .await?;
    replace_intent_address(
        pint_directory.join("allowed"),
        "signed.pnt",
        2,
        &addresses.signed_transfer_from,
    )
    .await?;
    replace_intent_address(
        pint_directory.join("allowed"),
        "signed.pnt",
        3,
        &addresses.signed_transfer_from_to,
    )
    .await?;

    Ok(())
}

pub async fn replace_intent_address(
    pint_directory: PathBuf,
    name: &str,
    num: usize,
    address: &IntentAddress,
) -> anyhow::Result<()> {
    let mut intent = read_pint_file(pint_directory.clone(), name).await?;
    let set = find_address(&intent, (num - 1) * 2)
        .ok_or_else(|| anyhow::anyhow!("{} missing set address", name))?;
    intent = intent.replace(set, &hex::encode_upper(address.set.0));
    let intent_addr = find_address(&intent, (num - 1) * 2 + 1)
        .ok_or_else(|| anyhow::anyhow!("{} missing intent address", name))?;
    intent = intent.replace(intent_addr, &hex::encode_upper(address.intent.0));
    tokio::fs::write(pint_directory.join(name), intent).await?;
    Ok(())
}

pub async fn replace_set_address(
    pint_directory: PathBuf,
    name: &str,
    address: &ContentAddress,
) -> anyhow::Result<()> {
    let mut intent = read_pint_file(pint_directory.clone(), name).await?;
    let set =
        find_address(&intent, 1).ok_or_else(|| anyhow::anyhow!("{} missing set address", name))?;
    intent = intent.replace(set, &hex::encode_upper(address.0));
    tokio::fs::write(pint_directory.join(name), intent).await?;
    Ok(())
}

pub fn find_address(intent: &str, num: usize) -> Option<&str> {
    intent
        .split("0x")
        .nth(num)
        .and_then(|s| s.split(&[' ', ')', ',']).next())
        .map(|s| s.trim())
}
