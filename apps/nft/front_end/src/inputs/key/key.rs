use crate::inputs::{Int, WriteDecVars};

pub type TransientData = crate::inputs::auth::TransientData;

pub struct DecVars {
    pub new_nonce: Int,
    pub sig: essential_signer::secp256k1::ecdsa::RecoverableSignature,
    pub public_key: essential_signer::secp256k1::PublicKey,
}

impl DecVars {
    pub fn encode(&self) -> Vec<essential_types::Value> {
        let Self {
            new_nonce,
            sig,
            public_key,
        } = self;
        let mut decision_variables = vec![];

        new_nonce.write_dec_var(&mut decision_variables);
        sig.write_dec_var(&mut decision_variables);
        public_key.write_dec_var(&mut decision_variables);

        decision_variables
    }
}
