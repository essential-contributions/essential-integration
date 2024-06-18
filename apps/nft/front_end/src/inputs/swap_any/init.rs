use app_utils::inputs::{WriteDecVars, B256};

pub struct DecVars {
    pub set: B256,
}

impl DecVars {
    pub fn encode(&self) -> Vec<essential_types::Value> {
        let Self { set } = self;
        let mut decision_variables = vec![];

        set.write_dec_var(&mut decision_variables);

        decision_variables
    }
}
