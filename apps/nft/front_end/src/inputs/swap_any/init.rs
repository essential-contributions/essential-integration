use app_utils::inputs::{WriteDecVars, B256};

pub struct DecVars {
    pub contract: B256,
}

impl DecVars {
    pub fn encode(&self) -> Vec<essential_types::Value> {
        let Self { contract } = self;
        let mut decision_variables = vec![];

        contract.write_dec_var(&mut decision_variables);

        decision_variables
    }
}
