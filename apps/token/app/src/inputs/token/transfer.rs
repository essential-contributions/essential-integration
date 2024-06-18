use app_utils::inputs::{Instance, WriteDecVars};

pub struct DecVars {
    pub auth_addr: Instance,
}

impl DecVars {
    pub fn encode(&self) -> Vec<essential_types::Value> {
        let Self { auth_addr } = self;
        let mut decision_variables = vec![];

        auth_addr.write_dec_var(&mut decision_variables);

        decision_variables
    }
}
