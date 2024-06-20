use anyhow::bail;
use app_utils::inputs::{index_mutation, Int, WriteDecVars, B256};
use essential_signer::secp256k1::{ecdsa::RecoverableSignature, PublicKey};
use essential_types::{
    convert::word_4_from_u8_32,
    solution::{Solution, SolutionData},
    ContentAddress, PredicateAddress, Word,
};
use essential_wallet::Wallet;

use crate::Swap;

#[derive(Clone, Debug)]
pub struct SwapArgs {
    // All transient
    pub key: B256,
    pub account_b: B256,
    pub token_a: B256,
    pub token_b: B256,
    pub amount_a_max: Int,
    pub amount_b_min: Int,
}

#[derive(Clone, Debug)]
pub struct SignedSwapArgs {
    pub account_name: String,
    pub swap: SwapArgs,
    pub nonce: Int,
    pub token_addr: PredicateAddress,
    pub signed_swap_addr: PredicateAddress,
}

#[derive(Clone, Debug)]
pub struct SignedSwap {
    pub swap_sig: RecoverableSignature,
    pub transfer_sig: RecoverableSignature,
    pub public_key: PublicKey,
}

#[derive(Clone, Debug)]
pub struct Trade {
    pub swap_a: Swap,
    pub swap_b: Swap,
    pub swap_addr: PredicateAddress,
    pub signed_swap_addr: PredicateAddress,
    pub auth_intent: PredicateAddress,
    pub transfer_a: Transfer,
    pub transfer_b: Transfer,
}

#[derive(Clone, Debug)]
pub struct Transfer {
    pub from: B256,
    pub to: B256,
    pub amount: Int,
    pub new_from_balance: Int,
    pub new_to_balance: Int,
    pub nonce: Int,
    pub token_addr: PredicateAddress,
}

impl Trade {
    pub fn build(&self) -> anyhow::Result<Solution> {
        let token_a_path = 0;
        let signed_swap_a_path = 1;
        let token_b_path = 2;
        let signed_swap_b_path = 3;
        let token_a_auth_path = 4;
        let token_b_auth_path = 5;
        let swap_a_path = 6;
        let swap_b_path = 7;

        let token_a_transfer_auth = build_transfer_auth(
            token_a_path,
            signed_swap_a_path,
            self.auth_intent.clone(),
            self.signed_swap_addr.clone(),
            self.swap_a.signed.clone(),
            self.transfer_a.clone(),
        );

        let token_b_transfer_auth = build_transfer_auth(
            token_b_path,
            signed_swap_b_path,
            self.auth_intent.clone(),
            self.signed_swap_addr.clone(),
            self.swap_b.signed.clone(),
            self.transfer_b.clone(),
        );

        let token_a_transfer = build_transfer(
            token_a_auth_path,
            self.auth_intent.clone(),
            self.swap_a.swap.clone(),
            self.transfer_a.clone(),
        );

        let token_b_transfer = build_transfer(
            token_b_auth_path,
            self.auth_intent.clone(),
            self.swap_b.swap.clone(),
            self.transfer_b.clone(),
        );

        let signed_swap_a = build_signed_swap(
            swap_a_path,
            self.swap_addr.clone(),
            self.signed_swap_addr.clone(),
            self.swap_a.clone(),
        );

        let signed_swap_b = build_signed_swap(
            swap_b_path,
            self.swap_addr.clone(),
            self.signed_swap_addr.clone(),
            self.swap_b.clone(),
        );

        let swap_a = build_swap(self.swap_a.swap.clone(), self.swap_addr.clone());

        let swap_b = build_swap(self.swap_b.swap.clone(), self.swap_addr.clone());

        let mut solution = blank_solution();

        solution.data[token_a_auth_path as usize] = token_a_transfer_auth;
        solution.data[token_b_auth_path as usize] = token_b_transfer_auth;

        solution.data[token_a_path as usize] = token_a_transfer;
        solution.data[token_b_path as usize] = token_b_transfer;

        solution.data[signed_swap_a_path as usize] = signed_swap_a;
        solution.data[signed_swap_b_path as usize] = signed_swap_b;

        solution.data[swap_a_path as usize] = swap_a;
        solution.data[swap_b_path as usize] = swap_b;

        Ok(solution)
    }
}

