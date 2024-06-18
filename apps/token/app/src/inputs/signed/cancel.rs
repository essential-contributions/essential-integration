use app_utils::inputs::{WriteDecVars, B256};

pub type TransientData = crate::inputs::token::Interface;

pub struct DecVars {
    pub key: B256,
    pub sig: essential_signer::secp256k1::ecdsa::RecoverableSignature,
    pub public_key: essential_signer::secp256k1::PublicKey,
}

impl DecVars {
    pub fn encode(&self) -> Vec<essential_types::Value> {
        let Self {
            key,
            sig,
            public_key,
        } = self;
        let mut decision_variables = vec![];

        key.write_dec_var(&mut decision_variables);
        sig.write_dec_var(&mut decision_variables);
        public_key.write_dec_var(&mut decision_variables);

        decision_variables
    }
}
