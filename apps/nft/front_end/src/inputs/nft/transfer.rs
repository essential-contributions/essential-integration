use app_utils::inputs::{Int, WriteDecVars};

pub struct DecVars {
    pub auth_auth_pathway: Int,
}

impl DecVars {
    pub fn encode(&self) -> Vec<essential_types::Value> {
        let Self { auth_auth_pathway } = self;
        let mut decision_variables = vec![];

        auth_auth_pathway.write_dec_var(&mut decision_variables);

        decision_variables
    }
}
