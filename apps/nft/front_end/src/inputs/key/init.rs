use crate::inputs::{WriteDecVars, B256};

pub struct DecVars {
    pub sig: essential_signer::secp256k1::ecdsa::RecoverableSignature,
    pub key: B256,
    pub public_key: essential_signer::secp256k1::PublicKey,
}

impl DecVars {
    pub fn encode(&self) -> Vec<essential_types::Value> {
        let Self {
            sig,
            key,
            public_key,
        } = self;
        let mut decision_variables = vec![];

        sig.write_dec_var(&mut decision_variables);
        key.write_dec_var(&mut decision_variables);
        public_key.write_dec_var(&mut decision_variables);

        decision_variables
    }
}
