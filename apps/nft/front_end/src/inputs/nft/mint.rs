use essential_app_utils::inputs::{Int, WriteDecVars, B256};
use essential_types::Value;

pub struct DecVars {
    pub token: Int,
    pub new_owner: B256,
}

impl DecVars {
    pub fn encode(&self) -> Vec<Value> {
        let Self { token, new_owner } = self;
        let mut decision_variables = vec![];

        token.write_dec_var(&mut decision_variables);
        new_owner.write_dec_var(&mut decision_variables);

        decision_variables
    }
}