fn build_transfer(
    auth_path: Word,
    auth_intent: PredicateAddress,
    swap_args: SwapArgs,
    transfer: Transfer,
) -> SolutionData {
    let mut decision_variables = vec![];
    auth_intent.write_dec_var(&mut decision_variables);
    Int(auth_path).write_dec_var(&mut decision_variables);

    let transient_data = vec![
        index_mutation(0, transfer.from.to_value()),
        index_mutation(1, transfer.to.to_value()),
        index_mutation(2, transfer.amount.to_value()),
    ];

    let state_mutations = vec![
        token::inputs::token::balances(transfer.from, transfer.new_from_balance),
        token::inputs::token::balances(transfer.to, transfer.new_to_balance),
        token::inputs::token::nonce(swap_args.key, transfer.nonce),
    ];

    SolutionData {
        predicate_to_solve: transfer.token_addr.clone(),
        decision_variables,
        transient_data,
        state_mutations,
    }
}

fn build_swap(swap_args: SwapArgs, swap_addr: PredicateAddress) -> SolutionData {
    let transient_data = vec![
        index_mutation(0, swap_args.key.to_value()),
        index_mutation(1, swap_args.account_b.to_value()),
        index_mutation(2, swap_args.token_a.to_value()),
        index_mutation(3, swap_args.token_b.to_value()),
        index_mutation(4, swap_args.amount_a_max.to_value()),
        index_mutation(5, swap_args.amount_b_min.to_value()),
    ];
    SolutionData {
        predicate_to_solve: swap_addr.clone(),
        decision_variables: Default::default(),
        transient_data,
        state_mutations: Default::default(),
    }
}

fn build_signed_swap(
    swap_path: Word,
    swap_addr: PredicateAddress,
    signed_swap_addr: PredicateAddress,
    swap: Swap,
) -> SolutionData {
    let mut decision_variables = vec![];
    swap.signed.swap_sig.write_dec_var(&mut decision_variables);
    swap.signed
        .public_key
        .write_dec_var(&mut decision_variables);
    swap_addr.write_dec_var(&mut decision_variables);
    Int(swap_path).write_dec_var(&mut decision_variables);

    let transient_data = vec![index_mutation(0, swap.swap.key.to_value())];

    SolutionData {
        predicate_to_solve: signed_swap_addr.clone(),
        decision_variables,
        transient_data,
        state_mutations: Default::default(),
    }
}

fn build_transfer_auth(
    token_path: Word,
    signed_swap_path: Word,
    auth_intent: PredicateAddress,
    signed_swap_addr: PredicateAddress,
    signed: SignedSwap,
    transfer: Transfer,
) -> SolutionData {
    let mut decision_variables = vec![];
    Int(1).write_dec_var(&mut decision_variables);
    Int(token_path).write_dec_var(&mut decision_variables);
    signed.transfer_sig.write_dec_var(&mut decision_variables);
    signed.public_key.write_dec_var(&mut decision_variables);
    signed_swap_addr.write_dec_var(&mut decision_variables);
    Int(signed_swap_path).write_dec_var(&mut decision_variables);

    let transient_data = vec![
        index_mutation(0, B256::from(transfer.token_addr.contract.0).to_value()),
        index_mutation(1, B256::from(transfer.token_addr.predicate.0).to_value()),
    ];
    SolutionData {
        predicate_to_solve: auth_intent.clone(),
        decision_variables,
        transient_data,
        state_mutations: Default::default(),
    }
}

impl SignedSwapArgs {
    pub fn build(&self, wallet: &mut Wallet) -> anyhow::Result<SignedSwap> {
        let mut data = vec![];
        data.extend(self.swap.key.0);
        data.extend(self.swap.account_b.0);
        data.extend(self.swap.token_a.0);
        data.extend(self.swap.token_b.0);
        data.push(self.swap.amount_a_max.0);
        data.push(self.swap.amount_b_min.0);
        data.push(self.nonce.0);

        let sig = wallet.sign_words(&data, &self.account_name)?;
        let swap_sig = match sig {
            essential_signer::Signature::Secp256k1(sig) => sig,
            _ => bail!("Invalid signature"),
        };

        let mut data = vec![];
        data.extend(self.swap.key.0);
        data.push(self.nonce.0);
        data.extend(word_4_from_u8_32(self.token_addr.contract.0));
        data.extend(word_4_from_u8_32(self.token_addr.predicate.0));
        data.extend(word_4_from_u8_32(self.signed_swap_addr.contract.0));
        data.extend(word_4_from_u8_32(self.signed_swap_addr.predicate.0));

        let sig = wallet.sign_words(&data, &self.account_name)?;
        let transfer_sig = match sig {
            essential_signer::Signature::Secp256k1(sig) => sig,
            _ => bail!("Invalid signature"),
        };

        let public_key = wallet.get_public_key(&self.account_name)?;
        let essential_signer::PublicKey::Secp256k1(public_key) = public_key else {
            bail!("Invalid public key")
        };

        Ok(SignedSwap {
            swap_sig,
            transfer_sig,
            public_key,
        })
    }
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
        data: vec![data; 8],
    }
}
