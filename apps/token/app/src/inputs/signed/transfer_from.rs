use app_utils::inputs::{Instance, WriteDecVars};

pub type TransientData = crate::inputs::token::Interface;

pub struct DecVars {
    pub sig: essential_signer::secp256k1::ecdsa::RecoverableSignature,
    pub public_key: essential_signer::secp256k1::PublicKey,
    pub constraints: Instance,
}

impl DecVars {
    pub fn encode(&self) -> Vec<essential_types::Value> {
        let Self {
            sig,
            public_key,
            constraints,
        } = self;
        let mut decision_variables = vec![];

        sig.write_dec_var(&mut decision_variables);
        public_key.write_dec_var(&mut decision_variables);
        constraints.write_dec_var(&mut decision_variables);

        decision_variables
    }
}
