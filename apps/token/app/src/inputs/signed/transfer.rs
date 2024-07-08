use essential_app_utils::inputs::{Int, WriteDecVars};

pub type TransientData = super::TransientData;

pub struct DecVars {
    pub token_path: Int,
    pub sig: essential_signer::secp256k1::ecdsa::RecoverableSignature,
    pub public_key: essential_signer::secp256k1::PublicKey,
}

impl DecVars {
    pub fn encode(&self) -> Vec<essential_types::Value> {
        let Self {
            token_path,
            sig,
            public_key,
        } = self;
        let mut decision_variables = vec![];

        token_path.write_dec_var(&mut decision_variables);
        sig.write_dec_var(&mut decision_variables);
        public_key.write_dec_var(&mut decision_variables);

        decision_variables
    }
}
